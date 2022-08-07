use rand::rngs::ThreadRng;

use crate::{core::ray::Ray, services::texture_service::TextureService, hittables::hit_record::HitRecord, math::vector3::Vector3};

use super::{material::Material, scatter_record::ScatterRecord};

pub struct Isotropic {
    pub albedo_texture_index: usize,
}

impl Isotropic {
    pub fn new(texture_index: usize) -> Isotropic {
        Isotropic { albedo_texture_index: texture_index }
    }
}

impl Material for Isotropic {
    fn scatter(&self, rng: &mut ThreadRng, texture_service: &TextureService, ray:&Ray, hit: &HitRecord, scatter_out: &mut ScatterRecord) -> bool{
        scatter_out.is_specular = true;
        scatter_out.specular_ray = Ray{ origin: hit.position, direction: Vector3::random_in_unit_sphere(rng), time: ray.time };
        texture_service.value(self.albedo_texture_index, hit.u, hit.v, &hit.position, &mut scatter_out.attenuation)
    }
}