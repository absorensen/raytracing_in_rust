use std::f32;

use nalgebra::Vector3;
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::core::ray::Ray;
use crate::math::utility::random_in_unit_disk;

pub struct Camera {
    origin: Vector3::<f32>,
    horizontal: Vector3::<f32>,
    vertical: Vector3::<f32>,
    lower_left_corner: Vector3::<f32>,
    u: Vector3::<f32>,
    v: Vector3::<f32>,
    _w: Vector3::<f32>,
    lens_radius: f32,
    time_0: f32,
    time_1: f32,
}

impl Camera {
    pub fn new(look_from: Vector3::<f32>, look_at: Vector3::<f32>, v_up: Vector3::<f32>, vfov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32, time_0: f32, time_1: f32) -> Self {
        let theta: f32 = f32::consts::PI / 180.0 * vfov;
        let h: f32 = f32::tan(theta * 0.5);
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let w: Vector3<f32> = (look_from - look_at).normalize();
        let u: Vector3<f32> = Vector3::<f32>::cross(&v_up, &w).normalize();
        let v: Vector3<f32> = Vector3::<f32>::cross(&w, &u).normalize();

        let origin: Vector3<f32> = look_from;
        let horizontal: Vector3<f32> = u * focus_dist * viewport_width;
        let vertical: Vector3<f32> = v * focus_dist * viewport_height;
        let lower_left_corner: Vector3<f32> = origin - horizontal * 0.5 - vertical * 0.5 - w * focus_dist;

        let lens_radius: f32 = aperture * 0.5;

        Camera{
            origin, 
            horizontal, 
            vertical, 
            lower_left_corner, 
            u, v, _w:w, 
            lens_radius, 
            time_0, time_1
        }
    }

    #[inline]
    pub fn get_ray(&self, rng: &mut ThreadRng, s: f32, t: f32) -> Ray {
        let rd: Vector3<f32> = random_in_unit_disk(rng) * self.lens_radius;
        let offset: Vector3<f32> = self.u * rd.x + self.v * rd.y;

        Ray{
            origin: self.origin + offset, 
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset, 
            time:(self.time_1 - self.time_0) * rng.gen::<f32>() + self.time_0
        }
    }

    #[inline]
    pub fn get_start_time(&self) -> f32{
        self.time_0
    }

    #[inline]
    pub fn get_end_time(&self) -> f32{
        self.time_1
    }
}