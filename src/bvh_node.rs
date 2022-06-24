use core::panic;
use std::{sync::Arc, cmp::Ordering};
use rand::{Rng, rngs::ThreadRng};
use crate::{hittable::{Hittable, HitRecord, HittableList}, aabb::AABB, ray::Ray};


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
        let axis: u8 = rng.gen_range(0, 3);
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

        let bbox_left = result_left.bounding_box(time_0, time_1);
        let bbox_right = result_right.bounding_box(time_0, time_1);

        if bbox_left.is_none() || bbox_right.is_none() {
            panic!("Is missing a bounding box when constructing BVH");
        }

        let mut bbox_result = bbox_left.unwrap();
        bbox_result.expand_by_box(&bbox_right.unwrap());

        BVHNode{ left: result_left, right: result_right, bbox: bbox_result}
    }

    // Inspired by https://github.com/fyrchik/rayst/blob/main/src/bvh.rs
    // but I don't have my Vector3 as an array, so I had to come up with a fix
    fn compare<const AXIS: usize>(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        let box_a = a.bounding_box(0.0, 0.0).unwrap();
        let box_b = b.bounding_box(0.0, 0.0).unwrap();

        match AXIS {
            0 => {
                if box_a.minimum.x < box_b.minimum.x {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            },
            1 => {
                if box_a.minimum.y < box_b.minimum.y {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            },
            2 => {
                if box_a.minimum.z < box_b.minimum.z {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            },
            _ => {
                panic!("Chose invalid axis in compare function!")
            }
        }
    }
}

impl Hittable for BVHNode{


    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(ray, t_min, t_max){
            return None;
        }
        
        return match self.left.hit(ray, t_min, t_max) {
            None => self.right.hit(ray, t_min, t_max),
            Some(hit_left) => match self.right.hit(ray, t_min, hit_left.t) {
                None => Some(hit_left),
                hit_right => hit_right,
            },
        }
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}