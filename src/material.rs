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
}

impl Material for Metal {
    fn scatter(&self, ray:&Ray, hit: &HitRecord) -> Option<(Color, Ray)>{
        let reflected = Ray::reflect(&ray.direction.normalized(), &hit.normal);
        let scattered = Ray{origin: hit.position, direction: reflected};
        let attenuation = self.albedo;
        if 0.0 < Vector3::dot(&scattered.direction, &hit.normal){
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}