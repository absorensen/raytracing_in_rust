use rand::{rngs::ThreadRng, Rng};

use crate::{services::hittable_service::HittableService, ray::Ray, math::vector3::Vector3, geometry::aabb::AABB};

use super::{hittable::Hittable, hit_record::HitRecord};

pub struct ConstantMedium {
    boundary_index: usize,
    phase_function: usize,
    negative_inverse_density: f32,
}

impl ConstantMedium {
    pub fn new(model_index: usize, phase_function: usize, density: f32) -> ConstantMedium {
        ConstantMedium { 
            boundary_index: model_index, 
            phase_function, 
            negative_inverse_density: -1.0 / density 
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, rng: &mut ThreadRng, hittable_service: &HittableService, ray: &Ray, t_min: f32, t_max: f32, hit_out: &mut HitRecord) -> bool {

        // TODO: Try using hit_out in both hits
        let zero_vector = Vector3::zero();
        let mut hit_1 = HitRecord::new(ray, 0.0, 0.0, 0.0, &zero_vector, &zero_vector, 0);
        if !hittable_service.hit(self.boundary_index, rng, ray, f32::NEG_INFINITY, f32::INFINITY, &mut hit_1) {
            return false;
        }

        let mut hit_2 = HitRecord::new(ray, 0.0, 0.0, 0.0, &zero_vector, &zero_vector, 0);
        if !hittable_service.hit(self.boundary_index, rng, ray, hit_1.t+0.0001, f32::INFINITY, &mut hit_2) {
            return false;
        }


        if hit_1.t < t_min { hit_1.t = t_min; };
        if t_max < hit_2.t { hit_2.t = t_max; };

        if hit_2.t <= hit_1.t { return false; }

        if hit_1.t < 0.0 { hit_1.t = 0.0; }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (hit_2.t - hit_1.t) * ray_length;
        let hit_distance = self.negative_inverse_density * rng.gen::<f32>().ln();

        if distance_inside_boundary < hit_distance { return false; }

        let t = hit_1.t + hit_distance / ray_length; 

        hit_out.t = t;
        hit_out.u = 0.0;
        hit_out.v = 0.0;
        hit_out.position = ray.at(t);
        hit_out.normal = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
        hit_out.is_front_face = true;
        hit_out.material = self.phase_function;

        true
    }

    fn bounding_box(&self, hittable_service: &HittableService, time_0: f32, time_1: f32, box_out: &mut AABB) -> bool {
        hittable_service.bounding_box(self.boundary_index, time_0, time_1, box_out)
        //self.boundary.bounding_box(time_0, time_1, box_out)
    }

    fn pdf_value(&self, _rng: &mut ThreadRng, _hittable_service: &HittableService, _origin: &Vector3, _v: &Vector3) -> f32 { 0.0 }

    fn random(&self, _rng: &mut ThreadRng, _hittable_service: &HittableService, _origin: &Vector3) -> Vector3 { Vector3::new(1.0, 0.0, 0.0) }

}
