use rand::{rngs::ThreadRng};

use crate::{services::hittable_service::HittableService, math::{vector3::Vector3}};

use super::{cosine_pdf::CosinePDF, hittable_pdf::HittablePDF};

// Mixture PDF is not in here because it creates a potentially infinite size
// Also a mixture PDF should at this point only every hold 2 PDFs
pub enum PDFEnum {
    None(),
    CosinePDF(CosinePDF),
    HittablePDF(HittablePDF),
}

impl PDF for PDFEnum {
    fn value(&self, rng: &mut ThreadRng, hittable_service: &HittableService, direction: &Vector3) -> f32 {
        match self {
            PDFEnum::None() => 0.0,
            PDFEnum::CosinePDF (cosine_pdf ) => cosine_pdf.value(rng, hittable_service, direction),
            PDFEnum::HittablePDF(hittable_pdf) => hittable_pdf.value(rng, hittable_service, direction),
        }
    }

    fn generate(&self, rng: &mut ThreadRng, hittable_service: &HittableService) -> Vector3 {
        match self {
            PDFEnum::None() => Vector3::zero(),
            PDFEnum::CosinePDF (cosine_pdf ) => cosine_pdf.generate(rng, hittable_service),
            PDFEnum::HittablePDF(hittable_pdf) => hittable_pdf.generate(rng, hittable_service),
        }
    }    
}

pub trait PDF: Sync + Send {
    // Maybe convert these to take an output argument
    fn value(&self, rng: &mut ThreadRng, hittable_service: &HittableService, direction: &Vector3) -> f32;
    fn generate(&self, rng: &mut ThreadRng, hittable_service: &HittableService) -> Vector3;
}



