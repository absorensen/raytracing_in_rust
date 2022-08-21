use rand::{rngs::ThreadRng};

use crate::core::color_rgb::ColorRGB;
use crate::hittables::hit_record::HitRecord;
use crate::core::ray::Ray;
use crate::services::texture_service::TextureService;

use super::scatter_record::ScatterRecord;



pub trait Material : Sync + Send {
    fn emitted(&self, _texture_service: &TextureService, _ray:&Ray, _hit: &HitRecord, emitted_out: &mut ColorRGB) {
        emitted_out.r = 0.0;
        emitted_out.g = 0.0;
        emitted_out.b = 0.0;
    }

    fn scatter(&self, _rng: &mut ThreadRng, _texture_service: &TextureService, _ray:&Ray, _hit: &HitRecord, _scatter_out: &mut ScatterRecord) -> bool {
        false
    }

    fn scattering_pdf(&self, _rng: &mut ThreadRng, _ray: &Ray, _hit: &HitRecord, _scattered_ray: &Ray) -> f32 {
        0.0
    }
}

