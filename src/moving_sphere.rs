use crate::material::Material;
use crate::vector3::Vector3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};

pub struct MovingSphere<M: Material> {
    pub radius: f64,
    pub center_0: Vector3,
    pub center_1: Vector3,
    pub material: M,
    pub time_0: f64,
    pub time_1: f64,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(radius: f64, center_0: Vector3, center_1: Vector3,  material: M, time_0: f64, time_1: f64) -> Self { 
        MovingSphere {
            radius, 
            center_0,
            center_1,
            material,
            time_0,
            time_1 
        } 
    }
}

impl<M: Material> MovingSphere<M> {
    pub fn center(&self, time: f64) -> Vector3 {
        self.center_0 + ((time - self.time_0) / (self.time_1 - self.time_0)) * (self.center_1 - self.center_0)
    }
}

impl<M:Material> Hittable for MovingSphere<M>{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let a = Vector3::dot(&ray.direction, &ray.direction);
        let b = Vector3::dot(&oc,&ray.direction);
        let c = Vector3::dot(&oc,&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.at(t);
                let normal = (p - self.center(ray.time)) / self.radius;
                return Some(HitRecord { t, position: p, normal, material: &self.material })
            }
            let t = (-b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.at(t);
                let normal = (p - self.center(ray.time)) / self.radius;
                return Some(HitRecord { t, position: p, normal, material: &self.material })
            }
        }
        None
    }
}