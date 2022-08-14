use rand::{rngs::ThreadRng};

use crate::{services::hittable_service::HittableService, math::{vector3::Vector3}};

use super::{mixture_pdf::MixturePDF, cosine_pdf::CosinePDF, hittable_pdf::HittablePDF};

pub enum PDFEnum {
    CosinePDF(CosinePDF),
    HittablePDF(HittablePDF),
    MixturePDF(MixturePDF),
}

pub trait PDF: Sync + Send {
    // Maybe convert these to take an output argument
    fn value(&self, rng: &mut ThreadRng, hittable_service: &HittableService, direction: &Vector3) -> f32;
    fn generate(&self, rng: &mut ThreadRng, hittable_service: &HittableService) -> Vector3;
}



