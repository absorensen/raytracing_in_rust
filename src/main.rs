extern crate minifb;
use minifb::{Key, ScaleMode, Window, WindowOptions};

const IMAGE_WIDTH: i64 = 256;
const IMAGE_HEIGHT: i64 = 256;
const IMAGE_COLOR_MODE: i64 = 3;

fn main() {
    // Display Image
    let mut image_buffer: Vec<f64> = vec![0.0; (IMAGE_WIDTH * IMAGE_HEIGHT * IMAGE_COLOR_MODE) as usize];

    for row_index in 0..IMAGE_HEIGHT {
        for column_index in 0..IMAGE_WIDTH {
            let r = column_index as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = row_index as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            image_buffer[(row_index * IMAGE_WIDTH * IMAGE_COLOR_MODE + column_index * IMAGE_COLOR_MODE + 0) as usize] = (255.999 * r) as f64;
            image_buffer[(row_index * IMAGE_WIDTH * IMAGE_COLOR_MODE + column_index * IMAGE_COLOR_MODE + 1) as usize] = (255.999 * g) as f64;
            image_buffer[(row_index * IMAGE_WIDTH * IMAGE_COLOR_MODE + column_index * IMAGE_COLOR_MODE + 2) as usize] = (255.999 * b) as f64;
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
