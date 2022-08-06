use crate::{math::vector3::{Color, Vector3}, ray::Ray, hittables::hit_record::HitRecord, services::texture_service::TextureService};

use super::material::Material;

pub struct DiffuseLight {
    pub emission_texture_index: usize,
}

impl DiffuseLight {
    pub fn new(texture_index: usize) -> DiffuseLight {
        DiffuseLight { emission_texture_index: texture_index }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, texture_service: &TextureService, _ray:&Ray, hit: &HitRecord, u: f32, v: f32, point: &Vector3) -> Color {
        if hit.is_front_face {
            let mut color_out = Color::zero();
            texture_service.value(self.emission_texture_index, u, v, point, &mut color_out);
            return color_out;
        }
        Color{x: 0.0, y: 0.0, z: 0.0}
    }
}