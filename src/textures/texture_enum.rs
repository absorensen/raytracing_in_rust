use crate::{services::texture_service::TextureService, core::color_rgb::ColorRGB};
use ultraviolet::Vec3;

use super::{default_texture::DefaultTexture, solid_color_texture::SolidColorTexture, checker_texture::CheckerTexture, noise_texture::NoiseTexture, image_texture::ImageTexture, texture::Texture};

pub enum TextureEnum {
    DefaultTexture(DefaultTexture),
    SolidColorTexture(SolidColorTexture),
    CheckerTexture(CheckerTexture),
    NoiseTexture(NoiseTexture),
    ImageTexture(ImageTexture),
}

impl Texture for TextureEnum {
    #[inline]
    fn value(&self, texture_service: &TextureService, u: f32, v: f32, p: &Vec3, color_out: &mut ColorRGB) -> bool {
        match self {
            TextureEnum::DefaultTexture(default) => default.value(texture_service, u, v, p, color_out),
            TextureEnum::SolidColorTexture(solid_color) => solid_color.value(texture_service, u, v, p, color_out),
            TextureEnum::CheckerTexture(checker) => checker.value(texture_service, u, v, p, color_out),
            TextureEnum::NoiseTexture(noise) => noise.value(texture_service, u, v, p, color_out),
            TextureEnum::ImageTexture(image) => image.value(texture_service, u, v, p, color_out),
        }
    }
}