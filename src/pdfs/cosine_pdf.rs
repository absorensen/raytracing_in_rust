use std::f32::consts::PI;

use ultraviolet::Vec3;
use rand::rngs::ThreadRng;

use crate::{math::{ortho_normal_base::OrthoNormalBase, utility::random_cosine_direction}, services::hittable_service::HittableService};

use super::pdf::PDF;

#[derive(Clone, Copy)]
pub struct CosinePDF {
    pub uvw: OrthoNormalBase,
}

impl CosinePDF {
    pub fn new(w: &Vec3) -> CosinePDF {
        CosinePDF{ uvw: OrthoNormalBase::build_from_w(w) }
    }

    pub fn update(&mut self, n: Vec3) {
        self.uvw.update(n);
    }
}

impl PDF for CosinePDF {
    fn value(&self, _rng: &mut ThreadRng, _hittable_service: &HittableService, direction: &Vec3) -> f32 {
        let cosine: f32 = direction.normalized().dot(self.uvw.w);

        if cosine <= 0.0 { 0.0 } else { cosine / PI }    
    }

    fn generate(&self, rng: &mut ThreadRng, _hittable_service: &HittableService) -> Vec3 {
        self.uvw.local_vector(&random_cosine_direction(rng))
    }
}
