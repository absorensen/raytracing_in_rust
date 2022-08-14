use minifb::{Key, ScaleMode, Window, WindowOptions};
use crate::core::color_rgb::ColorRGB;
use crate::render::renderer::render_pixel;
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
mod render;



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
                scale
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
