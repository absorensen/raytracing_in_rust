use rand::{rngs::ThreadRng, Rng};

use crate::{services::hittable_service::HittableService, math::vector3::Vector3};

use super::pdf::{PDF, PDFEnum};

const PDF_COUNT: usize = 2;

pub struct MixturePDF {
    // Get these removed into an array of pdf enums
    pdfs: [PDFEnum; PDF_COUNT],
    probability: f32,
}

impl MixturePDF {
    pub fn new(pdf_a: PDFEnum, pdf_b: PDFEnum) -> MixturePDF {
        let probability = 1.0 / (PDF_COUNT as f32); // One over number of PDFs
        MixturePDF{ pdfs: [pdf_a, pdf_b], probability }
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