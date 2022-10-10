use rand::rngs::ThreadRng;
use rand_chacha::ChaCha20Rng;
use ultraviolet::Vec3;

use crate::{services::hittable_service::HittableService, core::ray::Ray, geometry::aabb::AABB};

use super::{xy_rect::XYRect, xz_rect::XZRect, yz_rect::YZRect, bvh_node::BVHNode, hittable::Hittable, hit_record::HitRecord, hittable_enum::HittableEnum};

// Change this to BVH
pub struct BoxHittable {
    sides_index: usize,
    box_min: Vec3,
    box_max: Vec3,
}

impl BoxHittable {
    pub fn new(rng: &mut ChaCha20Rng, hittable_service: &mut HittableService, point_0: Vec3, point_1: Vec3, material: usize) -> BoxHittable {
        let mut sides : Vec<usize> = Vec::new();

        let hittable_index = hittable_service.add_hittable(HittableEnum::XYRect(XYRect::new(point_0.x, point_1.x, point_0.y, point_1.y, point_1.z, material)));
        sides.push(hittable_index);

        let hittable_index = hittable_service.add_hittable(HittableEnum::XYRect(XYRect::new(point_0.x, point_1.x, point_0.y, point_1.y, point_0.z, material)));
        sides.push(hittable_index);


        let hittable_index = hittable_service.add_hittable(HittableEnum::XZRect(XZRect::new(point_0.x, point_1.x, point_0.z, point_1.z, point_1.y, material)));
        sides.push(hittable_index);
        
        let hittable_index = hittable_service.add_hittable(HittableEnum::XZRect(XZRect::new(point_0.x, point_1.x, point_0.z, point_1.z, point_0.y, material)));
        sides.push(hittable_index);

        
        let hittable_index = hittable_service.add_hittable(HittableEnum::YZRect(YZRect::new(point_0.y, point_1.y, point_0.z, point_1.z, point_1.x, material)));
        sides.push(hittable_index);

        let hittable_index = hittable_service.add_hittable(HittableEnum::YZRect(YZRect::new(point_0.y, point_1.y, point_0.z, point_1.z, point_0.x, material)));
        sides.push(hittable_index);

        let root_node = HittableEnum::BVHNode(BVHNode::from_index_list(rng, hittable_service, &mut sides, 0.0, 1.0));
        let root_node_index = hittable_service.add_hittable(root_node);

        BoxHittable { box_min: point_0, box_max: point_1, sides_index: root_node_index }
    }

}

impl Hittable for BoxHittable {
    fn hit(&self, rng: &mut ThreadRng, hittable_service: &HittableService, ray: &Ray, t_min: f32, t_max: f32, hit_out: &mut HitRecord) -> bool {
        hittable_service.hit(self.sides_index, rng, ray, t_min, t_max, hit_out)
    }

    fn bounding_box(&self, _hittable_service: &HittableService, _time_0: f32, _time_1: f32, box_out: &mut AABB) -> bool {
        box_out.minimum.x = self.box_min.x;
        box_out.minimum.y = self.box_min.y;
        box_out.minimum.z = self.box_min.z;

        box_out.maximum.x = self.box_max.x;
        box_out.maximum.y = self.box_max.y;
        box_out.maximum.z = self.box_max.z;

        true
    }

}