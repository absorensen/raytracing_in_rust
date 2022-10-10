use ultraviolet::Vec3;
use rand::rngs::ThreadRng;

use crate::{services::hittable_service::HittableService, core::ray::Ray, geometry::aabb::AABB};

use super::{default_hittable::DefaultHittable, bvh_node::BVHNode, sphere::Sphere, moving_sphere::MovingSphere, hittable_list::HittableList, xy_rect::XYRect, xz_rect::XZRect, yz_rect::YZRect, box_hittable::BoxHittable, rotate_y::RotateY, translate::Translate, constant_medium::ConstantMedium, flip_face::FlipFace, hittable::Hittable, hit_record::HitRecord};

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

impl HittableEnum {
    
    #[inline]
    pub fn hit(&self, hittable_service: &HittableService, rng: &mut ThreadRng, ray: &Ray, t_min: f32, t_max: f32, hit_out: &mut HitRecord) -> bool {
        match self {
            HittableEnum::DefaultHittable(default) => default.hit(rng, hittable_service, ray, t_min, t_max, hit_out),
            HittableEnum::BVHNode(bvh_node) => bvh_node.hit(rng, hittable_service, ray, t_min, t_max, hit_out),
            HittableEnum::Sphere(sphere) => sphere.hit(rng, hittable_service, ray, t_min, t_max, hit_out),
            HittableEnum::MovingSphere(moving_sphere) => moving_sphere.hit(rng, hittable_service, ray, t_min, t_max, hit_out),
            HittableEnum::HittableList(hittable_list) => hittable_list.hit(rng, hittable_service, ray, t_min, t_max, hit_out),
            HittableEnum::XYRect(xy_rect) => xy_rect.hit(rng, hittable_service, ray, t_min, t_max, hit_out),
            HittableEnum::XZRect(xz_rect) => xz_rect.hit(rng, hittable_service, ray, t_min, t_max, hit_out),
            HittableEnum::YZRect(yz_rect) => yz_rect.hit(rng, hittable_service, ray, t_min, t_max, hit_out),
            HittableEnum::BoxHittable(box_hittable) => box_hittable.hit(rng, hittable_service, ray, t_min, t_max, hit_out),
            HittableEnum::RotateY(rotate_y) => rotate_y.hit(rng, hittable_service, ray, t_min, t_max, hit_out),
            HittableEnum::Translate(translate) => translate.hit(rng, hittable_service, ray, t_min, t_max, hit_out),
            HittableEnum::ConstantMedium(constant_medium) => constant_medium.hit(rng, hittable_service, ray, t_min, t_max, hit_out),
            HittableEnum::FlipFace(flip_face) => flip_face.hit(rng, hittable_service, ray, t_min, t_max, hit_out),
        }
    }

    #[inline] 
    pub fn bounding_box(&self, hittable_service: &HittableService, time_0: f32, time_1: f32, box_out: &mut AABB) -> bool {
        match self {
            HittableEnum::DefaultHittable(default) => default.bounding_box(hittable_service, time_0, time_1, box_out),
            HittableEnum::BVHNode(bvh_node) => bvh_node.bounding_box(hittable_service, time_0, time_1, box_out),
            HittableEnum::Sphere(sphere) => sphere.bounding_box(hittable_service, time_0, time_1, box_out),
            HittableEnum::MovingSphere(moving_sphere) => moving_sphere.bounding_box(hittable_service, time_0, time_1, box_out),
            HittableEnum::HittableList(hittable_list) => hittable_list.bounding_box(hittable_service, time_0, time_1, box_out),
            HittableEnum::XYRect(xy_rect) => xy_rect.bounding_box(hittable_service, time_0, time_1, box_out),
            HittableEnum::XZRect(xz_rect) => xz_rect.bounding_box(hittable_service, time_0, time_1, box_out),
            HittableEnum::YZRect(yz_rect) => yz_rect.bounding_box(hittable_service, time_0, time_1, box_out),
            HittableEnum::BoxHittable(box_hittable) => box_hittable.bounding_box(hittable_service, time_0, time_1, box_out),
            HittableEnum::RotateY(rotate_y) => rotate_y.bounding_box(hittable_service, time_0, time_1, box_out),
            HittableEnum::Translate(translate) => translate.bounding_box(hittable_service, time_0, time_1, box_out),
            HittableEnum::ConstantMedium(constant_medium) => constant_medium.bounding_box(hittable_service, time_0, time_1, box_out),
            HittableEnum::FlipFace(flip_face) => flip_face.bounding_box(hittable_service, time_0, time_1, box_out),
        }
    }
    
    #[inline] 
    pub fn pdf_value(&self, hittable_service: &HittableService, rng: &mut ThreadRng, origin: &Vec3, vv: &Vec3) -> f32 { 
        match self {
            HittableEnum::DefaultHittable(default) => default.pdf_value(rng, hittable_service, origin, vv),
            HittableEnum::BVHNode(bvh_node) => bvh_node.pdf_value(rng, hittable_service, origin, vv),
            HittableEnum::Sphere(sphere) => sphere.pdf_value(rng, hittable_service, origin, vv),
            HittableEnum::MovingSphere(moving_sphere) => moving_sphere.pdf_value(rng, hittable_service, origin, vv),
            HittableEnum::HittableList(hittable_list) => hittable_list.pdf_value(rng, hittable_service, origin, vv),
            HittableEnum::XYRect(xy_rect) => xy_rect.pdf_value(rng, hittable_service, origin, vv),
            HittableEnum::XZRect(xz_rect) => xz_rect.pdf_value(rng, hittable_service, origin, vv),
            HittableEnum::YZRect(yz_rect) => yz_rect.pdf_value(rng, hittable_service, origin, vv),
            HittableEnum::BoxHittable(box_hittable) => box_hittable.pdf_value(rng, hittable_service, origin, vv),
            HittableEnum::RotateY(rotate_y) => rotate_y.pdf_value(rng, hittable_service, origin, vv),
            HittableEnum::Translate(translate) => translate.pdf_value(rng, hittable_service, origin, vv),
            HittableEnum::ConstantMedium(constant_medium) => constant_medium.pdf_value(rng, hittable_service, origin, vv),
            HittableEnum::FlipFace(flip_face) => flip_face.pdf_value(rng, hittable_service, origin, vv),
        }
    }
    
    #[inline] 
    pub fn random(&self, hittable_service: &HittableService, rng: &mut ThreadRng, origin: &Vec3) -> Vec3 {
        match &self {
            HittableEnum::DefaultHittable(default) => default.random(rng, hittable_service, origin),
            HittableEnum::BVHNode(bvh_node) => bvh_node.random(rng, hittable_service, origin),
            HittableEnum::Sphere(sphere) => sphere.random(rng, hittable_service, origin),
            HittableEnum::MovingSphere(moving_sphere) => moving_sphere.random(rng, hittable_service, origin),
            HittableEnum::HittableList(hittable_list) => hittable_list.random(rng, hittable_service, origin),
            HittableEnum::XYRect(xy_rect) => xy_rect.random(rng, hittable_service, origin),
            HittableEnum::XZRect(xz_rect) => xz_rect.random(rng, hittable_service, origin),
            HittableEnum::YZRect(yz_rect) => yz_rect.random(rng, hittable_service, origin),
            HittableEnum::BoxHittable(box_hittable) => box_hittable.random(rng, hittable_service, origin),
            HittableEnum::RotateY(rotate_y) => rotate_y.random(rng, hittable_service, origin),
            HittableEnum::Translate(translate) => translate.random(rng, hittable_service, origin),
            HittableEnum::ConstantMedium(constant_medium) => constant_medium.random(rng, hittable_service, origin),
            HittableEnum::FlipFace(flip_face) => flip_face.random(rng, hittable_service, origin),
        }
    }

}