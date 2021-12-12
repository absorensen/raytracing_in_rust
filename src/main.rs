extern crate minifb;
use minifb::{Key, ScaleMode, Window, WindowOptions};

extern crate raytracing_in_rust;
use raytracing_in_rust::geometry::vector3::*;

const IMAGE_WIDTH: i64 = 256;
const IMAGE_HEIGHT: i64 = 256;
const IMAGE_COLOR_MODE: i64 = 3;

fn main() {
    // Display Image
    let mut image_buffer: Vec<f64> = vec![0.0; (IMAGE_WIDTH * IMAGE_HEIGHT * IMAGE_COLOR_MODE) as usize];

    for row_index in 0..IMAGE_HEIGHT {
        println!("Tracing line {} of {}", row_index, IMAGE_HEIGHT);
        for column_index in 0..IMAGE_WIDTH {
            let color = Vector3{0: column_index as f64 / (IMAGE_WIDTH - 1) as f64, 1: row_index as f64 / (IMAGE_HEIGHT - 1) as f64, 2: 0.25};
            let buffer_offset: usize = ((IMAGE_HEIGHT - 1 - row_index) * IMAGE_WIDTH * IMAGE_COLOR_MODE + column_index * IMAGE_COLOR_MODE + 0) as usize;

            color.color_to_output(&mut image_buffer, buffer_offset);
        }
    }

    let window_buffer: Vec<u32> = image_buffer
        .chunks(3)
        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
        .collect();

    let mut window = Window::new(
        "Ray Tracing in Rust - Press ESC to exit",
        IMAGE_WIDTH as usize,
        IMAGE_HEIGHT as usize,
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
            IMAGE_WIDTH as usize,
            IMAGE_HEIGHT as usize,
        )
        .unwrap();
    }
}
