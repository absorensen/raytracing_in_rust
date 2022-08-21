use crate::core::color_rgb::ColorRGB;
use crate::render::integrator::render_pixel;
use crate::services::image_presentation_service::present_image;
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
    let use_loop_rendering: bool = true;

    let now = Instant::now();
    let total_pixels = image_height * config.image_width;
    let image: Vec<ColorRGB> = 
        (0..total_pixels).into_par_iter().map(|pixel_index:usize| {
            let mut rng = rand::thread_rng();
            render_pixel(
                use_loop_rendering,
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

    present_image(&config, image, image_height);

}
