use crate::{services::texture_service::TextureService, core::color_rgb::ColorRGB};
use ultraviolet::Vec3;

use super::{default::Default, solid_color::SolidColor, checker::Checker, noise::Noise, image::Image, texture::Texture};

pub enum TextureEnum {
    Default(Default),
    SolidColor(SolidColor),
    Checker(Checker),
    Noise(Noise),
    Image(Image),
}

impl Texture for TextureEnum {
    #[inline]
    fn value(&self, texture_service: &TextureService, u: f32, v: f32, p: &Vec3, color_out: &mut ColorRGB) -> bool {
        match self {
            TextureEnum::Default(default) => default.value(texture_service, u, v, p, color_out),
            TextureEnum::SolidColor(solid_color) => solid_color.value(texture_service, u, v, p, color_out),
            TextureEnum::Checker(checker) => checker.value(texture_service, u, v, p, color_out),
            TextureEnum::Noise(noise) => noise.value(texture_service, u, v, p, color_out),
            TextureEnum::Image(image) => image.value(texture_service, u, v, p, color_out),
        }
    }
}