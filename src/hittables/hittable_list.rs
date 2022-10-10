use ultraviolet::Vec3;
use rand::{rngs::ThreadRng, Rng};

use crate::{services::hittable_service::HittableService, core::ray::Ray, geometry::aabb::AABB};

use super::{hittable::Hittable, hit_record::HitRecord};

// Get rid of this, or make sure it is only used for a pre-BVH-build step
#[derive(Default)]
pub struct HittableList {
    hittable_indices: Vec<usize>,
}

impl HittableList {
    pub fn from_list(hittable_indices: Vec<usize>) -> HittableList {
        HittableList { hittable_indices }
    }
}

impl Hittable for HittableList {
    fn hit(&self, rng: &mut ThreadRng, hittable_service: &HittableService, ray: &Ray, t_min: f32, t_max: f32, hit_out: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for hittable_index in &self.hittable_indices {
            if hittable_service.hit(*hittable_index, rng, ray, t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                hit_out.t = temp_record.t;
                hit_out.u = temp_record.u;
                hit_out.v = temp_record.v;
                hit_out.position = temp_record.position;
                hit_out.normal = temp_record.normal;
                hit_out.is_front_face = temp_record.is_front_face;
                hit_out.material = temp_record.material;
            }
        }

        hit_anything
    }

    fn bounding_box(&self, hittable_service: &HittableService, time_0: f32, time_1: f32, box_out: &mut AABB) -> bool {
        if self.hittable_indices.len() < 1 { return false };

        let mut temp_box_option: AABB = AABB::default();
        let mut first_box: bool = true;

        for hittable_index in &self.hittable_indices {
            if hittable_service.bounding_box(*hittable_index, time_0, time_1, &mut temp_box_option) {
                if first_box { 
                    first_box = false;

                    box_out.minimum.x = temp_box_option.minimum.x;
                    box_out.minimum.y = temp_box_option.minimum.y;
                    box_out.minimum.z = temp_box_option.minimum.z;

                    box_out.maximum.x = temp_box_option.maximum.x;
                    box_out.maximum.y = temp_box_option.maximum.y;
                    box_out.maximum.z = temp_box_option.maximum.z;
                } else { 
                    box_out.expand_by_box(&temp_box_option);
                }
            }
        }


        !first_box
    }

    fn pdf_value(&self, rng: &mut ThreadRng, hittable_service: &HittableService, origin: &Vec3, v: &Vec3) -> f32 {
        let mut sum = 0.0;

        for object_index in 0..self.hittable_indices.len(){
            sum += hittable_service.pdf_value(self.hittable_indices[object_index], rng, origin, v);
        }

        sum / self.hittable_indices.len() as f32
    }

    fn random(&self, rng: &mut ThreadRng, hittable_service: &HittableService, origin: &Vec3) -> Vec3 {
        let random_object_index = rng.gen_range(0..self.hittable_indices.len());
        hittable_service.random(self.hittable_indices[random_object_index], rng, origin) / self.hittable_indices.len() as f32
    }

}
