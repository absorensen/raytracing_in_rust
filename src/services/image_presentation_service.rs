use minifb::{Key, ScaleMode, Window, WindowOptions};

use crate::{core::color_rgb::ColorRGB, utility::render_config::RenderConfig};

pub fn present_image(config: &RenderConfig, image: Vec<ColorRGB>, image_height: usize) {
    let window_buffer: Vec<u32> = convert_image_for_presentation(config, &image, image_height);

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
    save_image(config, &image, image_height);

}

fn convert_image_for_presentation(config: &RenderConfig, image: &Vec<ColorRGB>, image_height: usize) -> Vec<u32> {
    let black: ColorRGB = ColorRGB::black();
    let mut final_image: Vec<ColorRGB> = vec![black; image.len()];

    for row_index in 0..image_height {
        for column_index in 0..(config.image_width / 2) {
            let column_index_left = (row_index * config.image_width + column_index) as usize;
            let column_index_right = (row_index * config.image_width + (config.image_width - column_index - 1)) as usize;
            final_image[column_index_left] = image[column_index_right];
            final_image[column_index_right] = image[column_index_left];
        }
    }

    final_image
        .iter()
        .map(|v| ((v.r as u32) << 16) | ((v.g as u32) << 8) | v.b as u32)
        .rev()
        .collect()
}

fn save_image(config: &RenderConfig, image: &Vec<ColorRGB>, image_height: usize) {
    let black: ColorRGB = ColorRGB::black();
    let mut horizontally_flipped_image: Vec<ColorRGB> = vec![black; image.len()];
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