use std::sync::Arc;

use crate::material::Material;
use crate::vector3::Vector3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::aabb::AABB;

pub struct Sphere {
    pub radius: f64,
    pub center: Vector3,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: &Arc<dyn Material>) -> Self { 
        Sphere {center, radius, material: Arc::clone(material) } 
    }
}

impl Hittable for Sphere{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vector3::dot(&ray.direction, &oc);
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let position = ray.at(root);
        let normal = (position - self.center) / self.radius;
        let hit_rec = HitRecord::new(ray, root, &position, &normal, &self.material);  

        Some(hit_rec)
    }

    
    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<AABB> {
        Some(AABB{minimum:self.center - Vector3{x: self.radius, y: self.radius, z: self.radius}, maximum:self.center + Vector3{x: self.radius, y: self.radius, z: self.radius}})
    }
}