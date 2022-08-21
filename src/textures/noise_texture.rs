use nalgebra::{Vector3, Point3};
use rand::rngs::ThreadRng;

use crate::{services::texture_service::TextureService, noise::perlin::Perlin, core::color_rgb::ColorRGB};

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
    fn value(&self, _texture_service: &TextureService, _u: f32, _v: f32, point: &Vector3<f32>, color_out: &mut ColorRGB) -> bool {
        color_out.r = 1.0;
        color_out.g = 1.0;
        color_out.b = 1.0;
        let point: Point3<f32> = Point3::<f32>::new(point.x, point.y, point.z);
        *color_out = *color_out * 0.5 * (1.0 + (self.scale * point.z + 10.0 * self.perlin.turbulence_default(&point)).sin());
        true
    }
}