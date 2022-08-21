use nalgebra::Vector3;
use rand::prelude::ThreadRng;

use crate::geometry::aabb::AABB;
use crate::hittables::default_hittable::DefaultHittable;
use crate::hittables::hit_record::HitRecord;
use crate::core::ray::Ray;
use crate::hittables::hittable_enum::HittableEnum;

// Introduce a build step
// All elements are added with add hittable
// Entry point index is cached after build 
pub struct HittableService {
    bvh_root_index: usize,
    lights_root_index: usize,
    hittables: Vec<HittableEnum>,
}

impl HittableService {
    pub fn new() -> HittableService {
        let mut service = HittableService{ bvh_root_index: 0, lights_root_index: 0, hittables : Vec::new()};
        
        service.add_hittable(HittableEnum::DefaultHittable(DefaultHittable{}));

        service
    }

    pub fn add_hittable(&mut self, new_hittable: HittableEnum) -> usize {
        self.hittables.push(new_hittable);

        self.hittables.len() - 1
    }

    pub fn has_lights(&self) -> bool {
        self.lights_root_index != 0 
    }

    pub fn get_bvh_root_index(&self) -> usize {
        self.bvh_root_index
    }

    pub fn set_bvh_root_index(&mut self, index: usize) -> () {
        self.bvh_root_index = index;
    }

    pub fn get_lights_root_index(&self) -> usize {
        self.lights_root_index
    }

    pub fn set_lights_root_index(&mut self, index: usize) -> () {
        self.lights_root_index = index;
    }

    // TODO: How to do this in a nice way with lights, either double up on function calls or use a switch in the arguments

    #[inline] // Recursive to begin with, but make it a loop
    pub fn hit(&self, index: usize, rng: &mut ThreadRng, ray: &Ray, t_min: f32, t_max: f32, hit_out: &mut HitRecord) -> bool {
        self.hittables[index].hit(&self, rng, ray, t_min, t_max, hit_out)
    }

    #[inline] 
    pub fn bounding_box(&self, index:usize, time_0: f32, time_1: f32, box_out: &mut AABB) -> bool {
        self.hittables[index].bounding_box(&self, time_0, time_1, box_out)
    }
    
    #[inline] 
    pub fn pdf_value(&self, index:usize, rng: &mut ThreadRng, origin: &Vector3<f32>, vv: &Vector3<f32>) -> f32 { 
        self.hittables[index].pdf_value(&self, rng, origin, vv)
    }
    
    #[inline] 
    pub fn random(&self, index:usize, rng: &mut ThreadRng, origin: &Vector3<f32>) -> Vector3<f32> {
        self.hittables[index].random(&self, rng, origin)
    }
}
