use std::f32::consts::PI;

use rand::rngs::ThreadRng;

use crate::{core::ray::Ray, math::vector3::Vector3, services::hittable_service::HittableService, geometry::aabb::AABB};

use super::{hittable::{Hittable}, hit_record::HitRecord};

pub struct MovingSphere {
    pub radius: f32,
    pub center_0: Vector3,
    pub center_1: Vector3,
    pub material: usize,
    pub time_0: f32,
    pub time_1: f32,
}

impl MovingSphere {
    pub fn new(radius: f32, center_0: Vector3, center_1: Vector3, material: usize, time_0: f32, time_1: f32) -> Self { 
        MovingSphere {
            radius, 
            center_0,
            center_1,
            material,
            time_0,
            time_1 
        } 
    }

    fn get_sphere_uv(p: &Vector3) -> (f32, f32) {
        let theta = (-(*p).y).acos();
        let phi = f32::atan2(-(*p).z,(*p).x) + PI;

        ( phi / (2.0 * PI), theta / PI ) 
    }
}

impl MovingSphere {
    pub fn center(&self, time: f32) -> Vector3 {
        self.center_0 + (self.center_1 - self.center_0) * ((time - self.time_0) / (self.time_1 - self.time_0))
    }
}

impl Hittable for MovingSphere{
    fn hit(&self, _rng: &mut ThreadRng, _hittable_service: &HittableService, ray: &Ray, t_min: f32, t_max: f32, hit_out: &mut HitRecord) -> bool {
        let center = self.center(ray.time);

        let oc = ray.origin - center;
        let a = ray.direction.length_squared();
        let half_b = Vector3::dot(&ray.direction, &oc);
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        let position = ray.at(root);
        let normal = (position - center) / self.radius;
        let (u, v) = MovingSphere::get_sphere_uv(&normal);

        hit_out.t = root;
        hit_out.u = u;
        hit_out.v = v;
        hit_out.position = position;
        hit_out.set_face_normal(ray, &normal);
        hit_out.material = self.material;

        true
    }



    fn bounding_box(&self, _hittable_service: &HittableService, time_0: f32, time_1: f32, box_out: &mut AABB) -> bool {
        let center_0 = self.center(time_0);
        let center_1 = self.center(time_1);

        box_out.minimum.x = center_0.x - self.radius;
        box_out.minimum.y = center_0.y - self.radius;
        box_out.minimum.z = center_0.z - self.radius;

        box_out.maximum.x = center_0.x + self.radius;
        box_out.maximum.y = center_0.y + self.radius;
        box_out.maximum.z = center_0.z + self.radius;

        box_out.expand_by_point(&(center_1 - Vector3{x: self.radius, y: self.radius, z: self.radius}));
        box_out.expand_by_point(&(center_1 + Vector3{x: self.radius, y: self.radius, z: self.radius}));

        true
    }

    fn pdf_value(&self, _rng: &mut ThreadRng, _hittable_service: &HittableService, _origin: &Vector3, _v: &Vector3) -> f32 {
        0.0
    }

    fn random(&self, _rng: &mut ThreadRng, _hittable_service: &HittableService, _origin: &Vector3) -> Vector3 {
        Vector3::new(1.0, 0.0, 0.0)
    }
}