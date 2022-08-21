use rand::{rngs::ThreadRng, Rng};
use crate::{
    services::{
        service_locator::ServiceLocator, 
        material_service::MaterialService, 
        hittable_service::HittableService, 
        texture_service::TextureService, scene_service::SceneService
    }, 
    core::{
        color_rgb::ColorRGB, 
        ray::Ray
    }, 
    hittables::hit_record::HitRecord, 
    materials::scatter_record::ScatterRecord, 
    pdfs::{
        pdf_enum::PDFEnum, 
        hittable_pdf::HittablePDF, 
        mixture_pdf::MixturePDF, 
        pdf::PDF
    }, utility::render_config::RenderConfig, scene::camera::Camera
};

// These functions aren't needed. The only one that should stay is ray_color_loop_lights. 
// The no_lights version is staying so I don't have to modify any of the scenes from the book.
// The recursive functions are left here for pedagogical reasons to make it easier to translate
// the recursive functions from Peter Shirleys books to a loop based version.
fn ray_color_recursive(
    rng: &mut ThreadRng,
    service_locator: &ServiceLocator,
    material_service: &MaterialService,
    hittable_service: &HittableService,
    texture_service: &TextureService,
    bvh_root_index: usize,
    lights_root_index: usize,
    background: &ColorRGB,
    ray: &Ray,
    depth: usize) -> ColorRGB {

    if depth <= 0 {
        return ColorRGB::new(0.0, 0.0, 0.0);
    }

    let mut rec:HitRecord = HitRecord::default();

    if !hittable_service.hit(bvh_root_index, rng, ray, 0.001, f32::MAX, &mut rec) {
        return *background;
    }


    let mut scatter_record= ScatterRecord::default();
    let mut emitted: ColorRGB = ColorRGB::black();
    material_service.emitted(texture_service, ray, &rec, &mut emitted);

    if !material_service.scatter(rng, texture_service, ray, &rec, &mut scatter_record) {
        return emitted;
    }

    if scatter_record.is_specular {
        return scatter_record.attenuation *
            ray_color_recursive(
                rng,
                service_locator,
                material_service,
                hittable_service,
                texture_service,
                bvh_root_index,
                lights_root_index,
                background,
                &scatter_record.specular_ray,
                depth - 1
            );
    }

    if lights_root_index != 0 {
        // Maybe put the non-recursive loop after this if statement and move the above in there
        let light_pdf: PDFEnum = PDFEnum::HittablePDF(HittablePDF::new(&rec.position, lights_root_index));
        let other_pdf: PDFEnum =
            match scatter_record.pdf {
                PDFEnum::None() => PDFEnum::HittablePDF(HittablePDF::new(&rec.position, lights_root_index)),
                _ => scatter_record.pdf,
            };
        let mixture_pdf: MixturePDF = MixturePDF::new( light_pdf, other_pdf );
        let scattered = Ray::new_normalized(rec.position, mixture_pdf.generate(rng, hittable_service), ray.time);
        let pdf_val = mixture_pdf.value(rng, hittable_service, &scattered.direction);

        return
            emitted +
            scatter_record.attenuation *
            material_service.scattering_pdf(rng, ray, &rec, &scattered) *
            ray_color_recursive(
                rng,
                service_locator,
                material_service,
                hittable_service,
                texture_service,
                bvh_root_index,
                lights_root_index,
                background,
                &scattered,
                depth - 1) /
            pdf_val;
    } else {
        let pdf: PDFEnum = scatter_record.pdf;
        let scattered = Ray::new_normalized(rec.position, pdf.generate(rng, hittable_service), ray.time);
        let pdf_val = pdf.value(rng, hittable_service, &scattered.direction);
    
        return
            emitted +
            scatter_record.attenuation *
            material_service.scattering_pdf(rng, ray, &rec, &scattered) *
            ray_color_recursive(
                rng,
                service_locator,
                material_service,
                hittable_service,
                texture_service,
                bvh_root_index,
                lights_root_index,
                background,
                &scattered,
                depth - 1) /
            pdf_val;
    }

}

fn ray_color_loop(
    rng: &mut ThreadRng,
    material_service: &MaterialService,
    hittable_service: &HittableService,
    texture_service: &TextureService,
    bvh_root_index: usize,
    lights_root_index: usize,
    background: &ColorRGB,
    first_ray: &Ray,
    max_depth: usize) -> ColorRGB {

    let mut output_color: ColorRGB = ColorRGB::black();
    let mut throughput: ColorRGB = ColorRGB::white();
    let mut was_scattered: bool = false;
    let mut rec:HitRecord = HitRecord::default();
    let mut scatter_record: ScatterRecord = ScatterRecord::default();
    let mut emitted: ColorRGB = ColorRGB::black();
    let mut ray: Ray = first_ray.clone();
    for depth in (0..max_depth+1).rev() {
        if depth <= 0 {
            break;
        }

        if !hittable_service.hit(bvh_root_index, rng, &ray, 0.001, f32::MAX, &mut rec) {
            throughput *= *background;
            output_color += throughput;
            break;
        }

        material_service.emitted(texture_service, &ray, &rec, &mut emitted);

        if !material_service.scatter(rng, texture_service, &ray, &rec, &mut scatter_record) {
            throughput *= emitted;
            output_color += throughput;            
            break;
        }

        if scatter_record.is_specular {
            ray = scatter_record.specular_ray;
            throughput *= scatter_record.attenuation;
            continue;
        }

        if throughput.is_nan() || emitted.is_nan() { break }

        if lights_root_index != 0 {
            let light_pdf: PDFEnum = PDFEnum::HittablePDF(HittablePDF::new(&rec.position, lights_root_index));
            let other_pdf: PDFEnum =
                match &scatter_record.pdf {
                    PDFEnum::None() => PDFEnum::HittablePDF(HittablePDF::new(&rec.position, lights_root_index)),
                    _ => scatter_record.pdf,
                };
            let mixture_pdf: MixturePDF = MixturePDF::new( light_pdf, other_pdf );
            let scattered: Ray = Ray::new_normalized(rec.position, mixture_pdf.generate(rng, hittable_service), ray.time);
            let pdf_val: f32 = mixture_pdf.value(rng, hittable_service, &scattered.direction);

            let l_i: ColorRGB = 
                scatter_record.attenuation 
                * material_service.scattering_pdf(rng, &ray, &rec, &scattered) 
                / pdf_val;

            if l_i.is_nan() { break }

            output_color += throughput * emitted;
            throughput *= l_i;
            was_scattered = true;

            ray = scattered;
        } else {
            let pdf: PDFEnum = scatter_record.pdf;
            let scattered = Ray::new_normalized(rec.position, pdf.generate(rng, hittable_service), ray.time);
            let pdf_val = pdf.value(rng, hittable_service, &scattered.direction);
    
            let new_term: ColorRGB = 
                scatter_record.attenuation 
                * material_service.scattering_pdf(rng, &ray, &rec, &scattered) 
                / pdf_val;
    
            if new_term.is_nan() { break }
    
            output_color += throughput * emitted;
            throughput *= new_term;
            was_scattered = true;
    
            ray = scattered;
        }
    }

    if was_scattered && !throughput.is_nan() {
        output_color += throughput;
    }

    output_color
}

pub fn render_pixel(
    config: &RenderConfig,
    rng: &mut ThreadRng,
    service_locator: &ServiceLocator,
    pixel_index: usize)
    -> ColorRGB {
    let column_index: usize = pixel_index % config.image_width;
    let row_index: usize = pixel_index / config.image_width;


    let scene_service: &SceneService = service_locator.get_scene_service();
    let camera: &Camera = scene_service.get_camera();
    let background: &ColorRGB = scene_service.get_background();

    let material_service: &MaterialService = service_locator.get_material_service();
    let texture_service: &TextureService = service_locator.get_texture_service();

    let hittable_service: &HittableService = service_locator.get_hittable_service();
    let bvh_root_index: usize = hittable_service.get_bvh_root_index();
    let lights_root_index: usize = hittable_service.get_lights_root_index();

    let samples: Vec<(f32, f32)> = (0..config.samples_per_pixel).into_iter().map(|_| (rng.gen::<f32>(), rng.gen::<f32>()) ).collect();
    let mut color_buffer: ColorRGB = samples.into_iter().map(|(seed0, seed1)| {
        let mut rng = rand::thread_rng();
        let u = (column_index as f32 + seed0 ) / ((config.image_width - 1) as f32);
        let v = (row_index as f32 + seed1 ) / ((config.image_height - 1) as f32);
        let ray = camera.get_ray(&mut rng, u, v);
        if config.use_loop_rendering {
            ray_color_loop(
                &mut rng,
                material_service,
                hittable_service,
                texture_service,
                bvh_root_index,
                lights_root_index,
                background,
                &ray,
                config.max_depth
            )
        } else {
            ray_color_recursive(
                &mut rng,
                service_locator,
                material_service,
                hittable_service,
                texture_service,
                bvh_root_index,
                lights_root_index,
                background,
                &ray,
                config.max_depth
            )
        }

    }).sum();

    color_buffer.scale_for_output(config.image_scale);

    color_buffer
}