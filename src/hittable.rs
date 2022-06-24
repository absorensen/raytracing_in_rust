use crate::vector3::Vector3;
use crate::ray::Ray;
use crate::material::Material;
use crate::aabb::AABB;

use std::sync::Arc;

pub struct HitRecord<'a> {
    pub t: f64,
    pub position: Vector3,
    pub normal: Vector3,
    pub material: &'a dyn Material,
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB>;
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn push(&mut self, hittable: impl Hittable + 'static) {
        self.objects.push(Arc::new(hittable))
    }

    pub fn as_slice_mut(&mut self) -> &mut [Arc<dyn Hittable>] {
        &mut self.objects
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything: Option<HitRecord> = None;
        for hittable in self.objects.iter() {
            if let Some(hit) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        hit_anything
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB> {
        if self.objects.len() < 1 { return None };

        let mut temp_box_option: Option<AABB>;
        let mut output_box: AABB = AABB { minimum: Vector3 { x: 0.0, y: 0.0, z: 0.0 }, maximum: Vector3 { x: 0.0, y: 0.0, z: 0.0 } };
        let mut first_box: bool = true;

        for object in &self.objects {
            temp_box_option = object.bounding_box(time_0, time_1);

            match temp_box_option {
                Some(temp_box) => {
                    if first_box { 
                        first_box = false;
                        output_box.clone_from(&temp_box); 
                    } else { 
                        output_box.expand_by_box(&temp_box);
                    }
                },
                None => return None,
            }
        }


        Some(output_box)
    }

}