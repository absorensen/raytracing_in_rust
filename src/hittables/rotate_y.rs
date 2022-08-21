use nalgebra::Vector3;
use rand::rngs::ThreadRng;

use crate::{geometry::aabb::AABB, services::hittable_service::HittableService, core::ray::Ray};

use super::{hittable::Hittable, hit_record::HitRecord};

pub struct RotateY {
    model_index: usize,
    sin_theta: f32,
    cos_theta: f32,
    has_bbox: bool,
    bbox: AABB,
}

impl RotateY {
    pub fn new(hittable_service: &HittableService, angle: f32, model_index:usize) -> RotateY {
        let radians = angle.to_radians();

        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let mut bbox = AABB::default();
        let has_bbox = hittable_service.bounding_box(model_index, 0.0, 1.0, &mut bbox);

        let mut min: Vector3<f32> = Vector3::<f32>::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max: Vector3<f32> = Vector3::<f32>::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
        
        for i in 0..2 {
            let i_f = i as f32;
            for j in 0..2 {
                let j_f = j as f32;
                for k in 0..2 {
                    let k_f = k as f32;

                    let x = i_f  * bbox.maximum.x + (1.0 - i_f) * bbox.minimum.x;
                    let y = j_f  * bbox.maximum.y + (1.0 - j_f) * bbox.minimum.y;
                    let z = k_f  * bbox.maximum.z + (1.0 - k_f) * bbox.minimum.z;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester: Vector3<f32> = Vector3::<f32>::new(new_x, y, new_z);

                    min[0] = min[0].min(tester[0]);
                    min[1] = min[1].min(tester[1]);
                    min[2] = min[2].min(tester[2]);
                    max[0] = max[0].max(tester[0]);
                    max[1] = max[1].max(tester[1]);
                    max[2] = max[2].max(tester[2]);
                }
            }
        }

        let bbox = AABB{minimum: min, maximum: max};

        RotateY { model_index, sin_theta, cos_theta, has_bbox, bbox }
    }
}

impl Hittable for RotateY {
    fn hit(&self, rng: &mut ThreadRng, hittable_service: &HittableService, ray: &Ray, t_min: f32, t_max: f32, hit_out: &mut HitRecord) -> bool {
        let mut origin = ray.origin.clone();
        let mut direction = ray.direction.clone();

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_ray = Ray{ origin, direction, time: ray.time };

        if !hittable_service.hit(self.model_index, rng, &rotated_ray, t_min, t_max, hit_out) {
            return false;
        }

        let mut position = hit_out.position.clone();
        let mut normal = hit_out.normal.clone();

        position[0] = self.cos_theta * hit_out.position[0] + self.sin_theta * hit_out.position[2];
        position[2] = -self.sin_theta * hit_out.position[0] + self.cos_theta * hit_out.position[2];

        normal[0] = self.cos_theta * hit_out.normal[0] + self.sin_theta * hit_out.normal[2];
        normal[2] = -self.sin_theta * hit_out.normal[0] + self.cos_theta * hit_out.normal[2];

        hit_out.position = position;
        // Cloning the normal here is poop, and should be refactored somehow.
        // The issues is hit is borrowed mutably for set_face_normal, making 
        // impossible the immutable borrow for outward_normal
        hit_out.set_face_normal(&rotated_ray, &normal);


        true
    }

    fn bounding_box(&self, _hittable_service: &HittableService, _time_0: f32, _time_1: f32, box_out: &mut AABB) -> bool {
        if !self.has_bbox { return false; }

        box_out.minimum.x = self.bbox.minimum.x;
        box_out.minimum.y = self.bbox.minimum.y;
        box_out.minimum.z = self.bbox.minimum.z;

        box_out.maximum.x = self.bbox.maximum.x;
        box_out.maximum.y = self.bbox.maximum.y;
        box_out.maximum.z = self.bbox.maximum.z;

        true
    }

}
