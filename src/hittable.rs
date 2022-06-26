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
    // Maybe convert these to take an output argument
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

pub struct XYRect {
    material: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: &Arc<dyn Material>) -> XYRect {
        XYRect { material: Arc::clone(material), x0, x1, y0, y1, k }
    }

}

impl Hittable for XYRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t_max < t {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;

        if x < self.x0 || self.x1 < x || y < self.y0 || self.y1 < y {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = Vector3{x: 0.0, y: 0.0, z: 1.0};
        
        Some(
            HitRecord::new(
                ray, 
                t, 
                u, 
                v,
                &ray.at(t), 
                &outward_normal, 
                &self.material
            )
        )

        

    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<AABB> {
        Some(AABB{ minimum: Vector3 { x: self.x0, y: self.y0, z: self.k-0.0001 }, maximum: Vector3{x: self.x1, y: self.y1, z: self.k+0.0001} })
    }

}

pub struct XZRect {
    material: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: &Arc<dyn Material>) -> XZRect {
        XZRect { material: Arc::clone(material), x0, x1, z0, z1, k }
    }

}

impl Hittable for XZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t_max < t {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        if x < self.x0 || self.x1 < x || z < self.z0 || self.z1 < z {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vector3{x: 0.0, y: 1.0, z: 0.0};
        
        Some(
            HitRecord::new(
                ray, 
                t, 
                u, 
                v,
                &ray.at(t), 
                &outward_normal, 
                &self.material
            )
        )

    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<AABB> {
        Some(AABB{ minimum: Vector3 { x: self.x0, y: self.k-0.0001, z: self.z0 }, maximum: Vector3{x: self.x1, y: self.k+0.0001, z: self.z1} })
    }

}

pub struct YZRect {
    material: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64,  k: f64, material: &Arc<dyn Material>) -> YZRect {
        YZRect { material: Arc::clone(material), y0, y1, z0, z1, k }
    }

}

impl Hittable for YZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t_max < t {
            return None;
        }

        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;


        if y < self.y0 || self.y1 < y || z < self.z0 || self.z1 < z {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vector3{x: 1.0, y: 0.0, z: 0.0};
        
        Some(
            HitRecord::new(
                ray, 
                t, 
                u, 
                v,
                &ray.at(t), 
                &outward_normal, 
                &self.material
            )
        )

        

    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<AABB> {
        Some(AABB{ minimum: Vector3 { x: self.k-0.0001, y: self.y0, z: self.z0 }, maximum: Vector3{x: self.k+0.0001, y: self.y1, z: self.z1} })
    }

}