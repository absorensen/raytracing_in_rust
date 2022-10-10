use std::f32::consts::PI;

use ultraviolet::Vec3;
use rand::{rngs::ThreadRng, Rng};
use rand_chacha::ChaChaRng;

#[inline]
pub fn random_range_chacha(rng: &mut ChaChaRng, minimum: f32, maximum: f32) -> Vec3 {
    Vec3::new(rng.gen_range(minimum..maximum), rng.gen_range(minimum..maximum), rng.gen_range(minimum..maximum) )
}

#[inline]
pub fn reflect(v: &Vec3, normal: &Vec3, reflected_out: &mut Vec3) -> bool {
    *reflected_out = (*v) - (*normal) * (2.0 * v.dot(*normal));

    true
}

#[inline]
pub fn refract(v: &Vec3, n: &Vec3, etai_over_etat: f32, refracted_out: &mut Vec3) -> bool {
    let negative_uv: Vec3 = -*v;
    let cos_theta = negative_uv.dot(*n).min(1.0);
    let ray_out_perp: Vec3 = (*v + (*n) * cos_theta) * etai_over_etat;
    let ray_out_parallel: Vec3 = (-(*n)) * (1.0 - ray_out_perp.mag_sq()).abs().sqrt();    
    *refracted_out = ray_out_perp + ray_out_parallel;
    
    true
}

#[inline]
pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    let mut candidate: Vec3 = Vec3::zero();
    loop {
        candidate.x = rng.gen_range(-1.0..1.0);
        candidate.y = rng.gen_range(-1.0..1.0);
        candidate.z = rng.gen_range(-1.0..1.0);

        if candidate.mag_sq() < 1.0 {
            return candidate;
        }
    }
}

#[inline]
pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Vec3 {
    let mut candidate: Vec3 = Vec3::zero();
    loop {
        candidate.x = rng.gen_range(-1.0..1.0);
        candidate.y = rng.gen_range(-1.0..1.0);

        if candidate.mag_sq() < 1.0 {
            return candidate;
        }
    }
}

#[inline]
pub fn random_cosine_direction(rng: &mut ThreadRng) -> Vec3 {
    let r1: f32 = rng.gen::<f32>();
    let r2: f32 = rng.gen::<f32>();
    let z: f32 = (1.0 - r2).sqrt();

    let phi: f32 = 2.0 * PI * r1;
    let x: f32 = phi.cos() * r2.sqrt();
    let y: f32 = phi.sin() * r2.sqrt();

    Vec3::new( x, y, z )
}