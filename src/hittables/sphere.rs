use std::f32::consts::PI;

use ultraviolet::Vec3;
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::math::ortho_normal_base::OrthoNormalBase;
use crate::{geometry::aabb::AABB};
use crate::core::ray::Ray;
use crate::services::hittable_service::HittableService;

use super::hit_record::HitRecord;
use super::hittable::{Hittable};

pub struct Sphere {
    pub radius: f32,
    pub center: Vec3,
    pub material: usize,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: usize) -> Self { 
        Sphere {center, radius, material } 
    }

    fn get_sphere_uv(p: &Vec3) -> (f32, f32) {
        let theta = (-(*p).y).acos();
        let phi = f32::atan2(-(*p).z,(*p).x) + PI;

        ( phi / (2.0 * PI), theta / PI ) 
    }
}

impl Hittable for Sphere{
    fn hit(&self, _rng: &mut ThreadRng, _hittable_service: &HittableService, ray: &Ray, t_min: f32, t_max: f32, hit_out: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.mag_sq();
        let half_b = ray.direction.dot(oc);
        let c = oc.mag_sq() - (self.radius * self.radius);
        
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
        let normal = (position - self.center) / self.radius;
        let (u, v) = Sphere::get_sphere_uv(&normal);

        hit_out.t = root;
        hit_out.u = u;
        hit_out.v = v;
        hit_out.position = position;
        hit_out.set_face_normal(ray, &normal);
        hit_out.material = self.material;

        true
    }

    
    fn bounding_box(&self, _hittable_service: &HittableService, _time_0: f32, _time_1: f32, box_out: &mut AABB) -> bool {
        box_out.minimum.x = self.center.x - self.radius;
        box_out.minimum.y = self.center.y - self.radius;
        box_out.minimum.z = self.center.z - self.radius;

        box_out.maximum.x = self.center.x + self.radius;
        box_out.maximum.y = self.center.y + self.radius;
        box_out.maximum.z = self.center.z + self.radius;

        true
    }

    fn pdf_value(&self, rng: &mut ThreadRng, hittable_service: &HittableService, origin: &Vec3, v: &Vec3) -> f32 {
        let hit_out = &mut HitRecord::default();
        if self.hit(rng, hittable_service,  &Ray::new_normalized(*origin, *v, 0.5), 0.001, f32::INFINITY, hit_out) {
            let cos_theta_max = (1.0 - self.radius * self.radius / (self.center - *origin).mag_sq()).sqrt();
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

            return 1.0 / solid_angle;
        }

        0.0
    }

    fn random(&self, rng: &mut ThreadRng, _hittable_service: &HittableService, origin: &Vec3) -> Vec3 {
        let direction = self.center - *origin;
        let distance_squared = direction.mag_sq();
        let uvw = OrthoNormalBase::build_from_w(&direction);
        uvw.local_vector(&random_to_sphere(rng, self.radius, distance_squared))
    }


}

#[inline]
fn random_to_sphere(rng: &mut ThreadRng, radius: f32, distance_squared: f32) -> Vec3 {
    let r1 = rng.gen::<f32>();
    let r2 = rng.gen::<f32>();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    Vec3::new(x, y, z)
}