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
    }, utility::render_config::RenderConfig, scene::{camera::Camera, self}
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
    depth: usize,
    has_lights: bool) -> ColorRGB {

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
                depth - 1,
                has_lights
            );
    }

    if has_lights {
        // Maybe put the non-recursive loop after this if statement and move the above in there
        let light_pdf: PDFEnum = PDFEnum::HittablePDF(HittablePDF::new(&rec.position, lights_root_index));
        let mixture_pdf: MixturePDF = MixturePDF::new( light_pdf, scatter_record.pdf );
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
                depth - 1,
                has_lights) /
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
                depth - 1,
                has_lights) /
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
    max_depth: usize,
    has_lights: bool) -> ColorRGB {

    let mut l: ColorRGB = ColorRGB::black();
    let mut beta: ColorRGB = ColorRGB::white();
    let mut rec:HitRecord = HitRecord::default();
    let mut scatter_record: ScatterRecord = ScatterRecord::default();
    let mut emitted: ColorRGB = ColorRGB::black();
    let mut ray: Ray = first_ray.clone();
    let mut depth: usize = 0;
    loop {
        if max_depth <= depth {
            break;
        }

        // Hit nothing, add background color
        if !hittable_service.hit(bvh_root_index, rng, &ray, 0.001, f32::MAX, &mut rec) {
            l += beta * *background;
            break;
        }

        material_service.emitted(texture_service, &ray, &rec, &mut emitted);

        // We probably hit a lightning material and just have to add the emission
        if !material_service.scatter(rng, texture_service, &ray, &rec, &mut scatter_record) {
            l += beta * emitted;            
            break;
        }

        if scatter_record.is_specular {
            ray = scatter_record.specular_ray;
            beta *= scatter_record.attenuation;
            depth += 1;
            continue;
        }

        if beta.is_nan() || emitted.is_nan() { break }

        if has_lights {
            let light_pdf: PDFEnum = PDFEnum::HittablePDF(HittablePDF::new(&rec.position, lights_root_index));
            let mixture_pdf: MixturePDF = MixturePDF::new( light_pdf, scatter_record.pdf );
            let scattered: Ray = Ray::new_normalized(rec.position, mixture_pdf.generate(rng, hittable_service), ray.time);
            let pdf_val: f32 = mixture_pdf.value(rng, hittable_service, &scattered.direction);

            let l_i: ColorRGB = 
                scatter_record.attenuation 
                * material_service.scattering_pdf(rng, &ray, &rec, &scattered) 
                / pdf_val;

            if l_i.is_nan() { break }

            l += beta * emitted;
            beta *= l_i;

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
    
            l += beta * emitted;
            beta *= new_term;
    
            ray = scattered;
        }

        // Monte Carlo
        if 3 < depth {
            let p: f32 = f32::max(beta.r, f32::max(beta.g, beta.b));
            if p < rng.gen::<f32>() {
                break;
            }

            beta *= 1.0 / p;
        }
    }

    l
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

    let has_lights: bool = hittable_service.has_lights();

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
                config.max_depth,
                has_lights
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
                config.max_depth,
                has_lights
            )
        }
        
    }).sum();

    color_buffer.scale_for_output(config.image_scale);

    color_buffer
}