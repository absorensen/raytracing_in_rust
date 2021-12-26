use crate::vector3::{Vector3, Point3};
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    horizontal: Vector3,
    vertical: Vector3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3{x: 0.0, y: 0.0, z: 0.0};
        let horizontal = Vector3{x: viewport_width, y: 0.0, z: 0.0};
        let vertical = Vector3{x: 0.0, y: viewport_height, z: 0.0};
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - Vector3{x: 0.0, y: 0.0, z: focal_length};

        Camera{origin: origin, horizontal: horizontal, vertical: vertical, lower_left_corner: lower_left_corner}
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray{origin: self.origin, direction: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin}
    }
}