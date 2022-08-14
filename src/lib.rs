extern crate minifb;
use hittables::hit_record::HitRecord;
use materials::scatter_record::ScatterRecord;
use minifb::{Key, ScaleMode, Window, WindowOptions, clamp};
use pdfs::hittable_pdf::HittablePDF;
use pdfs::mixture_pdf::MixturePDF;
use pdfs::pdf::PDF;
// Look into performance optimization of the RNG
use rand::prelude::*;
use services::hittable_service::HittableService;
use services::material_service::MaterialService;
use services::service_locator::ServiceLocator;
use services::texture_service::TextureService;
use crate::core::ray::Ray;
use crate::core::color_rgb::ColorRGB;
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
mod core;

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
    let emitted: ColorRGB = material_service.emission(texture_service, ray, &rec, rec.u, rec.v, &rec.position);
    
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
                has_lights, 
                background, 
                &scattered, 
                depth - 1
            ) /
            pdf_val;
    } else {
        let pdf: Box<dyn PDF> = scatter_record.pdf.expect("Failed to unwrap pdf");
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




    let mut color_buffer = ColorRGB::black();
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

    if color_buffer.r != color_buffer.r { color_buffer.r = 0.0; }
    if color_buffer.g != color_buffer.g { color_buffer.g = 0.0; }
    if color_buffer.b != color_buffer.b { color_buffer.b = 0.0; }

    // Try and apply this scaling to the colors before summation
    color_buffer.r = 255.999 * clamp(0.0, (scale * color_buffer.r).sqrt(), 0.999);
    color_buffer.g = 255.999 * clamp(0.0, (scale * color_buffer.g).sqrt(), 0.999);
    color_buffer.b = 255.999 * clamp(0.0, (scale * color_buffer.b).sqrt(), 0.999);

    color_buffer
}

pub fn render(config_path: &str) {
    let config: RenderConfig = confy::load_path(config_path).expect("Unable to load config file");

    let (_aspect_ratio, image_height, service_locator) = SceneBuilder::build_scene(config.aspect_ratio, config.image_width, config.scene_index);
    
    let scale = 1.0 / (config.samples_per_pixel as f32);

    let now = Instant::now();
    let total_pixels = image_height * config.image_width;
    let image: Vec<ColorRGB> = 
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

    let zero = ColorRGB::black();
    let mut final_image: Vec<ColorRGB> = vec![zero; image.len()];

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
        .map(|v| ((v.r as u32) << 16) | ((v.g as u32) << 8) | v.b as u32)
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

    let mut horizontally_flipped_image: Vec<ColorRGB> = vec![zero; image.len()];
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
            .flat_map(|vector| [vector.r as u8, vector.g as u8, vector.b as u8])
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
