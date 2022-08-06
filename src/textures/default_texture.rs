use crate::{services::texture_service::TextureService, math::vector3::{Vector3, Color}};

use super::texture::Texture;

pub struct DefaultTexture {
}

impl DefaultTexture {
    pub fn _default() -> Self {
        DefaultTexture{}
    }
}

impl Texture for DefaultTexture {
    fn value(&self, _texture_service: &TextureService, _u: f32, _v: f32, _p: &Vector3, color_out: &mut Color) -> bool {
        color_out.x = 1.0;
        color_out.y = 0.0;
        color_out.z = 0.0;

        true
    }
}
