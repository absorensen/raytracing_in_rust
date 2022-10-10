use ultraviolet::Vec3;
use rand::rngs::ThreadRng;

use crate::{services::hittable_service::HittableService, geometry::aabb::AABB, core::ray::Ray};

use super::{hittable::Hittable, hit_record::HitRecord};

pub struct DefaultHittable {
}

impl Hittable for DefaultHittable {
    fn hit(&self, _rng: &mut ThreadRng, _hittable_service: &HittableService, _ray: &Ray, _t_min: f32, _t_max: f32, _hit_out: &mut HitRecord) -> bool { false }
    fn bounding_box(&self, _hittable_service: &HittableService, _time_0: f32, _time_1: f32, _box_out: &mut AABB) -> bool { false }
    fn pdf_value(&self, _rng: &mut ThreadRng, _hittable_service: &HittableService, _origin: &Vec3,_vv: &Vec3) -> f32 { 0.0 }
    fn random(&self, _rng: &mut ThreadRng, _hittable_service: &HittableService, _origin: &Vec3) -> Vec3 { Vec3::new(1.0, 0.0, 0.0) }
}