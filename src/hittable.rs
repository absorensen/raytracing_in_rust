use crate::vector3::Vector3;
use crate::ray::Ray;
use crate::material::Material;
use crate::aabb::AABB;

use std::sync::Arc;

pub struct HitRecord {
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub position: Vector3,
    pub normal: Vector3,
    pub is_front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord{
    pub fn new(
        ray: &Ray,
        t: f64,
        u: f64,
        v: f64,
        position: &Vector3,
        normal: &Vector3,
        material: &Arc<dyn Material>
    ) -> Self {
        let mut result = HitRecord{ t, u, v, position: position.clone(), normal: normal.clone(), is_front_face: true, material: Arc::clone(material) };
        result.set_face_normal(ray, normal);
        result
    }
    
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) {
        self.is_front_face = Vector3::dot(&ray.direction, outward_normal) < 0.0;
        if self.is_front_face {
            self.normal = outward_normal.clone();
        } else {
            self.normal = -outward_normal.clone();
        }
    }
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
        let mut output_box: AABB = 
            AABB { 
                minimum: Vector3 { x: 0.0, y: 0.0, z: 0.0 }, 
                maximum: Vector3 { x: 0.0, y: 0.0, z: 0.0 } 
            };
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