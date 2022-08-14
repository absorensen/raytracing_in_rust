use minifb::clamp;
use rand::{rngs::ThreadRng, Rng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{services::{service_locator::ServiceLocator, material_service::MaterialService, hittable_service::HittableService, texture_service::TextureService}, core::{color_rgb::ColorRGB, ray::Ray}, hittables::hit_record::HitRecord, materials::scatter_record::ScatterRecord, pdfs::{pdf_enum::PDFEnum, hittable_pdf::HittablePDF, mixture_pdf::MixturePDF, pdf::PDF}};

// Try splitting this into a mixture and non-mixture pdfs function, as some scenes don't have lights (though they should)
fn ray_color_recursive(
    rng: &mut ThreadRng,
    service_locator: &ServiceLocator,
    material_service: &MaterialService,
    hittable_service: &HittableService,
    texture_service: &TextureService,
    bvh_root_index: usize,
    lights_root_index: usize,
    has_lights: bool,
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
    let emitted: ColorRGB = material_service.emitted(texture_service, ray, &rec, rec.u, rec.v, &rec.position);
    
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
                has_lights,
                background,
                &scatter_record.specular_ray, 
                depth - 1
            );
    }
    
    // Maybe put the non-recursive loop after this if statement and move the above in there
    let (pdf_val, scattered) = if has_lights {
        let light_pdf: PDFEnum = PDFEnum::HittablePDF(HittablePDF::new(&rec.position, lights_root_index));
        let other_pdf: PDFEnum =
            match scatter_record.pdf {
                PDFEnum::None() => PDFEnum::HittablePDF(HittablePDF::new(&rec.position, lights_root_index)),
                _ => scatter_record.pdf,
            };
        let mixture_pdf: MixturePDF = MixturePDF::new( light_pdf, other_pdf ); 
    
        let scattered = Ray::new_normalized(rec.position, mixture_pdf.generate(rng, hittable_service), ray.time);
        (mixture_pdf.value(rng, hittable_service, &scattered.direction), scattered)
    
    } else {
        let pdf: PDFEnum = scatter_record.pdf;
        let scattered = Ray::new_normalized(rec.position, pdf.generate(rng, hittable_service), ray.time);
        (pdf.value(rng, hittable_service, &scattered.direction), scattered) 
    };

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
            has_lights, 
            background, 
            &scattered, 
            depth - 1) /
        pdf_val;

}

pub fn render_pixel(
    rng: &mut ThreadRng, 
    service_locator: &ServiceLocator,
    pixel_index: usize, 
    image_width: usize, 
    image_height: usize, 
    samples_per_pixel: usize, 
    max_depth: usize, 
    scale: f32) 
    -> ColorRGB {
    let column_index = pixel_index % image_width;
    let row_index = pixel_index / image_width;


    let scene_service = service_locator.get_scene_service();
    let camera = scene_service.get_camera();
    let background = scene_service.get_background();

    let material_service: &MaterialService = service_locator.get_material_service();
    let texture_service: &TextureService = service_locator.get_texture_service();

    let hittable_service: &HittableService = service_locator.get_hittable_service();
    let bvh_root_index: usize = hittable_service.get_bvh_root_index();
    let lights_root_index: usize = hittable_service.get_lights_root_index();
    let has_lights: bool = hittable_service.has_lights();




    let seeds: Vec<(f32, f32)> = (0..samples_per_pixel).into_iter().map(|_| (rng.gen::<f32>(), rng.gen::<f32>()) ).collect();
    let mut color_buffer: ColorRGB = seeds.into_par_iter().map(|(seed0, seed1)| {
        let mut rng = rand::thread_rng();
        let u = (column_index as f32 + seed0 ) / ((image_width - 1) as f32);
        let v = (row_index as f32 + seed1 ) / ((image_height - 1) as f32);
        let ray = camera.get_ray(&mut rng, u, v);
        ray_color_recursive(
            &mut rng, 
            service_locator, 
            material_service, 
            hittable_service, 
            texture_service, 
            bvh_root_index, 
            lights_root_index, 
            has_lights, 
            background, 
            &ray, 
            max_depth
        )
    }).sum();

    color_buffer.scale_for_output(scale);

    color_buffer
}