use nalgebra::Vector3;

use crate::{services::texture_service::TextureService, core::color_rgb::ColorRGB};

use super::texture::Texture;

pub struct DefaultTexture {
}

impl DefaultTexture {
    pub fn _default() -> Self {
        DefaultTexture{}
    }
}

impl Texture for DefaultTexture {
    fn value(&self, _texture_service: &TextureService, _u: f32, _v: f32, _p: &Vector3<f32>, color_out: &mut ColorRGB) -> bool {
        color_out.r = 1.0;
        color_out.g = 0.0;
        color_out.b = 0.0;

        true
    }
}
