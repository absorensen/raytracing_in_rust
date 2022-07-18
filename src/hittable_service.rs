use rand::prelude::ThreadRng;

use crate::aabb::AABB;
use crate::hittable::{HittableList, XYRect, XZRect, YZRect, BoxHittable, RotateY, Translate, ConstantMedium, FlipFace, HitRecord, DefaultHittable};
use crate::ray::Ray;
use crate::vector3::{Vector3};

pub enum HittableEnum {
    DefaultHittable(DefaultHittable),
    HittableList(HittableList),
    XYRect(XYRect),
    XZRect(XZRect),
    YZRect(YZRect),
    BoxHittable(BoxHittable),
    RotateY(RotateY),
    Translate(Translate),
    ConstantMedium(ConstantMedium),
    FlipFace(FlipFace),
}

pub struct HittableService {
    hittables: Vec<HittableEnum>,
    lights: Vec<usize>, // For lights sampling through LightsPDF
}

impl HittableService {
    pub fn new() -> HittableService {
        let mut service = HittableService{ hittables : Vec::new(), lights: Vec::new() };
        
        service.add_hittable(HittableEnum::DefaultHittable(DefaultHittable{}));

        service
    }

    pub fn add_hittable(&mut self, new_hittable: HittableEnum) -> usize {
        self.hittables.push(new_hittable);

        self.hittables.len() - 1
    }

    // TODO: How to do this in a nice way with lights, either double up on function calls or use a switch in the arguments

    #[inline] // Recursive to begin with, but make it a loop
    fn hit(&self, rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64, hit_out: &mut HitRecord) -> bool {
        // TODO: Implement this
        false
    }

    #[inline] 
    fn bounding_box(&self, time_0: f64, time_1: f64, box_out: &mut AABB) -> bool {
        // TODO: Implement this
        false
    }
    
    #[inline] 
    fn pdf_value(&self, _rng: &mut ThreadRng, _origin: &Vector3,_vv: &Vector3) -> f64 { 
        // TODO: Implement this
        0.0 
    }
    
    #[inline] 
    fn random(&self, _rng: &mut ThreadRng, _origin: &Vector3) -> Vector3 {
        // TODO: Implement this 
        Vector3::new(1.0, 0.0, 0.0) 
    }
}
