extern crate minifb;
use hittables::hittable::HitRecord;
use materials::material::ScatterRecord;
use math::vector3::{Color, Vector3};
use minifb::{Key, ScaleMode, Window, WindowOptions, clamp};
use pdf::{PDF, HittablePDF, MixturePDF};
use pdfs::pdf;
// Look into performance optimization of the RNG
use rand::prelude::*;
use ray::Ray;
use services::hittable_service::HittableService;
use services::material_service::MaterialService;
use services::service_locator::ServiceLocator;
use services::texture_service::TextureService;
use std::f32;
use std::time::Instant;
use rayon::prelude::*;

use crate::scene::scene_builder::SceneBuilder;
use crate::utility::render_config::RenderConfig;

mod geometry;
mod hittables;
mod materials;
mod math;
mod noise;
mod pdfs;
mod scene;
mod services;
mod textures;
mod utility;
mod ray;

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
    depth: usize) -> Color {

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec:HitRecord = HitRecord::default();
    
    
    if !hittable_service.hit(bvh_root_index, rng, ray, 0.001, f32::MAX, &mut rec) {
        return *background;
    }


    let mut scatter_record= ScatterRecord::default();
    let emitted: Color = material_service.emission(texture_service, ray, &rec, rec.u, rec.v, &rec.position);
    
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
    pixel_index: usize, 
    image_width: usize, 
    image_height: usize, 
    samples_per_pixel: usize, 
    max_depth: usize, 
    scale: f32, 
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
        let seeds: Vec<(f32, f32)> = (0..samples_per_pixel).into_iter().map(|_| (rng.gen::<f32>(), rng.gen::<f32>()) ).collect();
        color_buffer = seeds.into_par_iter().map(|(seed0, seed1)| {
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
    } else {
        for _sample_index in 0..samples_per_pixel {
            let u = (column_index as f32 + rng.gen::<f32>() ) / ((image_width - 1) as f32);
            let v = (row_index as f32 + rng.gen::<f32>() ) / ((image_height - 1) as f32);
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

fn main() {
    let config: RenderConfig = confy::load("render_config/book_3.render_config").expect("Unable to load config file");

    let (_aspect_ratio, image_height, service_locator) = SceneBuilder::build_scene(config.aspect_ratio, config.image_width, config.scene_index);
    
    let scale = 1.0 / (config.samples_per_pixel as f32);

    let now = Instant::now();
    let total_pixels = image_height * config.image_width;
    let image: Vec<Vector3> = 
        (0..total_pixels).into_par_iter().map(|pixel_index:usize| {
            let mut rng = rand::thread_rng();
            render_pixel(
                &mut rng, 
                &service_locator,
                pixel_index, 
                config.image_width, 
                image_height, 
                config.samples_per_pixel, 
                config.max_depth, 
                scale, 
                config.run_sample_parallel
            )
        }).collect();

    println!("{} seconds elapsed", now.elapsed().as_millis() as f32 * 0.001);

    let zero = Vector3{x: 0.0, y: 0.0, z: 0.0};
    let mut final_image: Vec<Vector3> = vec![zero; image.len()];

    for row_index in 0..image_height {
        for column_index in 0..(config.image_width / 2) {
            let column_index_left = (row_index * config.image_width + column_index) as usize;
            let column_index_right = (row_index * config.image_width + (config.image_width - column_index - 1)) as usize;
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
        config.image_width,
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
            config.image_width,
            image_height as usize,
        )
        .unwrap();
    }

    let mut horizontally_flipped_image: Vec<Vector3> = vec![zero; image.len()];
    for row_index in 0..(image_height / 2) {
        for column_index in 0..config.image_width {
            let row_index_top = (row_index * config.image_width + column_index) as usize;
            let row_index_bottom = ((image_height - row_index - 1) * config.image_width + column_index) as usize;
            horizontally_flipped_image[row_index_top] = image[row_index_bottom];
            horizontally_flipped_image[row_index_bottom] = image[row_index_top];
        }
    }

    let ouput_buffer: Vec<u8> = 
        horizontally_flipped_image.iter()
            .flat_map(|vector| [vector.x as u8, vector.y as u8, vector.z as u8])
            .collect();



    let save_result = image::save_buffer_with_format(
        &config.output_path, 
        &ouput_buffer, 
        config.image_width.try_into().unwrap(), 
        image_height.try_into().unwrap(), 
        image::ColorType::Rgb8, 
        image::ImageFormat::Png
    );

    if save_result.is_ok() {
        println!("Saved output image to {}", config.output_path);
    } else {
        let error = save_result.unwrap_err();
        panic!("{}", error.to_string());
    }

}
