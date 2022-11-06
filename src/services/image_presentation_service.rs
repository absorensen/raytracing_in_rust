use crate::{core::color_rgb::ColorRGB, utility::render_config::RenderConfig};

pub fn save_image(config: &RenderConfig, image: &Vec<ColorRGB>) {
    let black: ColorRGB = ColorRGB::black();
    let mut horizontally_flipped_image: Vec<ColorRGB> = vec![black; image.len()];
    for row_index in 0..(config.image_height / 2) {
        for column_index in 0..config.image_width {
            let row_index_top = (row_index * config.image_width + column_index) as usize;
            let row_index_bottom = ((config.image_height - row_index - 1) * config.image_width + column_index) as usize;
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
        config.image_height.try_into().unwrap(), 
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