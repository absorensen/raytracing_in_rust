use std::f32::consts::PI;

use rand::rngs::ThreadRng;

use crate::{math::{ortho_normal_base::OrthoNormalBase, vector3::Vector3}, services::hittable_service::HittableService};

use super::pdf::PDF;

#[derive(Clone, Copy)]
pub struct CosinePDF {
    pub uvw: OrthoNormalBase,
}

impl CosinePDF {
    pub fn new(w: &Vector3) -> CosinePDF {
        CosinePDF{ uvw: OrthoNormalBase::build_from_w(w) }
    }

    pub fn update(&mut self, n: Vector3) {
        self.uvw.update(n);
    }
}

impl PDF for CosinePDF {
    fn value(&self, _rng: &mut ThreadRng, _hittable_service: &HittableService, direction: &Vector3) -> f32 {
        let cosine = Vector3::dot(&direction.get_normalized(), &self.uvw.w);

        if cosine <= 0.0 { 0.0 } else { cosine / PI }    
    }

    fn generate(&self, rng: &mut ThreadRng, _hittable_service: &HittableService) -> Vector3 {
        self.uvw.local_vector(&Vector3::random_cosine_direction(rng))
    }
}
