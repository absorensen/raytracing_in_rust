use std::f64::consts::PI;

use rand::Rng;
use rand::rngs::ThreadRng;

use crate::hittable_service::HittableService;
use crate::ortho_normal_base::OrthoNormalBase;
use crate::vector3::Vector3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::aabb::AABB;

pub struct Sphere {
    pub radius: f64,
    pub center: Vector3,
    pub material: usize,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: usize) -> Self { 
        Sphere {center, radius, material } 
    }

    fn get_sphere_uv(p: &Vector3) -> (f64, f64) {
        let theta = (-(*p).y).acos();
        let phi = f64::atan2(-(*p).z,(*p).x) + PI;

        ( phi / (2.0 * PI), theta / PI ) 
    }
}

impl Hittable for Sphere{
    fn hit(&self, _rng: &mut ThreadRng, _hittable_service: &HittableService, ray: &Ray, t_min: f64, t_max: f64, hit_out: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
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

    
    fn bounding_box(&self, _hittable_service: &HittableService, _time_0: f64, _time_1: f64, box_out: &mut AABB) -> bool {
        box_out.minimum.x = self.center.x - self.radius;
        box_out.minimum.y = self.center.y - self.radius;
        box_out.minimum.z = self.center.z - self.radius;

        box_out.maximum.x = self.center.x + self.radius;
        box_out.maximum.y = self.center.y + self.radius;
        box_out.maximum.z = self.center.z + self.radius;

        true
    }

    fn pdf_value(&self, rng: &mut ThreadRng, hittable_service: &HittableService, origin: &Vector3, v: &Vector3) -> f64 {
        let hit_out = &mut HitRecord::default();
        if self.hit(rng, hittable_service,  &Ray::new(*origin, *v, 0.5), 0.001, f64::INFINITY, hit_out) {
            let cos_theta_max = (1.0 - self.radius * self.radius / (self.center - *origin).length_squared()).sqrt();
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

            return 1.0 / solid_angle;
        }

        0.0
    }

    fn random(&self, rng: &mut ThreadRng, _hittable_service: &HittableService, origin: &Vector3) -> Vector3 {
        let direction = self.center - *origin;
        let distance_squared = direction.length_squared();
        let uvw = OrthoNormalBase::build_from_w(&direction);
        uvw.local_vector(&random_to_sphere(rng, self.radius, distance_squared))
    }


}

#[inline]
fn random_to_sphere(rng: &mut ThreadRng, radius: f64, distance_squared: f64) -> Vector3 {
    let r1 = rng.gen::<f64>();
    let r2 = rng.gen::<f64>();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    Vector3::new(x, y, z)
}