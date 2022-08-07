use rand::rngs::ThreadRng;

use crate::{math::vector3::Vector3, services::hittable_service::HittableService, core::ray::Ray, geometry::aabb::AABB};

use super::{hittable::Hittable, hit_record::HitRecord};

pub struct Translate {
    model_index: usize,
    offset: Vector3,
}

impl Translate {
    pub fn new(displacement: Vector3, model_index: usize) -> Translate {
        Translate{model_index, offset: displacement}
    }

}

impl Hittable for Translate {
    fn hit(&self, rng: &mut ThreadRng, hittable_service: &HittableService, ray: &Ray, t_min: f32, t_max: f32, hit_out: &mut HitRecord) -> bool {
        let moved_ray = Ray{ origin: ray.origin - self.offset, direction: ray.direction, time: ray.time };
        if !hittable_service.hit(self.model_index, rng, &moved_ray, t_min, t_max, hit_out) {
            return false;
        }

        hit_out.position += self.offset;
        // Cloning the normal here is poop, and should be refactored somehow.
        // The issues is hit is borrowed mutably for set_face_normal, making 
        // impossible the immutable borrow for outward_normal
        hit_out.set_face_normal(&moved_ray, &hit_out.normal.clone());

        true
    }

    fn bounding_box(&self, hittable_service: &HittableService, time_0: f32, time_1: f32, box_out: &mut AABB) -> bool {
        if !hittable_service.bounding_box(self.model_index, time_0, time_1, box_out) {
            return false;
        } 

        box_out.minimum += self.offset;
        box_out.maximum += self.offset;

        true
    }

}