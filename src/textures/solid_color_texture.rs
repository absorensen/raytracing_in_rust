use crate::{services::texture_service::TextureService, math::vector3::{Vector3, Color}};

use super::texture::Texture;

pub struct SolidColorTexture {
    color: Color,
}

impl SolidColorTexture {
    pub fn from_color(color: &Color) -> Self {
        SolidColorTexture{ color: color.clone() }
    }
}

impl Texture for SolidColorTexture {
    fn value(&self, _texture_service: &TextureService, _u: f32, _v: f32, _p: &Vector3, color_out: &mut Color) -> bool {
        color_out.x = self.color.x;
        color_out.y = self.color.y;
        color_out.z = self.color.z;

        true
    }
}
