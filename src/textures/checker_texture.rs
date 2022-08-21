use nalgebra::Vector3;

use crate::{services::texture_service::TextureService, core::color_rgb::ColorRGB};

use super::texture::Texture;

pub struct CheckerTexture {
    odd: usize,
    even: usize,
}

impl CheckerTexture {
    pub fn new(odd: usize, even: usize) -> Self {
        CheckerTexture{odd, even}
    }

}

impl Texture for CheckerTexture {
    fn value(&self, texture_service: &TextureService, u: f32, v: f32, p: &Vector3<f32>, color_out: &mut ColorRGB) -> bool {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            return texture_service.value(self.odd, u, v, p, color_out);
        } else {
            return texture_service.value(self.even, u, v, p, color_out);
        }
    }
}
