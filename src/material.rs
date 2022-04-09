use rand::Rng;

use crate::vector3::{Color, Vector3};
use crate::ray::Ray;
use crate::hittable::{HitRecord};

pub trait Material : Sync + Send {
    fn scatter(&self, ray:&Ray, hit: &HitRecord, attenuation_out: &mut Color, ray_out: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, ray:&Ray, hit: &HitRecord, attenuation_out: &mut Color, ray_out: &mut Ray) -> bool{
        let mut scatter_direction = hit.position + Vector3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }
        *attenuation_out = self.albedo;
        ray_out.origin = hit.position;
        ray_out.direction = scatter_direction;
        ray_out.time = ray.time;
        return true;
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64, // should be saturated to 1
}

impl Material for Metal {
    fn scatter(&self, ray:&Ray, hit: &HitRecord, attenuation_out: &mut Color, ray_out: &mut Ray) -> bool {
        let reflected = Ray::reflect(&ray.direction.normalized(), &hit.normal);
        ray_out.origin = hit.position;
        ray_out.direction = reflected + self.fuzz * Vector3::random_in_unit_sphere(); 
        ray_out.time = ray.time;

        if 0.0 < Vector3::dot(&ray_out.direction, &hit.normal){
            *attenuation_out = self.albedo;

            return true;
        } else {
            return false;
        }
        return false;
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub ref_idx: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray:&Ray, hit: &HitRecord, attenuation_out: &mut Color, ray_out: &mut Ray) -> bool {
        attenuation_out.x = 1.0;
        attenuation_out.y = 1.0;
        attenuation_out.z = 1.0;

        let (outward_normal, ni_over_nt, cosine) = if Vector3::dot(&ray.direction,&hit.normal) > 0.0 {
            let cosine = self.ref_idx * Vector3::dot(&ray.direction,&hit.normal) / ray.direction.length();
            (-hit.normal, self.ref_idx, cosine)
        } else {
            let cosine = Vector3::dot(&-ray.direction, &hit.normal) / ray.direction.length();
            (hit.normal, 1.0 / self.ref_idx, cosine)
        };
        let mut refracted: Vector3 = Vector3::one();
        if Ray::refract(&ray.direction, &outward_normal, ni_over_nt, &mut refracted) {
            let reflect_prob = Dielectric::reflectance(cosine, self.ref_idx);
            if rand::thread_rng().gen::<f64>() >= reflect_prob {
                ray_out.origin = hit.position;
                ray_out.direction = refracted;
                return true
            }
        }

        ray_out.origin = hit.position;
        ray_out.direction = Ray::reflect(&ray.direction, &hit.normal);
        ray_out.time = ray.time;
        true
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0_squared = r0 * r0;
        let inverse_cosine = 1.0 - cosine;
        let inverse_cosine_squared = inverse_cosine * inverse_cosine;
        r0_squared + (1.0 - r0_squared) * inverse_cosine_squared * inverse_cosine_squared * inverse_cosine
    }
}