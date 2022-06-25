use std::sync::Arc;

use rand::{Rng, rngs::ThreadRng};

use crate::vector3::{Color, Vector3};
use crate::ray::Ray;
use crate::hittable::{HitRecord};
use crate::texture::{Texture};

pub trait Material : Sync + Send {
    fn scatter(&self, rng: &mut ThreadRng, ray:&Ray, hit: &HitRecord, attenuation_out: &mut Color, ray_out: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>
}

impl Material for Lambertian {
    fn scatter(&self, rng: &mut ThreadRng, ray:&Ray, hit: &HitRecord, attenuation_out: &mut Color, ray_out: &mut Ray) -> bool{
        let mut scatter_direction = hit.normal + Vector3::random_unit_vector(rng);

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        ray_out.origin = hit.position;
        ray_out.direction = scatter_direction;
        ray_out.time = ray.time;

        self.albedo.value(hit.u, hit.v, &hit.position, attenuation_out);

        return true;
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64, // should be saturated to 1
}

impl Material for Metal {
    fn scatter(&self, rng: &mut ThreadRng, ray:&Ray, hit: &HitRecord, attenuation_out: &mut Color, ray_out: &mut Ray) -> bool {
        let mut reflected = Vector3::zero();
        Vector3::reflect(&ray.direction.normalized(), &hit.normal, &mut reflected);
        ray_out.origin = hit.position;
        ray_out.direction = reflected + self.fuzz * Vector3::random_in_unit_sphere(rng); 
        ray_out.time = ray.time;

        if 0.0 < Vector3::dot(&ray_out.direction, &hit.normal){
            *attenuation_out = self.albedo;
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub index_of_refraction: f64,
    pub inverse_index_of_refraction: f64,
}

impl Material for Dielectric {
    fn scatter(&self, rng: &mut ThreadRng, ray:&Ray, hit: &HitRecord, attenuation_out: &mut Color, ray_out: &mut Ray) -> bool {
        attenuation_out.x = 1.0;
        attenuation_out.y = 1.0;
        attenuation_out.z = 1.0;

        let refraction_ratio = if hit.is_front_face { self.inverse_index_of_refraction } else { self.index_of_refraction };
        let unit_direction = Vector3::normalized(&ray.direction);
        let cos_theta = Vector3::dot(&-unit_direction, &hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = 1.0 < refraction_ratio * sin_theta;
        let mut direction = Vector3::zero();
        if cannot_refract || rng.gen::<f64>() < Dielectric::reflectance(cos_theta, refraction_ratio) {
            Vector3::reflect(&unit_direction, &hit.normal, &mut direction);
        } else {
            Vector3::refract(&unit_direction, &hit.normal, refraction_ratio, &mut direction);
        };

        ray_out.origin = hit.position;
        ray_out.direction = direction;

        true
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, index_of_refraction: f64) -> f64 {
        let r0 = (1.0 - index_of_refraction) / (1.0 + index_of_refraction);
        let r0_squared = r0 * r0;
        let inverse_cosine = 1.0 - cosine;
        let inverse_cosine_squared = inverse_cosine * inverse_cosine;
        r0_squared + (1.0 - r0_squared) * inverse_cosine_squared * inverse_cosine_squared * inverse_cosine
    }
}