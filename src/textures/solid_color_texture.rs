use crate::{services::texture_service::TextureService, math::vector3::{Vector3}, core::color_rgb::ColorRGB};

use super::texture::Texture;

pub struct SolidColorTexture {
    color: ColorRGB,
}

impl SolidColorTexture {
    pub fn from_color(color: &ColorRGB) -> Self {
        SolidColorTexture{ color: color.clone() }
    }
}

impl Texture for SolidColorTexture {
    fn value(&self, _texture_service: &TextureService, _u: f32, _v: f32, _p: &Vector3, color_out: &mut ColorRGB) -> bool {
        color_out.r = self.color.r;
        color_out.g = self.color.g;
        color_out.b = self.color.b;

        true
    }
}
