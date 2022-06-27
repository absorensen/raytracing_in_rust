use std::f64::consts::PI;
use std::sync::Arc;

use rand::{Rng, rngs::ThreadRng};

use crate::ortho_normal_base::OrthoNormalBase;
use crate::vector3::{Color, Vector3};
use crate::ray::Ray;
use crate::hittable::{HitRecord};
use crate::texture::{Texture, SolidColor};

pub trait Material : Sync + Send {
    fn scatter(&self, rng: &mut ThreadRng, ray:&Ray, hit: &HitRecord, albedo_out: &mut Color, ray_out: &mut Ray, pdf_out: &mut f64) -> bool;
    fn scattering_pdf(&self, rng: &mut ThreadRng, ray:&Ray, hit: &HitRecord, scattered:&Ray) -> f64;
    fn emitted(&self, ray:&Ray, hit: &HitRecord, u: f64, v: f64, point: &Vector3) -> Color;
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>
}

static INV_PI: f64 = 1.0 / PI;
static HALF_OVER_PI: f64 = 0.5 / PI;

impl Material for Lambertian {
    fn scatter(&self, rng: &mut ThreadRng, ray:&Ray, hit: &HitRecord, albedo_out: &mut Color, ray_out: &mut Ray, pdf_out: &mut f64) -> bool{
        let uvw = OrthoNormalBase::build_from_w(&hit.normal);
        let mut direction = uvw.local_vector(&Vector3::random_cosine_direction(rng));

        if direction.near_zero() { direction = hit.normal; }

        ray_out.origin = hit.position;
        ray_out.direction = direction.normalized();
        ray_out.time = ray.time;

        self.albedo.value(hit.u, hit.v, &hit.position, albedo_out);

        *pdf_out = Vector3::dot(&uvw.w, &ray_out.direction) / PI;

        return true;
    }

    fn scattering_pdf(&self, rng: &mut ThreadRng, ray:&Ray, hit: &HitRecord, scattered:&Ray) -> f64 {
        let cosine = Vector3::dot(&hit.normal, &(scattered.direction.normalized()));

        if cosine < 0.0 { 0.0 } else { cosine / PI }
    }

    fn emitted(&self, ray:&Ray, hit: &HitRecord, u: f64, v: f64, point: &Vector3) -> Color {
        Color{x: 0.0, y: 0.0, z: 0.0}
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64, // should be saturated to 1
}

impl Material for Metal {
    fn scatter(&self, rng: &mut ThreadRng, ray:&Ray, hit: &HitRecord, attenuation_out: &mut Color, ray_out: &mut Ray, pdf_out: &mut f64) -> bool {
        let mut reflected = Vector3::zero();
        Vector3::reflect(&ray.direction.normalized(), &hit.normal, &mut reflected);
        ray_out.origin = hit.position;
        ray_out.direction = reflected + self.fuzz * Vector3::random_in_unit_sphere(rng); 
        ray_out.time = ray.time;

        if 0.0 < Vector3::dot(&ray_out.direction, &hit.normal){
            *attenuation_out = self.albedo;
            return true;
        } else {
            return false;
        }
    }

    fn scattering_pdf(&self, rng: &mut ThreadRng, ray:&Ray, hit: &HitRecord, scattered:&Ray) -> f64 {
        0.0
    }

    fn emitted(&self, ray:&Ray, hit: &HitRecord, u: f64, v: f64, point: &Vector3) -> Color {
        Color{x: 0.0, y: 0.0, z: 0.0}
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub index_of_refraction: f64,
    pub inverse_index_of_refraction: f64,
}

impl Material for Dielectric {
    fn scatter(&self, rng: &mut ThreadRng, ray:&Ray, hit: &HitRecord, attenuation_out: &mut Color, ray_out: &mut Ray, pdf_out: &mut f64) -> bool {
        attenuation_out.x = 1.0;
        attenuation_out.y = 1.0;
        attenuation_out.z = 1.0;

        let refraction_ratio = if hit.is_front_face { self.inverse_index_of_refraction } else { self.index_of_refraction };
        let unit_direction = Vector3::normalized(&ray.direction);
        let cos_theta = Vector3::dot(&-unit_direction, &hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = 1.0 < refraction_ratio * sin_theta;
        let mut direction = Vector3::zero();
        if cannot_refract || rng.gen::<f64>() < Dielectric::reflectance(cos_theta, refraction_ratio) {
            Vector3::reflect(&unit_direction, &hit.normal, &mut direction);
        } else {
            Vector3::refract(&unit_direction, &hit.normal, refraction_ratio, &mut direction);
        };

        ray_out.origin = hit.position;
        ray_out.direction = direction;

        true
    }

    fn scattering_pdf(&self, rng: &mut ThreadRng, ray:&Ray, hit: &HitRecord, scattered:&Ray) -> f64 {
        0.0
    }

    fn emitted(&self, ray:&Ray, hit: &HitRecord, u: f64, v: f64, point: &Vector3) -> Color {
        Color{x: 0.0, y: 0.0, z: 0.0}
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, index_of_refraction: f64) -> f64 {
        let r0 = (1.0 - index_of_refraction) / (1.0 + index_of_refraction);
        let r0_squared = r0 * r0;
        let inverse_cosine = 1.0 - cosine;
        let inverse_cosine_squared = inverse_cosine * inverse_cosine;
        r0_squared + (1.0 - r0_squared) * inverse_cosine_squared * inverse_cosine_squared * inverse_cosine
    }
}

pub struct DiffuseLight {
    pub emission: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn from_texture(texture: &Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { emission: Arc::clone(texture) }
    }

    pub fn from_color(color: &Color) -> DiffuseLight {
        let texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(color));
        DiffuseLight { emission: texture }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _rng: &mut ThreadRng, _ray:&Ray, _hit: &HitRecord, _attenuation_out: &mut Color, _ray_out: &mut Ray, pdf_out: &mut f64) -> bool{
        false
    }

    fn scattering_pdf(&self, rng: &mut ThreadRng, ray:&Ray, hit: &HitRecord, scattered:&Ray) -> f64 {
        0.0
    }

    fn emitted(&self, ray:&Ray, hit: &HitRecord, u: f64, v: f64, point: &Vector3) -> Color {
        if hit.is_front_face {
            let mut color_out = Color::zero();
            self.emission.value(u, v, point, &mut color_out);
            return color_out;
        }
        Color{x: 0.0, y: 0.0, z: 0.0}
    }
}

pub struct Isotropic {
    pub albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn from_texture(texture: &Arc<dyn Texture>) -> Isotropic {
        Isotropic { albedo: Arc::clone(texture) }
    }

    pub fn from_color(color: &Color) -> Isotropic {
        let texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(color));
        Isotropic { albedo: texture }
    }
}

impl Material for Isotropic {
    fn scatter(&self, rng: &mut ThreadRng, ray:&Ray, hit: &HitRecord, attenuation_out: &mut Color, ray_out: &mut Ray, pdf_out: &mut f64) -> bool{
        *ray_out = Ray{ origin: hit.position, direction: Vector3::random_in_unit_sphere(rng), time: ray.time };
        self.albedo.value(hit.u, hit.v, &hit.position, attenuation_out)
    }

    fn scattering_pdf(&self, rng: &mut ThreadRng, ray:&Ray, hit: &HitRecord, scattered:&Ray) -> f64 {
        0.0
    }

    fn emitted(&self, ray:&Ray, hit: &HitRecord, u: f64, v: f64, point: &Vector3) -> Color {
        Color{x: 0.0, y: 0.0, z: 0.0}
    }
}