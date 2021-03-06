use std::{f64::consts::PI, sync::Arc};

use rand::{rngs::ThreadRng, Rng};

use crate::{vector3::Vector3, ortho_normal_base::OrthoNormalBase, hittable::{Hittable}};

pub trait PDF: Sync + Send {
    // Maybe convert these to take an output argument
    fn value(&self, rng: &mut ThreadRng, direction: &Vector3) -> f64;
    fn generate(&self, rng: &mut ThreadRng) -> Vector3;
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
    fn value(&self, _rng: &mut ThreadRng, direction: &Vector3) -> f64 {
        let cosine = Vector3::dot(&direction.normalized(), &self.uvw.w);

        if cosine <= 0.0 { 0.0 } else { cosine / PI }    
    }

    fn generate(&self, rng: &mut ThreadRng) -> Vector3 {
        self.uvw.local_vector(&Vector3::random_cosine_direction(rng))
    }
}

pub struct HittablePDF {
    pub origin: Vector3,
    pub hittable: Arc<dyn Hittable>,
}

impl HittablePDF {
    pub fn new(p: &Arc<dyn Hittable>, origin: &Vector3) -> HittablePDF {
        HittablePDF{ origin: origin.clone(), hittable: Arc::clone(p) }
    }
}

impl PDF for HittablePDF {
    fn value(&self, rng: &mut ThreadRng, direction: &Vector3) -> f64 {
        self.hittable.pdf_value(rng, &self.origin, direction)
    }

    fn generate(&self, rng: &mut ThreadRng) -> Vector3 {
        self.hittable.random(rng, &self.origin)
    }
}

pub struct MixturePDF {
    pub pdfs: [Box<dyn PDF>; 2],
}

impl MixturePDF {
    pub fn new(p0: Box<dyn PDF>, p1: Box<dyn PDF>) -> MixturePDF {
        MixturePDF{ pdfs: [p0, p1] }
    }
}

impl PDF for MixturePDF {
    fn value(&self, rng: &mut ThreadRng, direction: &Vector3) -> f64 {
        0.5 * self.pdfs[0].value(rng, direction) + 0.5 * self.pdfs[1].value(rng, direction)
    }

    fn generate(&self, rng: &mut ThreadRng) -> Vector3 {
        if rng.gen::<f64>() < 0.5 {
            self.pdfs[0].generate(rng)
        } else {
            self.pdfs[1].generate(rng)
        }
    }
}