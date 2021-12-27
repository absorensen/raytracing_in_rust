use crate::material::Material;
use crate::vector3::Vector3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};

pub struct Sphere<M: Material> {
    pub radius: f64,
    pub center: Vector3,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vector3, radius: f64, material: M) -> Self { 
        Sphere {center, radius, material} 
    }
}

impl<M:Material> Hittable for Sphere<M>{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vector3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
    
        if discriminant < 0.0 {
            return None
        }

        let sqrt_discriminant = f64::sqrt(discriminant);
        let root = (-half_b - sqrt_discriminant) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || t_max < root {
                return None
            }
        }

        let position = ray.at(root);
        let mut record = HitRecord{t: root, position: position, normal: (position - self.center) / self.radius, front_face: true, material: &self.material};
        let outward_normal = (record.position - self.center) / self.radius;
        record.set_face_normal(ray, &outward_normal);
        Some(record)
    }
}