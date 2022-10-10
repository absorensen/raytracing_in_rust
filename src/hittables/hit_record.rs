use ultraviolet::Vec3;

use crate::{core::ray::Ray};

pub struct HitRecord {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub position: Vec3,
    pub normal: Vec3,
    pub is_front_face: bool,
    pub material: usize,
}

impl HitRecord{
    pub fn default() -> Self {
        HitRecord { t: 0.0, u: 0.0, v: 0.0, position: Vec3::zero(), normal: Vec3::zero(), is_front_face: false, material: 0 }
    }

    pub fn new(
        ray: &Ray,
        t: f32,
        u: f32,
        v: f32,
        position: &Vec3,
        normal: &Vec3,
        material: usize
    ) -> Self {
        let mut result = HitRecord{ t, u, v, position: position.clone(), normal: normal.clone(), is_front_face: false, material };
        result.set_face_normal(ray, normal);
        result
    }
    
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.is_front_face = ray.direction.dot(*outward_normal) < 0.0;

        // Convert this to a multiplication of -1 or 1
        if self.is_front_face {
            self.normal.x = outward_normal.x;
            self.normal.y = outward_normal.y;
            self.normal.z = outward_normal.z;
        } else {
            self.normal.x = -outward_normal.x;
            self.normal.y = -outward_normal.y;
            self.normal.z = -outward_normal.z;
        }
    }
}