use ultraviolet::Vec3;
use crate::{services::texture_service::TextureService, core::color_rgb::ColorRGB};

pub trait Texture : Sync + Send {
    fn value(&self, texture_service: &TextureService, u: f32, v: f32, p: &Vec3, color_out: &mut ColorRGB) -> bool;
}