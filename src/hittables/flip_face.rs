use rand::rngs::ThreadRng;

use crate::{services::hittable_service::HittableService, core::ray::Ray, geometry::aabb::AABB};

use super::{hittable::Hittable, hit_record::HitRecord};

pub struct FlipFace {
    model_index: usize,
}

// Test whether this should be a bool on the other hittables instead
impl FlipFace {
    pub fn new(model_index: usize) -> FlipFace {
        FlipFace{model_index}
    }
}

impl Hittable for FlipFace {
    fn hit(&self, rng: &mut ThreadRng, hittable_service: &HittableService, ray: &Ray, t_min: f32, t_max: f32, hit_out: &mut HitRecord) -> bool {
        if !hittable_service.hit(self.model_index, rng, &ray, t_min, t_max, hit_out) {
            return false;            
        }

        // TODO: Shouldn't this also flip the normal?
        hit_out.is_front_face = !hit_out.is_front_face;

        true
    }

    fn bounding_box(&self, hittable_service: &HittableService, time_0: f32, time_1: f32, box_out: &mut AABB) -> bool {
        hittable_service.bounding_box(self.model_index, time_0, time_1, box_out)
    }
}