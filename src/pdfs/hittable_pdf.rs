use nalgebra::Vector3;
use rand::rngs::ThreadRng;

use crate::{services::hittable_service::HittableService};

use super::pdf::PDF;

#[derive(Clone, Copy)]
pub struct HittablePDF {
    pub origin: Vector3::<f32>,
    pub hittable_index: usize,
}

impl HittablePDF {
    pub fn new(origin: &Vector3::<f32>, p: usize) -> HittablePDF {
        HittablePDF{ origin: origin.clone(), hittable_index: p }
    }
}

impl PDF for HittablePDF {
    fn value(&self, rng: &mut ThreadRng, hittable_service: &HittableService, direction: &Vector3::<f32>) -> f32 {
        hittable_service.pdf_value(self.hittable_index, rng, &self.origin, direction)
    }

    fn generate(&self, rng: &mut ThreadRng, hittable_service: &HittableService) -> Vector3::<f32> {
        hittable_service.random(self.hittable_index, rng, &self.origin)
    }
}
