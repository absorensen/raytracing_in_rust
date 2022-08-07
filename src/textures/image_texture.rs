use crate::{services::texture_service::TextureService, math::vector3::{Vector3}, core::color_rgb::ColorRGB};

use super::texture::Texture;
pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
    bytes_per_pixel: usize,
    bytes_per_scanline: usize,
}

impl ImageTexture {
    pub fn new(path: &str) -> Self {
        let bytes_per_pixel: usize = 3;

        let image = image::open(path).expect("image not found").flipv().to_rgb8();
        let (width, height) = image.dimensions();
        let data = image.into_raw();

        ImageTexture{data, width: width as usize, height: height as usize, bytes_per_pixel, bytes_per_scanline: bytes_per_pixel * width as usize}
    }
}

impl Texture for ImageTexture {
    fn value(&self, _texture_service: &TextureService, u: f32, v: f32, _point: &Vector3, color_out: &mut ColorRGB) -> bool {
        if self.data.len() < 1 {
            color_out.r = 0.0;
            color_out.g = 1.0;
            color_out.b = 1.0;
            
            return false;
        }

        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        let mut i = ( u * self.width as f32 ) as usize;
        let mut j = ( v * self.height as f32 ) as usize;

        if self.width <= i { i = self.width - 1; }
        if self.height <= j { j = self.height - 1; } 

        let color_scale = 1.0 / 255.0;
        let pixel_index = j * self.bytes_per_scanline + i * self.bytes_per_pixel;

        color_out.r = color_scale * self.data[pixel_index    ] as f32;
        color_out.g = color_scale * self.data[pixel_index + 1] as f32;
        color_out.b = color_scale * self.data[pixel_index + 2] as f32;


        true
    }
}