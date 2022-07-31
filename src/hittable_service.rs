use rand::prelude::ThreadRng;

use crate::aabb::AABB;
use crate::bvh_node::BVHNode;
use crate::hittable::{HittableList, XYRect, XZRect, YZRect, BoxHittable, RotateY, Translate, ConstantMedium, FlipFace, HitRecord, DefaultHittable, Hittable};
use crate::moving_sphere::MovingSphere;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector3::{Vector3};

pub enum HittableEnum {
    DefaultHittable(DefaultHittable),
    BVHNode(BVHNode),
    Sphere(Sphere),
    MovingSphere(MovingSphere),
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
    pub fn hit(&self, index: usize, rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64, hit_out: &mut HitRecord) -> bool {
        match &self.hittables[index] {
            HittableEnum::DefaultHittable(default) => default.hit(rng, &self, ray, t_min, t_max, hit_out),
            HittableEnum::BVHNode(bvh_node) => bvh_node.hit(rng, &self, ray, t_min, t_max, hit_out),
            HittableEnum::Sphere(sphere) => sphere.hit(rng, &self, ray, t_min, t_max, hit_out),
            HittableEnum::MovingSphere(moving_sphere) => moving_sphere.hit(rng, &self, ray, t_min, t_max, hit_out),
            HittableEnum::HittableList(hittable_list) => hittable_list.hit(rng, &self, ray, t_min, t_max, hit_out),
            HittableEnum::XYRect(xy_rect) => xy_rect.hit(rng, &self, ray, t_min, t_max, hit_out),
            HittableEnum::XZRect(xz_rect) => xz_rect.hit(rng, &self, ray, t_min, t_max, hit_out),
            HittableEnum::YZRect(yz_rect) => yz_rect.hit(rng, &self, ray, t_min, t_max, hit_out),
            HittableEnum::BoxHittable(box_hittable) => box_hittable.hit(rng, &self, ray, t_min, t_max, hit_out),
            HittableEnum::RotateY(rotate_y) => rotate_y.hit(rng, &self, ray, t_min, t_max, hit_out),
            HittableEnum::Translate(translate) => translate.hit(rng, &self, ray, t_min, t_max, hit_out),
            HittableEnum::ConstantMedium(constant_medium) => constant_medium.hit(rng, &self, ray, t_min, t_max, hit_out),
            HittableEnum::FlipFace(flip_face) => flip_face.hit(rng, &self, ray, t_min, t_max, hit_out),
        }
    }

    #[inline] 
    pub fn bounding_box(&self, index:usize, time_0: f64, time_1: f64, box_out: &mut AABB) -> bool {
        match &self.hittables[index] {
            HittableEnum::DefaultHittable(default) => default.bounding_box(&self, time_0, time_1, box_out),
            HittableEnum::BVHNode(bvh_node) => bvh_node.bounding_box(&self, time_0, time_1, box_out),
            HittableEnum::Sphere(sphere) => sphere.bounding_box(&self, time_0, time_1, box_out),
            HittableEnum::MovingSphere(moving_sphere) => moving_sphere.bounding_box(&self, time_0, time_1, box_out),
            HittableEnum::HittableList(hittable_list) => hittable_list.bounding_box(&self, time_0, time_1, box_out),
            HittableEnum::XYRect(xy_rect) => xy_rect.bounding_box(&self, time_0, time_1, box_out),
            HittableEnum::XZRect(xz_rect) => xz_rect.bounding_box(&self, time_0, time_1, box_out),
            HittableEnum::YZRect(yz_rect) => yz_rect.bounding_box(&self, time_0, time_1, box_out),
            HittableEnum::BoxHittable(box_hittable) => box_hittable.bounding_box(&self, time_0, time_1, box_out),
            HittableEnum::RotateY(rotate_y) => rotate_y.bounding_box(&self, time_0, time_1, box_out),
            HittableEnum::Translate(translate) => translate.bounding_box(&self, time_0, time_1, box_out),
            HittableEnum::ConstantMedium(constant_medium) => constant_medium.bounding_box(&self, time_0, time_1, box_out),
            HittableEnum::FlipFace(flip_face) => flip_face.bounding_box(&self, time_0, time_1, box_out),
        }
    }
    
    #[inline] 
    pub fn pdf_value(&self, index:usize, rng: &mut ThreadRng, origin: &Vector3, vv: &Vector3) -> f64 { 
        match &self.hittables[index] {
            HittableEnum::DefaultHittable(default) => default.pdf_value(rng, &self, origin, vv),
            HittableEnum::BVHNode(bvh_node) => bvh_node.pdf_value(rng, &self, origin, vv),
            HittableEnum::Sphere(sphere) => sphere.pdf_value(rng, &self, origin, vv),
            HittableEnum::MovingSphere(moving_sphere) => moving_sphere.pdf_value(rng, &self, origin, vv),
            HittableEnum::HittableList(hittable_list) => hittable_list.pdf_value(rng, &self, origin, vv),
            HittableEnum::XYRect(xy_rect) => xy_rect.pdf_value(rng, &self, origin, vv),
            HittableEnum::XZRect(xz_rect) => xz_rect.pdf_value(rng, &self, origin, vv),
            HittableEnum::YZRect(yz_rect) => yz_rect.pdf_value(rng, &self, origin, vv),
            HittableEnum::BoxHittable(box_hittable) => box_hittable.pdf_value(rng, &self, origin, vv),
            HittableEnum::RotateY(rotate_y) => rotate_y.pdf_value(rng, &self, origin, vv),
            HittableEnum::Translate(translate) => translate.pdf_value(rng, &self, origin, vv),
            HittableEnum::ConstantMedium(constant_medium) => constant_medium.pdf_value(rng, &self, origin, vv),
            HittableEnum::FlipFace(flip_face) => flip_face.pdf_value(rng, &self, origin, vv),
        }
    }
    
    #[inline] 
    pub fn random(&self, index:usize, rng: &mut ThreadRng, origin: &Vector3) -> Vector3 {
        match &self.hittables[index] {
            HittableEnum::DefaultHittable(default) => default.random(rng, &self, origin),
            HittableEnum::BVHNode(bvh_node) => bvh_node.random(rng, &self, origin),
            HittableEnum::Sphere(sphere) => sphere.random(rng, &self, origin),
            HittableEnum::MovingSphere(moving_sphere) => moving_sphere.random(rng, &self, origin),
            HittableEnum::HittableList(hittable_list) => hittable_list.random(rng, &self, origin),
            HittableEnum::XYRect(xy_rect) => xy_rect.random(rng, &self, origin),
            HittableEnum::XZRect(xz_rect) => xz_rect.random(rng, &self, origin),
            HittableEnum::YZRect(yz_rect) => yz_rect.random(rng, &self, origin),
            HittableEnum::BoxHittable(box_hittable) => box_hittable.random(rng, &self, origin),
            HittableEnum::RotateY(rotate_y) => rotate_y.random(rng, &self, origin),
            HittableEnum::Translate(translate) => translate.random(rng, &self, origin),
            HittableEnum::ConstantMedium(constant_medium) => constant_medium.random(rng, &self, origin),
            HittableEnum::FlipFace(flip_face) => flip_face.random(rng, &self, origin),
        }
    }
}
