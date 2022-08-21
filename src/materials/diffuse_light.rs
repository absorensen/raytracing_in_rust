use crate::{core::{ray::Ray, color_rgb::ColorRGB}, hittables::hit_record::HitRecord, services::texture_service::TextureService};

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
    fn emitted(&self, texture_service: &TextureService, _ray:&Ray, hit: &HitRecord, emitted_out: &mut ColorRGB) {
        if hit.is_front_face {
            texture_service.value(self.emission_texture_index, hit.u, hit.v, &hit.position, emitted_out);
            return;
        }

        emitted_out.r = 0.0;
        emitted_out.g = 0.0;
        emitted_out.b = 0.0;
    }
}