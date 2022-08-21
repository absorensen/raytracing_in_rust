use crate::core::color_rgb::ColorRGB;
use crate::render::integrator::render_pixel;
use crate::services::image_presentation_service::present_image;
use crate::services::service_locator::ServiceLocator;
use ::core::panic;
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
    let mut config: RenderConfig = confy::load_path(config_path).expect("Unable to load config file");
    if !config.is_initialized {
        config.initialize();
    }

    let service_locator: ServiceLocator = SceneBuilder::build_scene(&config);
    
    let now: Instant = Instant::now();
    let total_pixels: usize = config.image_height * config.image_width;

    //
    // CHECK IF THE SETUP IS VALID
    // 
    if !config.is_initialized {
        panic!("Tried to render without an initialized config file!")
    }

    let image: Vec<ColorRGB> = 
        (0..total_pixels).into_par_iter().map(|pixel_index:usize| {
            let mut rng = rand::thread_rng();
            render_pixel(
                &config,
                &mut rng, 
                &service_locator,
                pixel_index 
            )
        }).collect();

    println!("{} seconds elapsed", now.elapsed().as_millis() as f32 * 0.001);

    present_image(&config, image);

}
