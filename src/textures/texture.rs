use crate::{math::vector3::{Vector3, Color}, services::texture_service::TextureService};

pub trait Texture : Sync + Send {
    fn value(&self, texture_service: &TextureService, u: f32, v: f32, p: &Vector3, color_out: &mut Color) -> bool;
}