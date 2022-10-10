use ultraviolet::Vec3;
use rand::rngs::ThreadRng;

use crate::{services::hittable_service::HittableService, core::ray::Ray, geometry::aabb::AABB};

use super::{hittable::Hittable, hit_record::HitRecord};

pub struct XYRect {
    material: usize,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
}

impl XYRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: usize) -> XYRect {
        XYRect { material, x0, x1, y0, y1, k }
    }

}

impl Hittable for XYRect {
    fn hit(&self, _rng: &mut ThreadRng, _hittable_service: &HittableService, ray: &Ray, t_min: f32, t_max: f32, hit_out: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t_max < t {
            return false;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;

        if x < self.x0 || self.x1 < x || y < self.y0 || self.y1 < y {
            return false;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);

        hit_out.t = t;
        hit_out.u = u;
        hit_out.v = v;
        hit_out.position = ray.at(t);
        hit_out.set_face_normal(ray, &outward_normal);
        hit_out.material = self.material;

        true
    }

    fn bounding_box(&self, _hittable_service: &HittableService, _time_0: f32, _time_1: f32, box_out: &mut AABB) -> bool {
        box_out.minimum.x = self.x0;
        box_out.minimum.y = self.y0;
        box_out.minimum.z = self.k - 0.0001;

        box_out.maximum.x = self.x1;
        box_out.maximum.y = self.y1;
        box_out.maximum.z = self.k + 0.0001;

        true
    }

}