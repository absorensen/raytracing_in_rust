use std::{f32::consts::PI};

use rand::{rngs::ThreadRng, Rng};

use crate::{vector3::Vector3, ortho_normal_base::OrthoNormalBase, hittable_service::HittableService};

pub trait PDF: Sync + Send {
    // Maybe convert these to take an output argument
    fn value(&self, rng: &mut ThreadRng, hittable_service: &HittableService, direction: &Vector3) -> f32;
    fn generate(&self, rng: &mut ThreadRng, hittable_service: &HittableService) -> Vector3;
}

pub struct CosinePDF {
    pub uvw: OrthoNormalBase,
}

impl CosinePDF {
    pub fn new(w: &Vector3) -> CosinePDF {
        CosinePDF{ uvw: OrthoNormalBase::build_from_w(w) }
    }
}

impl PDF for CosinePDF {
    fn value(&self, _rng: &mut ThreadRng, _hittable_service: &HittableService, direction: &Vector3) -> f32 {
        let cosine = Vector3::dot(&direction.normalized(), &self.uvw.w);

        if cosine <= 0.0 { 0.0 } else { cosine / PI }    
    }

    fn generate(&self, rng: &mut ThreadRng, _hittable_service: &HittableService) -> Vector3 {
        self.uvw.local_vector(&Vector3::random_cosine_direction(rng))
    }
}

pub struct HittablePDF {
    pub origin: Vector3,
    pub hittable_index: usize,
}

impl HittablePDF {
    pub fn new(origin: &Vector3, p: usize) -> HittablePDF {
        HittablePDF{ origin: origin.clone(), hittable_index: p }
    }
}

impl PDF for HittablePDF {
    fn value(&self, rng: &mut ThreadRng, hittable_service: &HittableService, direction: &Vector3) -> f32 {
        hittable_service.pdf_value(self.hittable_index, rng, &self.origin, direction)
    }

    fn generate(&self, rng: &mut ThreadRng, hittable_service: &HittableService) -> Vector3 {
        hittable_service.random(self.hittable_index, rng, &self.origin)
    }
}

pub struct MixturePDF {
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