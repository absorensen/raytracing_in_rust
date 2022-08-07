use std::f32::consts::PI;

use rand::rngs::ThreadRng;

use crate::{services::texture_service::TextureService, core::ray::Ray, hittables::hit_record::HitRecord, math::vector3::Vector3, pdfs::cosine_pdf::CosinePDF};

use super::{material::Material, scatter_record::ScatterRecord};

pub struct Lambertian {
    pub albedo_texture_index: usize
}

impl Lambertian {
    pub fn new(albedo_index: usize) -> Self {
        Lambertian { albedo_texture_index: albedo_index }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _rng: &mut ThreadRng, texture_service: &TextureService, _ray:&Ray, hit: &HitRecord, scatter_out: &mut ScatterRecord) -> bool {
        scatter_out.is_specular = false;
        texture_service.value(self.albedo_texture_index, hit.u, hit.v, &hit.position, &mut scatter_out.attenuation);
        scatter_out.pdf = Some(Box::new(CosinePDF::new(&hit.normal)));

        return true;
    }

    fn scattering_pdf(&self, _rng: &mut ThreadRng, _ray: &Ray, hit: &HitRecord, scattered_ray:&Ray) -> f32 {
        let cosine = Vector3::dot(&hit.normal, &(scattered_ray.direction.normalized()));

        if cosine < 0.0 { 0.0 } else { cosine / PI }
    }

}