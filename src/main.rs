extern crate minifb;
use minifb::{Key, ScaleMode, Window, WindowOptions, clamp};
use pdf::{PDF, HittablePDF, MixturePDF};
// Look into performance optimization of the RNG
use rand::prelude::*;
use std::f64;
use std::time::Instant;
use rayon::prelude::*;

mod ortho_normal_base;
mod vector3;
mod ray;
mod sphere;
mod moving_sphere;
mod hittable;
mod camera;
mod material;
mod aabb;
mod bvh_node;
mod texture;
mod perlin;
mod pdf;
mod material_service;
mod texture_service;
mod hittable_service;
mod service_locator;
mod scene_service;
mod scene_builder;

use vector3::{Vector3, Color};
use ray::Ray;
use hittable::HitRecord;
use material::ScatterRecord;
use material_service::MaterialService;
use texture_service::TextureService;
use hittable_service::HittableService;
use service_locator::ServiceLocator;

use crate::scene_builder::SceneBuilder;

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
    background: &Color, 
    ray: &Ray, 
    depth: i64) -> Color {

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec:HitRecord = HitRecord::default();
    
    
    if !hittable_service.hit(bvh_root_index, rng, ray, 0.001, f64::MAX, &mut rec) {
        return *background;
    }


    let mut scatter_record= ScatterRecord::default();
    let emitted: Color = material_service.emission(ray, &rec, rec.u, rec.v, &rec.position);
    
    if !material_service.scatter(rng, ray, &rec, &mut scatter_record) {
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
    // let has_lights = false;

    // Maybe put the non-recursive loop after this if statement and move the above in there
    if has_lights {
        let light_pdf: Box<dyn PDF> = Box::new(HittablePDF::new(&rec.position, lights_root_index));
        let other_pdf: Box<dyn PDF> = 
            if scatter_record.pdf.is_some() {  // Get rid of this whole option<Arc> thing
                scatter_record.pdf.expect("Failed to unwrap pdf")
            } else {
                Box::new(HittablePDF::new(&rec.position, lights_root_index))
            };
        let pdfs = vec![light_pdf, other_pdf]; 
        let mixture_pdf: MixturePDF = MixturePDF::new( pdfs ); 
    
        let scattered = Ray::new(rec.position, mixture_pdf.generate(rng, hittable_service), ray.time);
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
                has_lights, 
                background, 
                &scattered, 
                depth - 1
            ) /
            pdf_val;
    } else {
        let pdf: Box<dyn PDF> = scatter_record.pdf.expect("Failed to unwrap pdf");
        let scattered = Ray::new(rec.position, pdf.generate(rng, hittable_service), ray.time);
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
                has_lights, 
                background, 
                &scattered, 
                depth - 1) /
            pdf_val;
    }

}

fn render_pixel(
    rng: &mut ThreadRng, 
    service_locator: &ServiceLocator,
    pixel_index: i64, 
    image_width: i64, 
    image_height: i64, 
    samples_per_pixel: i64, 
    max_depth: i64, 
    scale: f64, 
    use_parallel: bool) 
    -> Vector3 {
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




    let mut color_buffer = Color{x: 0.0, y: 0.0, z: 0.0};
    if use_parallel {
        let seeds: Vec<(f64, f64)> = (0..samples_per_pixel).into_iter().map(|_| (rng.gen::<f64>(), rng.gen::<f64>()) ).collect();
        color_buffer = seeds.into_par_iter().map(|(seed0, seed1)| {
            let mut rng = rand::thread_rng();
            let u = (column_index as f64 + seed0 ) / ((image_width - 1) as f64);
            let v = (row_index as f64 + seed1 ) / ((image_height - 1) as f64);
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
    } else {
        for _sample_index in 0..samples_per_pixel {
            let u = (column_index as f64 + rng.gen::<f64>() ) / ((image_width - 1) as f64);
            let v = (row_index as f64 + rng.gen::<f64>() ) / ((image_height - 1) as f64);
            let ray = camera.get_ray(rng, u, v);
            color_buffer += 
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
                    &ray, 
                    max_depth
                );
        }
    }

    if color_buffer.x != color_buffer.x { color_buffer.x = 0.0; }
    if color_buffer.y != color_buffer.y { color_buffer.y = 0.0; }
    if color_buffer.z != color_buffer.z { color_buffer.z = 0.0; }

    // Try and apply this scaling to the colors before summation
    color_buffer.x = 255.999 * clamp(0.0, (scale * color_buffer.x).sqrt(), 0.999);
    color_buffer.y = 255.999 * clamp(0.0, (scale * color_buffer.y).sqrt(), 0.999);
    color_buffer.z = 255.999 * clamp(0.0, (scale * color_buffer.z).sqrt(), 0.999);

    color_buffer
}

// TODO:
// -- Injest config files
// -- Image descriptor service
// Project restructuring
// Unit testing
// Performance optimization
// ---- Reduce the amount of ARC
// ---- Use texture indices
// -- Reduce recursions to loops
// Replace vector3 with nalgebra or something numpy-like
// Change color to its own type
// ---- Try to convert from dynamic dispatch to static dispatch
// Try to convert to SIMD
// Refactor
// Enforce fused multiply-adds
fn main() {
    // Display Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i64 = 500;
    let output_path = "output.png";

    // Render Settings
    let samples_per_pixel = 100;
    let max_depth = 30;

    // Compute Settings
    let run_parallel = true;
    let run_samples_parallel = true;



    // Scene
    let scene_index = 8;

    let (_aspect_ratio, image_height, service_locator) = SceneBuilder::build_scene(aspect_ratio, image_width, scene_index);
    
    let scale = 1.0 / (samples_per_pixel as f64);

    let now = Instant::now();
    let total_pixels = image_height * image_width;
    let image: Vec<Vector3> = 
    if run_parallel {
        (0..total_pixels).into_par_iter().map(|pixel_index:i64| {
            let mut rng = rand::thread_rng();
            render_pixel(
                &mut rng, 
                &service_locator,
                pixel_index, 
                image_width, 
                image_height, 
                samples_per_pixel, 
                max_depth, 
                scale, 
                run_samples_parallel
            )
        }).collect()
    } else {
        let mut rng = rand::thread_rng();
        (0..total_pixels).into_iter().map(|pixel_index:i64| {
            render_pixel(
                &mut rng, 
                &service_locator,
                pixel_index, 
                image_width, 
                image_height, 
                samples_per_pixel, 
                max_depth, 
                scale, 
                run_samples_parallel
            )
        }).collect()
    };
    println!("{} seconds elapsed", now.elapsed().as_millis() as f64 * 0.001);

    let zero = Vector3{x: 0.0, y: 0.0, z: 0.0};
    let mut final_image: Vec<Vector3> = vec![zero; image.len()];

    for row_index in 0..image_height {
        for column_index in 0..(image_width / 2) {
            let column_index_left = (row_index * image_width + column_index) as usize;
            let column_index_right = (row_index * image_width + (image_width - column_index - 1)) as usize;
            final_image[column_index_left] = image[column_index_right];
            final_image[column_index_right] = image[column_index_left];
        }
    }

    
    let window_buffer: Vec<u32> = final_image
        .iter()
        .map(|v| ((v.x as u32) << 16) | ((v.y as u32) << 8) | v.z as u32)
        .rev()
        .collect();

    let mut window = Window::new(
        "Ray Tracing in Rust - Press ESC to exit",
        image_width as usize,
        image_height as usize,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Center,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");



    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
        .update_with_buffer(
            &window_buffer,
            image_width as usize,
            image_height as usize,
        )
        .unwrap();
    }

    let mut horizontally_flipped_image: Vec<Vector3> = vec![zero; image.len()];
    for row_index in 0..(image_height / 2) {
        for column_index in 0..image_width {
            let row_index_top = (row_index * image_width + column_index) as usize;
            let row_index_bottom = ((image_height - row_index - 1) * image_width + column_index) as usize;
            horizontally_flipped_image[row_index_top] = image[row_index_bottom];
            horizontally_flipped_image[row_index_bottom] = image[row_index_top];
        }
    }

    let ouput_buffer: Vec<u8> = 
        horizontally_flipped_image.iter()
            .flat_map(|vector| [vector.x as u8, vector.y as u8, vector.z as u8])
            .collect();



    let save_result = image::save_buffer_with_format(
        output_path, 
        &ouput_buffer, 
        image_width.try_into().unwrap(), 
        image_height.try_into().unwrap(), 
        image::ColorType::Rgb8, 
        image::ImageFormat::Png
    );

    if save_result.is_ok() {
        println!("Saved output image to {}", output_path);
    } else {
        let error = save_result.unwrap_err();
        panic!("{}", error.to_string());
    }

}
