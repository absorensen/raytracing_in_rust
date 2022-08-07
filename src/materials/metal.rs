use rand::rngs::ThreadRng;

use crate::{math::vector3::{Vector3}, services::texture_service::TextureService, core::{ray::Ray, color_rgb::ColorRGB}, hittables::hit_record::HitRecord};

use super::{material::Material, scatter_record::ScatterRecord};

pub struct Metal {
    pub albedo: ColorRGB,
    pub fuzz: f32, // should be saturated to 1
}

impl Metal {
    pub fn new(albedo: ColorRGB, fuzz:f32) -> Metal {
        Metal { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } }
    }
}

impl Material for Metal {
    fn scatter(&self, rng: &mut ThreadRng, _texture_service: &TextureService, ray:&Ray, hit: &HitRecord, scatter_out: &mut ScatterRecord) -> bool {
        let mut reflected = Vector3::default(); 
        Vector3::reflect(&ray.direction.normalized(), &hit.normal, &mut reflected);
        scatter_out.specular_ray = Ray::new(hit.position, reflected + self.fuzz * Vector3::random_in_unit_sphere(rng), ray.time);
        scatter_out.attenuation = self.albedo;
        scatter_out.is_specular = true;
        scatter_out.pdf = None;
        true
    }
}
