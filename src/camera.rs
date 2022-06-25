use std::f64;

use rand::Rng;
use rand::rngs::ThreadRng;

use crate::vector3::{Vector3, Point3};
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    horizontal: Vector3,
    vertical: Vector3,
    lower_left_corner: Point3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(look_from: Point3, look_at: Point3, v_up: Vector3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64, time0: f64, time1: f64) -> Self {
        let theta = f64::consts::PI / 180.0 * vfov;
        let h = f64::tan(theta * 0.5);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalized();
        let u = (Vector3::cross(&v_up, &w)).normalized();
        let v = Vector3::cross(&w, &u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - focus_dist * w;

        let lens_radius = aperture * 0.5;

        Camera{
            origin: origin, 
            horizontal: horizontal, 
            vertical: vertical, 
            lower_left_corner: lower_left_corner, 
            u: u, v:v, w:w, 
            lens_radius:lens_radius, 
            time0:time0, time1:time1
        }
    }

    #[inline]
    pub fn get_ray(&self, rng: &mut ThreadRng, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vector3::random_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;

        Ray{
            origin: self.origin + offset, 
            direction: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset, 
            time:(self.time1 - self.time0) * rng.gen::<f64>() + self.time0
        }
    }
}