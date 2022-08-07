use crate::{math::vector3::{Vector3}, services::texture_service::TextureService, core::color_rgb::ColorRGB};

pub trait Texture : Sync + Send {
    fn value(&self, texture_service: &TextureService, u: f32, v: f32, p: &Vector3, color_out: &mut ColorRGB) -> bool;
}