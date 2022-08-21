use nalgebra::Vector3;
use rand::rngs::ThreadRng;

use crate::{services::hittable_service::HittableService, core::ray::Ray, geometry::aabb::AABB};

use super::{hittable::Hittable, hit_record::HitRecord};

pub struct YZRect {
    material: usize,
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl YZRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: usize) -> YZRect {
        YZRect { material, y0, y1, z0, z1, k }
    }

}

impl Hittable for YZRect {
    fn hit(&self, _rng: &mut ThreadRng, _hittable_service: &HittableService, ray: &Ray, t_min: f32, t_max: f32, hit_out: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t_max < t {
            return false;
        }

        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;


        if y < self.y0 || self.y1 < y || z < self.z0 || self.z1 < z {
            return false;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal: Vector3<f32> = Vector3::<f32>::new(1.0, 0.0, 0.0);
        
        hit_out.t = t;
        hit_out.u = u;
        hit_out.v = v;
        hit_out.position = ray.at(t);
        hit_out.set_face_normal(ray, &outward_normal);
        hit_out.material = self.material;

        true
    }

    fn bounding_box(&self, _hittable_service: &HittableService, _time_0: f32, _time_1: f32, box_out: &mut AABB) -> bool {
        box_out.minimum.x = self.k - 0.0001;
        box_out.minimum.y = self.y0;
        box_out.minimum.z = self.z0;

        box_out.maximum.x = self.k + 0.0001;
        box_out.maximum.y = self.y1;
        box_out.maximum.z = self.z1;

        true
    }

}