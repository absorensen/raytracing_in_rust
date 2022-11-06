use ultraviolet::Vec3;
use rand::rngs::ThreadRng;

use crate::{services::hittable_service::HittableService};

use super::pdf::PDF;

#[derive(Clone, Copy)]
pub struct HittablePDF {
    pub origin: Vec3,
    pub hittable_index: usize,
}

impl HittablePDF {
    pub fn new(origin: &Vec3, p: usize) -> HittablePDF {
        HittablePDF{ origin: *origin, hittable_index: p }
    }
}

impl PDF for HittablePDF {
    fn value(&self, rng: &mut ThreadRng, hittable_service: &HittableService, direction: &Vec3) -> f32 {
        hittable_service.pdf_value(self.hittable_index, rng, &self.origin, direction)
    }

    fn generate(&self, rng: &mut ThreadRng, hittable_service: &HittableService) -> Vec3 {
        hittable_service.random(self.hittable_index, rng, &self.origin)
    }
}
