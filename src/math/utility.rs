use std::f32::consts::PI;

use nalgebra::Vector3;
use rand::{rngs::ThreadRng, Rng};
use rand_chacha::ChaChaRng;

#[inline]
pub fn random_range_chacha(rng: &mut ChaChaRng, minimum: f32, maximum: f32) -> Vector3<f32> {
    Vector3::<f32>::new(rng.gen_range(minimum..maximum), rng.gen_range(minimum..maximum), rng.gen_range(minimum..maximum) )
}

#[inline]
pub fn reflect(v: &Vector3<f32>, normal: &Vector3<f32>, reflected_out: &mut Vector3<f32>) -> bool {
    *reflected_out = (*v) - (*normal) * (2.0 * Vector3::<f32>::dot(v, normal));

    true
}

#[inline]
pub fn refract(v: &Vector3<f32>, n: &Vector3<f32>, etai_over_etat: f32, refracted_out: &mut Vector3<f32>) -> bool {
    let negative_uv: Vector3<f32> = -*v;
    let cos_theta = Vector3::<f32>::dot(&negative_uv,&n).min(1.0);
    let ray_out_perp: Vector3<f32> = (*v + (*n) * cos_theta) * etai_over_etat;
    let ray_out_parallel: Vector3<f32> = (-(*n)) * (1.0 - ray_out_perp.magnitude_squared()).abs().sqrt();    
    *refracted_out = ray_out_perp + ray_out_parallel;
    
    true
}

#[inline]
pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vector3<f32> {
    let mut candidate: Vector3<f32> = Vector3::<f32>::zeros();
    loop {
        candidate.x = rng.gen_range(-1.0..1.0);
        candidate.y = rng.gen_range(-1.0..1.0);
        candidate.z = rng.gen_range(-1.0..1.0);

        if candidate.magnitude_squared() < 1.0 {
            return candidate;
        }
    }
}

#[inline]
pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Vector3<f32> {
    let mut candidate: Vector3<f32> = Vector3::<f32>::zeros();
    loop {
        candidate.x = rng.gen_range(-1.0..1.0);
        candidate.y = rng.gen_range(-1.0..1.0);

        if candidate.magnitude_squared() < 1.0 {
            return candidate;
        }
    }
}

#[inline]
pub fn random_cosine_direction(rng: &mut ThreadRng) -> Vector3<f32> {
    let r1: f32 = rng.gen::<f32>();
    let r2: f32 = rng.gen::<f32>();
    let z: f32 = (1.0 - r2).sqrt();

    let phi: f32 = 2.0 * PI * r1;
    let x: f32 = phi.cos() * r2.sqrt();
    let y: f32 = phi.sin() * r2.sqrt();

    Vector3::<f32>::new( x, y, z )
}