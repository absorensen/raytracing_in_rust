use rand::rngs::ThreadRng;

use crate::{services::texture_service::TextureService, math::vector3::{Vector3, Color}, noise::perlin::Perlin};

use super::texture::Texture;

pub struct NoiseTexture {
    perlin: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(rng: &mut ThreadRng, point_count: u32, scale: f32) -> Self {
        NoiseTexture{perlin: Perlin::new(rng, point_count), scale}
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _texture_service: &TextureService, _u: f32, _v: f32, point: &Vector3, color_out: &mut Color) -> bool {
        color_out.x = 1.0;
        color_out.y = 1.0;
        color_out.z = 1.0;

        *color_out = *color_out * 0.5 * (1.0 + (self.scale * point.z + 10.0 * self.perlin.turbulence_default(point)).sin());
        true
    }
}