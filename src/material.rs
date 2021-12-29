use rand::Rng;

use crate::vector3::{Color, Vector3};
use crate::ray::Ray;
use crate::hittable::{HitRecord};

pub trait Material : Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, ray:&Ray, hit: &HitRecord) -> Option<(Color, Ray)>{
        let mut scatter_direction = hit.position + Vector3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        Some((self.albedo, Ray{origin: hit.position, direction:scatter_direction}))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64, // should be saturated to 1
}

impl Material for Metal {
    fn scatter(&self, ray:&Ray, hit: &HitRecord) -> Option<(Color, Ray)>{
        let reflected = Ray::reflect(&ray.direction.normalized(), &hit.normal);
        let scattered = Ray{origin: hit.position, direction: reflected + self.fuzz * Vector3::random_in_unit_sphere()};
        let attenuation = self.albedo;
        if 0.0 < Vector3::dot(&scattered.direction, &hit.normal){
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub ref_idx: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray:&Ray, hit: &HitRecord) -> Option<(Color, Ray)>{
        let attenuation = Color{x: 1.0, y: 1.0, z: 1.0};
        let (outward_normal, ni_over_nt, cosine) = if Vector3::dot(&ray.direction,&hit.normal) > 0.0 {
            let cosine = self.ref_idx * Vector3::dot(&ray.direction,&hit.normal) / ray.direction.length();
            (-hit.normal, self.ref_idx, cosine)
        } else {
            let cosine = Vector3::dot(&-ray.direction, &hit.normal) / ray.direction.length();
            (hit.normal, 1.0 / self.ref_idx, cosine)
        };
        let refracted = Ray::refract(&ray.direction, &outward_normal, ni_over_nt);

        if refracted.is_some(){
            let reflect_prob = Dielectric::reflectance(cosine, self.ref_idx);
            if rand::thread_rng().gen::<f64>() >= reflect_prob {
                let scattered = Ray::new(hit.position, refracted.unwrap());
                return Some((attenuation, scattered))
            }
        }

        let reflected = Ray::reflect(&ray.direction, &hit.normal);
        let scattered = Ray::new(hit.position, reflected);
        Some((attenuation, scattered))
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