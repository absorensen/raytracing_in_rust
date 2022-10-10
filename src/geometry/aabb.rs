use crate::{ core::ray::Ray};
use ultraviolet::Vec3;

#[derive(Default, Copy, Clone, PartialEq)]
pub struct AABB {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl AABB {
    pub fn expand_by_point(&mut self, expansion_point: &Vec3) -> () {
        self.minimum.x = if self.minimum.x < expansion_point.x { self.minimum.x } else { expansion_point.x };
        self.minimum.y = if self.minimum.y < expansion_point.y { self.minimum.y } else { expansion_point.y };
        self.minimum.z = if self.minimum.z < expansion_point.z { self.minimum.z } else { expansion_point.z };

        self.maximum.x = if expansion_point.x < self.maximum.x { self.maximum.x } else { expansion_point.x };
        self.maximum.y = if expansion_point.y < self.maximum.y { self.maximum.y } else { expansion_point.y };
        self.maximum.z = if expansion_point.z < self.maximum.z { self.maximum.z } else { expansion_point.z };
    }

    pub fn expand_by_box(&mut self, expansion_box: &AABB) -> () {
        self.expand_by_point(&expansion_box.minimum);
        self.expand_by_point(&expansion_box.maximum);
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {

        // X
        let inverse_d : f32 = 1.0 / ray.direction.x;
        let mut t_0 : f32 = (self.minimum.x - ray.origin.x) * inverse_d; 
        let mut t_1 : f32 = (self.maximum.x - ray.origin.x) * inverse_d;

        if inverse_d < 0.0 {
            std::mem::swap(&mut t_0, &mut t_1);
        }

        let t_min : f32 = if t_0 > t_min { t_0 } else { t_min };
        let t_max : f32 = if t_1 < t_max { t_1 } else { t_max };
                        
        if t_max <= t_min { return false; }



        // Y
        let inverse_d : f32 = 1.0 / ray.direction.y;
        let mut t_0 : f32 = (self.minimum.y - ray.origin.y) * inverse_d; 
        let mut t_1 : f32 = (self.maximum.y - ray.origin.y) * inverse_d;

        if inverse_d < 0.0 {
            std::mem::swap(&mut t_0, &mut t_1);
        }

        let t_min : f32 = if t_0 > t_min { t_0 } else { t_min };
        let t_max : f32 = if t_1 < t_max { t_1 } else { t_max };
                        
        if t_max <= t_min { return false; }



        // Z
        let inverse_d : f32 = 1.0 / ray.direction.z;
        let mut t_0 : f32 = (self.minimum.z - ray.origin.z) * inverse_d; 
        let mut t_1 : f32 = (self.maximum.z - ray.origin.z) * inverse_d;

        if inverse_d < 0.0 {
            std::mem::swap(&mut t_0, &mut t_1);
        }

        let t_min : f32 = if t_0 > t_min { t_0 } else { t_min };
        let t_max : f32 = if t_1 < t_max { t_1 } else { t_max };
                        
        if t_max <= t_min { return false; }



        true
    }
}