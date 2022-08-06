use rand::{rngs::ThreadRng};

use crate::hittables::hit_record::HitRecord;
use crate::math::vector3::{Color, Vector3};
use crate::ray::Ray;
use crate::services::texture_service::TextureService;

use super::scatter_record::ScatterRecord;



pub trait Material : Sync + Send {
    fn emitted(&self, _texture_service: &TextureService, _ray:&Ray, _hit: &HitRecord, _u: f32, _v: f32, _point: &Vector3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scatter(&self, _rng: &mut ThreadRng, _texture_service: &TextureService, _ray:&Ray, _hit: &HitRecord, _scatter_out: &mut ScatterRecord) -> bool {
        false
    }

    fn scattering_pdf(&self, _rng: &mut ThreadRng, _ray: &Ray, _hit: &HitRecord, _scattered_ray: &Ray) -> f32 {
        0.0
    }
}

