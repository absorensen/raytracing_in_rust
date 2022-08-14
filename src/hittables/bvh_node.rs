use core::panic;
use std::{cmp::Ordering};
use rand::{Rng, rngs::ThreadRng};
use rand_chacha::ChaCha20Rng;

use crate::{services::hittable_service::HittableService, geometry::aabb::AABB, core::ray::Ray};

use super::{hittable::{Hittable}, hit_record::HitRecord, hittable_enum::HittableEnum};


pub struct BVHNode {
    left_index : usize,
    right_index : usize,
    bbox: AABB,
}

impl BVHNode {
    pub fn from_index_list(rng: &mut ChaCha20Rng, hittable_service: &mut HittableService, index_list: &mut Vec<usize>, time_0: f32, time_1: f32) -> Self {
        let elements_count = index_list.len();
        let slice = index_list.as_mut_slice();
        Self::new(rng, hittable_service, slice, 0, elements_count, time_0, time_1)
    }

    // This doesn't need to be the fastest in the world right now as it only runs once before the render loop
    pub fn new(rng: &mut ChaCha20Rng, hittable_service: &mut HittableService, source_indices: &mut [usize], start: usize, end: usize, time_0: f32, time_1: f32) -> Self {
        let axis: u8 = rng.gen_range(0..3);

        // Probably optimize this by sharing this with the recursive call
        // This won't allow the capturing of hittable_service
        // This could be overcome by making the building loop based instead of 
        // Recursive, maybe in a different function
        let comparator: Box<dyn Fn(&usize, &usize) -> Ordering> = match axis {
            0 => Box::new(|a: &usize, b: &usize| -> Ordering {

                let mut box_a = AABB::default();
                hittable_service.bounding_box(*a, 0.0, 0.0, &mut box_a);
                let mut box_b = AABB::default();
                hittable_service.bounding_box(*b, 0.0, 0.0, &mut box_b);
    
                if box_a.minimum[0] < box_b.minimum[0] {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }),
            1 => Box::new(|a: &usize, b: &usize| -> Ordering {

                let mut box_a = AABB::default();
                hittable_service.bounding_box(*a, 0.0, 0.0, &mut box_a);
                let mut box_b = AABB::default();
                hittable_service.bounding_box(*b, 0.0, 0.0, &mut box_b);
    
                if box_a.minimum[1] < box_b.minimum[1] {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }),
            2 => Box::new(|a: &usize, b: &usize| -> Ordering {

                let mut box_a = AABB::default();
                hittable_service.bounding_box(*a, 0.0, 0.0, &mut box_a);
                let mut box_b = AABB::default();
                hittable_service.bounding_box(*b, 0.0, 0.0, &mut box_b);
    
                if box_a.minimum[2] < box_b.minimum[2] {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }),
            _ => panic!("Chose invalid axis in compare function!"),
        };



        let object_span = end - start;

        let result_left: usize;
        let result_right: usize;

        if object_span == 1 {
            result_left = source_indices[start];
            result_right = source_indices[start];
        } else if object_span == 2 {
            if comparator(&source_indices[start], &source_indices[start + 1]).is_lt() {
                result_left = source_indices[start];
                result_right = source_indices[start + 1];
            } else {
                result_left = source_indices[start + 1];
                result_right = source_indices[start];
            }
        } else {
            source_indices[start..end].sort_unstable_by(comparator);

            let mid = start + object_span / 2;
            let left_hittable = HittableEnum::BVHNode(Self::new(rng, hittable_service, source_indices, start, mid, time_0, time_1));
            result_left = hittable_service.add_hittable(left_hittable);
            let right_hittable = HittableEnum::BVHNode(Self::new(rng, hittable_service, source_indices, mid, end, time_0, time_1));
            result_right = hittable_service.add_hittable(right_hittable);
        }

        let mut bbox_left = AABB::default();
        let bbox_left_found = hittable_service.bounding_box(result_left, time_0, time_1, &mut bbox_left);

        let mut bbox_right = AABB::default();
        let bbox_right_found = hittable_service.bounding_box(result_right, time_0, time_1, &mut bbox_right);

        if !bbox_left_found || !bbox_right_found {
            panic!("Is missing a bounding box when constructing BVH");
        }

        bbox_left.expand_by_box(&bbox_right);

        BVHNode{ left_index: result_left, right_index: result_right, bbox: bbox_left}
    }


}

impl Hittable for BVHNode{
    fn hit(&self, rng: &mut ThreadRng, hittable_service: &HittableService, ray: &Ray, t_min: f32, t_max: f32, hit_out: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, t_min, t_max){
            return false;
        }

        let hit_left = hittable_service.hit(self.left_index, rng, ray, t_min, t_max, hit_out);
        let hit_right = hittable_service.hit(self.right_index, rng, ray, t_min, if hit_left { hit_out.t } else { t_max }, hit_out);

        hit_left || hit_right
    }

    fn bounding_box(&self, _hittable_service: &HittableService, _time_0: f32, _time_1: f32, box_out: &mut AABB) -> bool {
        box_out.minimum.x = self.bbox.minimum.x;
        box_out.minimum.y = self.bbox.minimum.y;
        box_out.minimum.z = self.bbox.minimum.z;

        box_out.maximum.x = self.bbox.maximum.x;
        box_out.maximum.y = self.bbox.maximum.y;
        box_out.maximum.z = self.bbox.maximum.z;

        true
    }

}