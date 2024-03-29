use ultraviolet::Vec3;
use rand::{rngs::ThreadRng};

use crate::{services::hittable_service::HittableService};


pub trait PDF: Sync + Send {
    // Maybe convert these to take an output argument
    fn value(&self, rng: &mut ThreadRng, hittable_service: &HittableService, direction: &Vec3) -> f32;
    fn generate(&self, rng: &mut ThreadRng, hittable_service: &HittableService) -> Vec3;
}



