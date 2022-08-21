use nalgebra::Vector3;
use rand::rngs::ThreadRng;

use crate::geometry::aabb::AABB;
use crate::core::ray::Ray;
use crate::services::hittable_service::{HittableService};

use super::hit_record::HitRecord;

pub trait Hittable: Sync + Send {
    // Maybe convert these to take an output argument
    fn hit(&self, rng: &mut ThreadRng, _hittable_service: &HittableService,  ray: &Ray, t_min: f32, t_max: f32, hit_out: &mut HitRecord) -> bool;
    fn bounding_box(&self, _hittable_service: &HittableService, time_0: f32, time_1: f32, box_out: &mut AABB) -> bool;
    fn pdf_value(&self, _rng: &mut ThreadRng, _hittable_service: &HittableService, _origin: &Vector3<f32>,_vv: &Vector3<f32>) -> f32 { 0.0 }
    fn random(&self, _rng: &mut ThreadRng, _hittable_service: &HittableService, _origin: &Vector3<f32>) -> Vector3<f32> { Vector3::<f32>::new(1.0, 0.0, 0.0) }
}
