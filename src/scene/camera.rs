use std::f32;

use rand::Rng;
use rand::rngs::ThreadRng;

use crate::math::vector3::{Point3, Vector3};
use crate::core::ray::Ray;

pub struct Camera {
    origin: Point3,
    horizontal: Vector3,
    vertical: Vector3,
    lower_left_corner: Point3,
    u: Vector3,
    v: Vector3,
    _w: Vector3,
    lens_radius: f32,
    time_0: f32,
    time_1: f32,
}

impl Camera {
    pub fn new(look_from: Point3, look_at: Point3, v_up: Vector3, vfov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32, time_0: f32, time_1: f32) -> Self {
        let theta = f32::consts::PI / 180.0 * vfov;
        let h = f32::tan(theta * 0.5);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalized();
        let u = (Vector3::cross(&v_up, &w)).normalized();
        let v = Vector3::cross(&w, &u);

        let origin = look_from;
        let horizontal = u * focus_dist * viewport_width;
        let vertical = v * focus_dist * viewport_height;
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - w * focus_dist;

        let lens_radius = aperture * 0.5;

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
        let rd = Vector3::random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

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