use nalgebra::Vector3;
use rand::rngs::ThreadRng;

use crate::{services::hittable_service::HittableService};

use super::{cosine_pdf::CosinePDF, pdf::PDF, hittable_pdf::HittablePDF};

// Mixture PDF is not in here because it creates a potentially infinite size
// Also a mixture PDF should at this point only every hold 2 PDFs
#[derive(Clone, Copy)]
pub enum PDFEnum {
    None(),
    CosinePDF(CosinePDF),
    HittablePDF(HittablePDF),
}

impl PDF for PDFEnum {

    #[inline]
    fn value(&self, rng: &mut ThreadRng, hittable_service: &HittableService, direction: &Vector3::<f32>) -> f32 {
        match self {
            PDFEnum::None() => 0.0,
            PDFEnum::CosinePDF (cosine_pdf ) => cosine_pdf.value(rng, hittable_service, direction),
            PDFEnum::HittablePDF(hittable_pdf) => hittable_pdf.value(rng, hittable_service, direction),
        }
    }

    #[inline]
    fn generate(&self, rng: &mut ThreadRng, hittable_service: &HittableService) -> Vector3::<f32> {
        match self {
            PDFEnum::None() => Vector3::<f32>::zeros(),
            PDFEnum::CosinePDF (cosine_pdf ) => cosine_pdf.generate(rng, hittable_service),
            PDFEnum::HittablePDF(hittable_pdf) => hittable_pdf.generate(rng, hittable_service),
        }
    }    
}