use ultraviolet::Vec3;
use rand::{rngs::ThreadRng, Rng};

use crate::{services::hittable_service::HittableService, core::ray::Ray, geometry::aabb::AABB};

use super::{hittable::Hittable, hit_record::HitRecord};

pub struct XZRect {
    material: usize,
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl XZRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: usize) -> XZRect {
        XZRect { material, x0, x1, z0, z1, k }
    }

}

impl Hittable for XZRect {
    fn hit(&self, _rng: &mut ThreadRng, _hittable_service: &HittableService, ray: &Ray, t_min: f32, t_max: f32, hit_out: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t_max < t {
            return false;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        if x < self.x0 || self.x1 < x || z < self.z0 || self.z1 < z {
            return false;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        
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
        box_out.minimum.y = self.k - 0.0001;
        box_out.minimum.z = self.z0;

        box_out.maximum.x = self.x1;
        box_out.maximum.y = self.k + 0.0001;
        box_out.maximum.z = self.z1;

        true
    }

    fn pdf_value(&self, rng: &mut ThreadRng, hittable_service: &HittableService, origin: &Vec3, v: &Vec3) -> f32 {
        let ray = Ray::new_normalized(*origin, *v, 0.0);
        let hit = &mut HitRecord::default();
        
        if !self.hit(rng, hittable_service, &ray, 0.001, f32::INFINITY, hit) {
            return 0.0;
        }

        let area = (self.x1 - self.x0) * (self.z1 - self.z0);
        let distance_squared = hit.t * hit.t * v.mag_sq();
        let cosine = (v.dot(hit.normal) / v.mag()).abs();

        distance_squared / (cosine * area)

    }

    fn random(&self, rng: &mut ThreadRng, _hittable_service: &HittableService, origin: &Vec3) -> Vec3 {
        let random_point: Vec3 = Vec3::new(rng.gen_range(self.x0..self.x1), self.k, rng.gen_range(self.z0..self.z1));

        random_point - *origin
    }

}