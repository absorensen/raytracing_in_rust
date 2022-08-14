use rand::{rngs::ThreadRng, Rng};

use crate::{services::hittable_service::HittableService, math::vector3::Vector3};

use super::pdf::PDF;

pub struct MixturePDF {
    // Get these removed into an array of pdf enums
    pdfs: Vec<Box<dyn PDF>>,
    probability: f32,
}

impl MixturePDF {
    pub fn new(pdfs: Vec<Box<dyn PDF>>) -> MixturePDF {
        let probability = 1.0 / (pdfs.len() as f32);
        MixturePDF{ pdfs, probability }
    }
}

impl PDF for MixturePDF {
    fn value(&self, rng: &mut ThreadRng, hittable_service: &HittableService, direction: &Vector3) -> f32 {
        let mut accumulation: f32 = 0.0;
        for pdf_index in 0..self.pdfs.len() {
            accumulation += self.pdfs[pdf_index].value(rng, hittable_service, direction);
        }
        accumulation * self.probability
    }

    fn generate(&self, rng: &mut ThreadRng, hittable_service: &HittableService) -> Vector3 {
        let random_number = rng.gen::<f32>();
        let quantized_index = (random_number * self.pdfs.len() as f32) as usize;
        self.pdfs[quantized_index].generate(rng, hittable_service)
    }
}