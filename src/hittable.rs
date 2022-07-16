use rand::Rng;
use rand::rngs::ThreadRng;

use crate::vector3::Vector3;
use crate::ray::Ray;
use crate::material::{Material, DefaultMaterial};
use crate::aabb::AABB;

use std::sync::Arc;

// Turn the material into an index and derive default
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
    fn hit(&self, rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64, hit_out: &mut HitRecord) -> bool;
    fn bounding_box(&self, time_0: f64, time_1: f64, box_out: &mut AABB) -> bool;
    fn pdf_value(&self, rng: &mut ThreadRng, origin: &Vector3, v: &Vector3, hit_out: &mut HitRecord) -> f64 { 0.0 }
    fn random(&self, rng: &mut ThreadRng, origin: &Vector3) -> Vector3 { Vector3::new(1.0, 0.0, 0.0) }
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn push(&mut self, hittable: impl Hittable + 'static) {
        self.objects.push(Arc::new(hittable))
    }

    pub fn push_arc(&mut self, hittable: &Arc<dyn Hittable>) {
        self.objects.push(Arc::clone(hittable))
    }

    pub fn as_slice_mut(&mut self) -> &mut [Arc<dyn Hittable>] {
        &mut self.objects
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }
}

impl Hittable for HittableList {
    fn hit(&self, rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64, hit_out: &mut HitRecord) -> bool {
        let mut closest_so_far = t_max;
        for hittable in self.objects.iter() {
            hittable.hit(rng, ray, t_min, closest_so_far, hit_out);
        }
        hit_out.t < t_max
    }

    fn bounding_box(&self, time_0: f64, time_1: f64, box_out: &mut AABB) -> bool {
        if self.objects.len() < 1 { return false };

        let mut temp_box_option: AABB = AABB::default();
        let mut first_box: bool = true;

        for object in &self.objects {
            if object.bounding_box(time_0, time_1, &mut temp_box_option) {
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

    fn pdf_value(&self, rng: &mut ThreadRng, origin: &Vector3, v: &Vector3, hit_out: &mut HitRecord) -> f64 {
        let mut sum = 0.0;
        let inverse_length = 1.0 / self.objects.len() as f64;
        for object_index in 0..self.objects.len(){
            sum += self.objects[object_index].pdf_value(rng, origin, v, hit_out) * inverse_length;
        }

        sum
    }

    fn random(&self, rng: &mut ThreadRng, origin: &Vector3) -> Vector3 {
        let random_object_index = rng.gen_range(0..self.objects.len());
        self.objects[random_object_index].random(rng, origin) / self.objects.len() as f64
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
    fn hit(&self, _rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64, hit_out: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t_max < t {
            return false;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;

        if x < self.x0 || self.x1 < x || y < self.y0 || self.y1 < y {
            return false;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = Vector3{x: 0.0, y: 0.0, z: 1.0};

        hit_out.t = t;
        hit_out.u = u;
        hit_out.v = v;
        hit_out.position = ray.at(t);
        hit_out.set_face_normal(ray, &outward_normal);
        hit_out.material = Arc::clone(&self.material);

        true
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64, box_out: &mut AABB) -> bool {
        box_out.minimum.x = self.x0;
        box_out.minimum.y = self.y0;
        box_out.minimum.z = self.k - 0.0001;

        box_out.maximum.x = self.x1;
        box_out.maximum.y = self.y1;
        box_out.maximum.z = self.k+0.0001;

        true
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
    fn hit(&self, _rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64, hit_out: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t_max < t {
            return false;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        if x < self.x0 || self.x1 < x || z < self.z0 || self.z1 < z {
            return false;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vector3{x: 0.0, y: 1.0, z: 0.0};
        
        hit_out.t = t;
        hit_out.u = u;
        hit_out.v = v;
        hit_out.position = ray.at(t);
        hit_out.set_face_normal(ray, &outward_normal);
        hit_out.material = Arc::clone(&self.material);

        true
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64, box_out: &mut AABB) -> bool {
        box_out.minimum.x = self.x0;
        box_out.minimum.y = self.k - 0.0001;
        box_out.minimum.z = self.z0;

        box_out.maximum.x = self.x1;
        box_out.maximum.y = self.k + 0.0001;
        box_out.maximum.z = self.z1;

        true
    }

    fn pdf_value(&self, rng: &mut ThreadRng, origin: &Vector3, v: &Vector3, hit_out: &mut HitRecord) -> f64 {
        let ray = Ray::new(origin.clone(), v.clone(), 0.0);
        if self.hit(rng, &ray, 0.001, f64::INFINITY, hit_out) {

            let area = (self.x1 - self.x0) * (self.z1 - self.z0);
            let distance_squared = hit_out.t * hit_out.t * v.length_squared();
            let cosine = (Vector3::dot(v, &hit_out.normal) / v.length()).abs();

            return distance_squared / (cosine * area);
        }

        0.0
    }

    fn random(&self, rng: &mut ThreadRng, origin: &Vector3) -> Vector3 {
        let random_point = Vector3::new(rng.gen_range(self.x0..self.x1), self.k, rng.gen_range(self.z0..self.z1));

        random_point - *origin
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
    fn hit(&self, _rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64, hit_out: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t_max < t {
            return false;
        }

        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;


        if y < self.y0 || self.y1 < y || z < self.z0 || self.z1 < z {
            return false;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vector3{x: 1.0, y: 0.0, z: 0.0};
        
        hit_out.t = t;
        hit_out.u = u;
        hit_out.v = v;
        hit_out.position = ray.at(t);
        hit_out.set_face_normal(ray, &outward_normal);
        hit_out.material = Arc::clone(&self.material);

        true
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64, box_out: &mut AABB) -> bool {
        box_out.minimum.x = self.k - 0.0001;
        box_out.minimum.y = self.y0;
        box_out.minimum.z = self.z0;

        box_out.maximum.x = self.k + 0.0001;
        box_out.maximum.y = self.y1;
        box_out.maximum.z = self.z1;

        true
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
    fn hit(&self, rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64, hit_out: &mut HitRecord) -> bool {
        self.sides.hit(rng, ray, t_min, t_max, hit_out)
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64, box_out: &mut AABB) -> bool {
        box_out.minimum.x = self.box_min.x;
        box_out.minimum.y = self.box_min.y;
        box_out.minimum.z = self.box_min.z;

        box_out.maximum.x = self.box_max.x;
        box_out.maximum.y = self.box_max.y;
        box_out.maximum.z = self.box_max.z;

        true
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
    fn hit(&self, rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64, hit_out: &mut HitRecord) -> bool {
        let moved_ray = Ray{ origin: ray.origin - self.offset, direction: ray.direction, time: ray.time };
        if !self.model.hit(rng, &moved_ray, t_min, t_max, hit_out) {
            return false;
        }

        hit_out.position += self.offset;
        // Cloning the normal here is poop, and should be refactored somehow.
        // The issues is hit is borrowed mutably for set_face_normal, making 
        // impossible the immutable borrow for outward_normal
        hit_out.set_face_normal(&moved_ray, &hit_out.normal.clone());

        true
    }

    fn bounding_box(&self, time_0: f64, time_1: f64, box_out: &mut AABB) -> bool {
        if !self.model.bounding_box(time_0, time_1, box_out) {
            return false;
        } 

        box_out.minimum += self.offset;
        box_out.maximum += self.offset;

        return true;
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

        let mut bbox = AABB::default();
        let has_bbox = model.bounding_box(0.0, 1.0, &mut bbox);

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
    fn hit(&self, rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64, hit_out: &mut HitRecord) -> bool {
        let mut origin = ray.origin.clone();
        let mut direction = ray.direction.clone();

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_ray = Ray{ origin, direction, time: ray.time };

        if !self.model.hit(rng, &rotated_ray, t_min, t_max, hit_out) {
            return false;
        }

        let mut position = hit_out.position.clone();
        let mut normal = hit_out.normal.clone();

        position[0] = self.cos_theta * hit_out.position[0] + self.sin_theta * hit_out.position[2];
        position[2] = -self.sin_theta * hit_out.position[0] + self.cos_theta * hit_out.position[2];

        normal[0] = self.cos_theta * hit_out.normal[0] + self.sin_theta * hit_out.normal[2];
        normal[2] = -self.sin_theta * hit_out.normal[0] + self.cos_theta * hit_out.normal[2];

        // Cloning the normal here is poop, and should be refactored somehow.
        // The issues is hit is borrowed mutably for set_face_normal, making 
        // impossible the immutable borrow for outward_normal
        hit_out.set_face_normal(&rotated_ray, &normal);

        hit_out.position = position;
        hit_out.normal = normal;


        true
    }

    fn bounding_box(&self, time_0: f64, time_1: f64, box_out: &mut AABB) -> bool {
        box_out.minimum.x = self.bbox.minimum.x;
        box_out.minimum.y = self.bbox.minimum.y;
        box_out.minimum.z = self.bbox.minimum.z;

        box_out.maximum.x = self.bbox.maximum.x;
        box_out.maximum.y = self.bbox.maximum.y;
        box_out.maximum.z = self.bbox.maximum.z;

        true
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
    fn hit(&self, rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64, hit_out: &mut HitRecord) -> bool {

        // TODO: Try using hit_out in both hits
        let default_material: Arc<dyn Material> = Arc::new(DefaultMaterial{});
        let mut hit_1 = HitRecord::new(ray, 0.0, 0.0, 0.0, &Vector3::zero(), &Vector3::zero(), &default_material);
        let hit_1_hit = self.boundary.hit(rng, ray, f64::NEG_INFINITY, f64::INFINITY, &mut hit_1);

        let mut hit_2 = HitRecord::new(ray, 0.0, 0.0, 0.0, &Vector3::zero(), &Vector3::zero(), &default_material);
        let hit_2_hit = self.boundary.hit(rng, ray, hit_1.t+0.0001, f64::INFINITY, &mut hit_2);


        if hit_1.t < t_min { hit_1.t = t_min; };
        if t_max < hit_2.t { hit_2.t = t_max; };

        if hit_2.t <= hit_1.t { return false; }

        if hit_1.t < 0.0 { hit_1.t = 0.0; }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (hit_2.t - hit_1.t) * ray_length;
        let hit_distance = self.negative_inverse_density * rng.gen::<f64>().ln();

        if distance_inside_boundary < hit_distance { return false; }

        let t = hit_1.t + hit_distance / ray_length; 

        hit_out.t = t;
        hit_out.u = 0.0;
        hit_out.v = 0.0;
        hit_out.position = ray.at(t);
        hit_out.set_face_normal(ray, &Vector3 { x: 1.0, y: 0.0, z: 0.0 });
        hit_out.material = Arc::clone(&self.phase_function);

        true
    }

    fn bounding_box(&self, time_0: f64, time_1: f64, box_out: &mut AABB) -> bool {
        self.boundary.bounding_box(time_0, time_1, box_out)
    }

    fn pdf_value(&self, rng: &mut ThreadRng, origin: &Vector3, v: &Vector3, hit_out: &mut HitRecord) -> f64 { 0.0 }

    fn random(&self, rng: &mut ThreadRng, origin: &Vector3) -> Vector3 { Vector3::new(1.0, 0.0, 0.0) }

}

pub struct FlipFace {
    model: Arc<dyn Hittable>,
}

impl FlipFace {
    pub fn new(model: &Arc<dyn Hittable>) -> FlipFace {
        FlipFace{model: Arc::clone(model)}
    }

}

impl Hittable for FlipFace {
    fn hit(&self, rng: &mut ThreadRng, ray: &Ray, t_min: f64, t_max: f64, hit_out: &mut HitRecord) -> bool {
        if !self.model.hit(rng, &ray, t_min, t_max, hit_out) {
            return false;            
        }

        // TODO: Shouldn't this also flip the normal?
        hit_out.is_front_face = !hit_out.is_front_face;

        true
    }

    fn bounding_box(&self, time_0: f64, time_1: f64, box_out: &mut AABB) -> bool {
        self.model.bounding_box(time_0, time_1, box_out)
    }
}