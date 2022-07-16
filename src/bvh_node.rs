use core::panic;
use std::{sync::Arc, cmp::Ordering};
use rand::{Rng, rngs::ThreadRng};
use crate::{hittable::{Hittable, HitRecord, HittableList}, aabb::AABB, ray::Ray, vector3::Vector3};


pub struct BVHNode {
    left : Arc<dyn Hittable>,
    right : Arc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn from_hittable_list(list: &mut HittableList, time_0: f64, time_1: f64) -> Self {
        let mut rng = rand::thread_rng();
        let elements_count = list.len();
        Self::new(&mut rng, list.as_slice_mut(), 0, elements_count, time_0, time_1)
    }

    // This doesn't need to be the fastest in the world right now as it only runs once before the render loop
    pub fn new(rng: &mut ThreadRng, source_objects: &mut [Arc<dyn Hittable>], start: usize, end: usize, time_0: f64, time_1: f64) -> Self {
        let axis: u8 = rng.gen_range(0..3);
        let comparator = match axis {
            0 => Self::compare::<0>,
            1 => Self::compare::<1>,
            2 => Self::compare::<2>,
            _ => panic!("Chose invalid axis in compare function!"),
        };

        let object_span = end - start;

        let result_left: Arc<dyn Hittable>;
        let result_right: Arc<dyn Hittable>;

        if object_span == 1 {
            result_left = Arc::clone(&source_objects[start]);
            result_right = Arc::clone(&source_objects[start]);
        } else if object_span == 2 {
            if comparator(&source_objects[start], &source_objects[start + 1]).is_lt() {
                result_left = Arc::clone(&source_objects[start]);
                result_right = Arc::clone(&source_objects[start + 1]);
            } else {
                result_left = Arc::clone(&source_objects[start + 1]);
                result_right = Arc::clone(&source_objects[start]);
            }
        } else {
            source_objects[start..end].sort_unstable_by(comparator);

            let mid = start + object_span / 2;
            result_left = Arc::new(Self::new(rng, source_objects, start, mid, time_0, time_1));
            result_right = Arc::new(Self::new(rng, source_objects, mid, end, time_0, time_1));
        }

        let mut bbox_left = AABB::default();
        let bbox_left_found = result_left.bounding_box(time_0, time_1, &mut bbox_left);

        let mut bbox_right = AABB::default();
        let bbox_right_found = result_right.bounding_box(time_0, time_1, &mut bbox_right);

        if !bbox_left_found || !bbox_right_found {
            panic!("Is missing a bounding box when constructing BVH");
        }

        bbox_left.expand_by_box(&bbox_right);

        BVHNode{ left: result_left, right: result_right, bbox: bbox_left}
    }

    // Inspired by https://github.com/fyrchik/rayst/blob/main/src/bvh.rs
    // but I don't have my Vector3 as an array, so I had to come up with a fix
    fn compare<const AXIS: usize>(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {

        let mut box_a = AABB::default();
        a.bounding_box(0.0, 0.0, &mut box_a);
        let mut box_b = AABB::default();
        b.bounding_box(0.0, 0.0, &mut box_b);

        if box_a.minimum[AXIS] < box_b.minimum[AXIS] {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl Hittable for BVHNode{


    fn hit(&self, rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64, hit_out: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, t_min, t_max){
            return false;
        }

        let hit_left = self.left.hit(rng, ray, t_min, t_max, hit_out);
        let hit_right = self.right.hit(rng, ray, t_min, if hit_left { hit_out.t } else { t_max }, hit_out);

        hit_left || hit_right
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64, box_out: &mut AABB) -> bool {
        box_out.minimum.x = self.bbox.minimum.x;
        box_out.minimum.y = self.bbox.minimum.y;
        box_out.minimum.z = self.bbox.minimum.z;

        box_out.maximum.x = self.bbox.maximum.x;
        box_out.maximum.y = self.bbox.maximum.y;
        box_out.maximum.z = self.bbox.maximum.z;

        true
    }

    fn pdf_value(&self, rng: &mut ThreadRng, origin: &Vector3, v: &Vector3, hit_out: &mut HitRecord) -> f64 {
        0.0
    }

    fn random(&self, rng: &mut ThreadRng, origin: &Vector3) -> Vector3 {
        Vector3::new(1.0, 0.0, 0.0)
    }
}