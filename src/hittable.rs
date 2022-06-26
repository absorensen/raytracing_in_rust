use rand::Rng;
use rand::rngs::ThreadRng;

use crate::texture::Texture;
use crate::vector3::Vector3;
use crate::ray::Ray;
use crate::material::{Material, Isotropic};
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
    fn hit(&self, rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
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
    fn hit(&self, rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything: Option<HitRecord> = None;
        for hittable in self.objects.iter() {
            if let Some(hit) = hittable.hit(rng, ray, t_min, closest_so_far) {
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
    fn hit(&self, _rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
    fn hit(&self, _rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
    fn hit(&self, _rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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

pub struct Box {
    sides: HittableList,
    box_min: Vector3,
    box_max: Vector3,
}

impl Box {
    pub fn new(point_0: Vector3, point_1: Vector3, material: &Arc<dyn Material>) -> Box {
        let mut sides : HittableList = HittableList::default();

        sides.push(XYRect::new(point_0.x, point_1.x, point_0.y, point_1.y, point_1.z, material));
        sides.push(XYRect::new(point_0.x, point_1.x, point_0.y, point_1.y, point_0.z, material));
        
        sides.push(XZRect::new(point_0.x, point_1.x, point_0.z, point_1.z, point_1.y, material));
        sides.push(XZRect::new(point_0.x, point_1.x, point_0.z, point_1.z, point_0.y, material));
        
        sides.push(YZRect::new(point_0.y, point_1.y, point_0.z, point_1.z, point_1.x, material));
        sides.push(YZRect::new(point_0.y, point_1.y, point_0.z, point_1.z, point_0.x, material));

        Box { box_min: point_0, box_max: point_1, sides: sides }
    }

}

impl Hittable for Box {
    fn hit(&self, rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(rng, ray, t_min, t_max)
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<AABB> {
        Some(AABB{minimum: self.box_min, maximum: self.box_max})
    }

}

pub struct Translate {
    model: Arc<dyn Hittable>,
    offset: Vector3,
}

impl Translate {
    pub fn new(displacement: Vector3, model: &Arc<dyn Hittable>) -> Translate {
        Translate{model: Arc::clone(model), offset: displacement}
    }

}

impl Hittable for Translate {
    fn hit(&self, rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray{ origin: ray.origin - self.offset, direction: ray.direction, time: ray.time };
        let hit_option = self.model.hit(rng, &moved_ray, t_min, t_max);
        if hit_option.is_none() {
            return None;            
        }

        let mut hit = hit_option.expect("Tried to get result which wasn't there");
        hit.position += self.offset;
        // Cloning the normal here is poop, and should be refactored somehow.
        // The issues is hit is borrowed mutably for set_face_normal, making 
        // impossible the immutable borrow for outward_normal
        hit.set_face_normal(&moved_ray, &hit.normal.clone());

        Some(hit)
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB> {
        if let Some(mut bbox) = self.model.bounding_box(time_0, time_1) {
            bbox.minimum += self.offset;
            bbox.maximum += self.offset;

            Some(bbox)
        } else {
            None
        }
    }

}

pub struct RotateY {
    model: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    has_bbox: bool,
    bbox: AABB,
}

impl RotateY {
    pub fn new(angle: f64, model: &Arc<dyn Hittable>) -> RotateY {
        let radians = angle.to_radians();

        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox_option = model.bounding_box(0.0, 1.0);
        let has_bbox = bbox_option.is_some();
        let bbox = if has_bbox { bbox_option.expect("Tried to get a bbox which wasn't available")} else { AABB::default() };

        let mut min = Vector3{x: f64::INFINITY, y: f64::INFINITY, z: f64::INFINITY };
        let mut max = Vector3{x: f64::NEG_INFINITY, y: f64::NEG_INFINITY, z: f64::NEG_INFINITY };
        
        for i in 0..2 {
            let i_f = i as f64;
            for j in 0..2 {
                let j_f = j as f64;
                for k in 0..2 {
                    let k_f = k as f64;

                    let x = i_f  * bbox.maximum.x + (1.0 - i_f) * bbox.minimum.x;
                    let y = j_f  * bbox.maximum.y + (1.0 - j_f) * bbox.minimum.y;
                    let z = k_f  * bbox.maximum.z + (1.0 - k_f) * bbox.minimum.z;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vector3{x: new_x, y, z: new_z};

                    min[0] = min[0].min(tester[0]);
                    min[1] = min[1].min(tester[1]);
                    min[2] = min[2].min(tester[2]);
                    max[0] = max[0].max(tester[0]);
                    max[1] = max[1].max(tester[1]);
                    max[2] = max[2].max(tester[2]);
                }
            }
        }

        let bbox = AABB{minimum: min, maximum: max};

        RotateY { model: Arc::clone(model), sin_theta, cos_theta, has_bbox, bbox }
    }
}

impl Hittable for RotateY {
    fn hit(&self, rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = ray.origin.clone();
        let mut direction = ray.direction.clone();

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_ray = Ray{ origin, direction, time: ray.time };

        if let Some(mut hit) = self.model.hit(rng, &rotated_ray, t_min, t_max) {
            let mut position = hit.position.clone();
            let mut normal = hit.normal.clone();

            position[0] = self.cos_theta * hit.position[0] + self.sin_theta * hit.position[2];
            position[2] = -self.sin_theta * hit.position[0] + self.cos_theta * hit.position[2];

            normal[0] = self.cos_theta * hit.normal[0] + self.sin_theta * hit.normal[2];
            normal[2] = -self.sin_theta * hit.normal[0] + self.cos_theta * hit.normal[2];

            // Cloning the normal here is poop, and should be refactored somehow.
            // The issues is hit is borrowed mutably for set_face_normal, making 
            // impossible the immutable borrow for outward_normal
            hit.set_face_normal(&rotated_ray, &normal);
    
            hit.position = position;
            hit.normal = normal;

            return Some(hit);
        }

        None
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB> {
        Some(self.bbox)
    }

}


pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    negative_inverse_density: f64,
}

impl ConstantMedium {
    pub fn new(model: &Arc<dyn Hittable>, phase_function: &Arc<dyn Material>, density: f64) -> ConstantMedium {
        ConstantMedium { 
            boundary: Arc::clone(model), 
            phase_function: Arc::clone(phase_function), 
            negative_inverse_density: -1.0 / density 
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let enable_debug = false;
        let debugging = enable_debug && rng.gen::<f64>() < 0.00001;

        let hit1 = self.boundary.hit(rng, ray, f64::NEG_INFINITY, f64::INFINITY);
        let mut hit1 = if hit1.is_some() { hit1.unwrap() } else { return None };

        let hit2 = self.boundary.hit(rng, ray, hit1.t+0.0001, f64::INFINITY);
        let mut hit2 = if hit2.is_some() { hit2.unwrap() } else { return None };

        if debugging { print!("\nt_min={}, t_max={}\n", hit1.t, hit2.t); };

        if hit1.t < t_min { hit1.t = t_min; };
        if t_max < hit2.t { hit2.t = t_max; };

        if hit2.t <= hit1.t { return None; }

        if hit1.t < 0.0 { hit1.t = 0.0; }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
        let hit_distance = self.negative_inverse_density * rng.gen::<f64>().ln();

        if distance_inside_boundary < hit_distance { return None; }

        let t = hit1.t + hit_distance / ray_length; 
        let hit = 
            HitRecord{ 
                t: t, 
                u: 0.0, 
                v: 0.0, 
                position: ray.at(t), 
                normal: Vector3 { x: 1.0, y: 0.0, z: 0.0 }, 
                is_front_face: true, 
                material: Arc::clone(&self.phase_function) 
            };

            Some(hit)
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB> {
        self.boundary.bounding_box(time_0, time_1)
    }

}