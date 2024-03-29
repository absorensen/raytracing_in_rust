use ultraviolet::Vec3;
use crate::{services::texture_service::TextureService, core::color_rgb::ColorRGB};

use super::texture::Texture;

pub struct SolidColor {
    color: ColorRGB,
}

impl SolidColor {
    pub fn from_color(color: &ColorRGB) -> Self {
        SolidColor{ color: *color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _texture_service: &TextureService, _u: f32, _v: f32, _p: &Vec3, color_out: &mut ColorRGB) -> bool {
        color_out.r = self.color.r;
        color_out.g = self.color.g;
        color_out.b = self.color.b;

        true
    }
}
