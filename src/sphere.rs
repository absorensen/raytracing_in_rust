use crate::material::Material;
use crate::vector3::Vector3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use std::sync::Arc;

pub struct Sphere<M: Material + 'static> {
    pub radius: f64,
    pub center: Vector3,
    pub material: Arc<M>,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vector3, radius: f64, material: Arc<M>) -> Self { 
        Sphere {center, radius, material} 
    }
}

impl<M:Material> Hittable for Sphere<M>{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = Vector3::dot(&ray.direction, &ray.direction);
        let b = Vector3::dot(&oc,&ray.direction);
        let c = Vector3::dot(&oc,&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord { t, position: p, normal, material: self.material.clone() })
            }
            let t = (-b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord { t, position: p, normal, material: self.material.clone() })
            }
        }
        None
    }
}