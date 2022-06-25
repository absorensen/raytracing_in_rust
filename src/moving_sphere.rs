use std::sync::Arc;

use crate::material::Material;
use crate::vector3::Vector3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::aabb::AABB;

pub struct MovingSphere {
    pub radius: f64,
    pub center_0: Vector3,
    pub center_1: Vector3,
    pub material: Arc<dyn Material>,
    pub time_0: f64,
    pub time_1: f64,
}

impl MovingSphere {
    pub fn new(radius: f64, center_0: Vector3, center_1: Vector3, material: &Arc<dyn Material>, time_0: f64, time_1: f64) -> Self { 
        MovingSphere {
            radius, 
            center_0,
            center_1,
            material: Arc::clone(material),
            time_0,
            time_1 
        } 
    }
}

impl MovingSphere {
    pub fn center(&self, time: f64) -> Vector3 {
        self.center_0 + ((time - self.time_0) / (self.time_1 - self.time_0)) * (self.center_1 - self.center_0)
    }
}

impl Hittable for MovingSphere{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center = self.center(ray.time);

        let oc = ray.origin - center;
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
        let normal = (position - center) / self.radius;
        let hit_rec = HitRecord::new(ray, root, &position, &normal, &self.material);  

        Some(hit_rec)
    }



    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB> {
        let mut output_box = AABB{minimum:self.center(time_0) - Vector3{x: self.radius, y: self.radius, z: self.radius}, maximum:self.center_0 + Vector3{x: self.radius, y: self.radius, z: self.radius}};
        output_box.expand_by_point(&(self.center(time_1) - Vector3{x: self.radius, y: self.radius, z: self.radius}));
        output_box.expand_by_point(&(self.center(time_1) + Vector3{x: self.radius, y: self.radius, z: self.radius}));

        Some(output_box)
    }
}