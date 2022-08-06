use std::f32::consts::PI;

use rand::{Rng, rngs::ThreadRng};

use crate::pdf::{PDF, CosinePDF};
use crate::texture_service::TextureService;
use crate::vector3::{Color, Vector3};
use crate::ray::Ray;
use crate::hittable::{HitRecord};

pub struct ScatterRecord {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Color,
    pub pdf: Option<Box<dyn PDF>>, // Try to remove this box
}

impl ScatterRecord {
    pub fn default() -> ScatterRecord {
        ScatterRecord { 
            specular_ray: Ray::default(), 
            is_specular: false, 
            attenuation: Color::default(), 
            pdf: None
        }
    }
}

pub trait Material : Sync + Send {
    fn emitted(&self, _texture_service: &TextureService, _ray:&Ray, _hit: &HitRecord, _u: f32, _v: f32, _point: &Vector3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scatter(&self, _rng: &mut ThreadRng, _texture_service: &TextureService, _ray:&Ray, _hit: &HitRecord, _scatter_out: &mut ScatterRecord) -> bool {
        false
    }

    fn scattering_pdf(&self, _rng: &mut ThreadRng, _ray: &Ray, _hit: &HitRecord, _scattered_ray: &Ray) -> f32 {
        0.0
    }
}

pub struct DefaultMaterial {

}

impl Material for DefaultMaterial {}

pub struct Lambertian {
    pub albedo_texture_index: usize
}

impl Lambertian {
    pub fn new(albedo_index: usize) -> Self {
        Lambertian { albedo_texture_index: albedo_index }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _rng: &mut ThreadRng, texture_service: &TextureService, _ray:&Ray, hit: &HitRecord, scatter_out: &mut ScatterRecord) -> bool {
        scatter_out.is_specular = false;
        texture_service.value(self.albedo_texture_index, hit.u, hit.v, &hit.position, &mut scatter_out.attenuation);
        scatter_out.pdf = Some(Box::new(CosinePDF::new(&hit.normal)));

        return true;
    }

    fn scattering_pdf(&self, _rng: &mut ThreadRng, _ray: &Ray, hit: &HitRecord, scattered_ray:&Ray) -> f32 {
        let cosine = Vector3::dot(&hit.normal, &(scattered_ray.direction.normalized()));

        if cosine < 0.0 { 0.0 } else { cosine / PI }
    }

}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32, // should be saturated to 1
}

impl Metal {
    pub fn new(albedo: Color, fuzz:f32) -> Metal {
        Metal { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } }
    }
}

impl Material for Metal {
    fn scatter(&self, rng: &mut ThreadRng, _texture_service: &TextureService, ray:&Ray, hit: &HitRecord, scatter_out: &mut ScatterRecord) -> bool {
        let mut reflected = Vector3::default(); 
        Vector3::reflect(&ray.direction.normalized(), &hit.normal, &mut reflected);
        scatter_out.specular_ray = Ray::new(hit.position, reflected + self.fuzz * Vector3::random_in_unit_sphere(rng), ray.time);
        scatter_out.attenuation = self.albedo;
        scatter_out.is_specular = true;
        scatter_out.pdf = None;
        true
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub index_of_refraction: f32,
    pub inverse_index_of_refraction: f32,
}

impl Material for Dielectric {
    fn scatter(&self, rng: &mut ThreadRng, _texture_service: &TextureService, ray:&Ray, hit: &HitRecord, scatter_out: &mut ScatterRecord) -> bool {
        scatter_out.is_specular = true;
        scatter_out.pdf = None;
        scatter_out.attenuation = Color::new(1.0, 1.0, 1.0);

        let refraction_ratio = if hit.is_front_face { self.inverse_index_of_refraction } else { self.index_of_refraction };
        let unit_direction = Vector3::normalized(&ray.direction);
        let cos_theta = Vector3::dot(&-unit_direction, &hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = 1.0 < refraction_ratio * sin_theta;
        let mut direction = Vector3::zero();
        if cannot_refract || rng.gen::<f32>() < Dielectric::reflectance(cos_theta, refraction_ratio) {
            Vector3::reflect(&unit_direction, &hit.normal, &mut direction);
        } else {
            Vector3::refract(&unit_direction, &hit.normal, refraction_ratio, &mut direction);
        };

        scatter_out.specular_ray = Ray::new(hit.position, direction, ray.time);

        true
    }

}

impl Dielectric {
    fn reflectance(cosine: f32, index_of_refraction: f32) -> f32 {
        let r0 = (1.0 - index_of_refraction) / (1.0 + index_of_refraction);
        let r0_squared = r0 * r0;
        let inverse_cosine = 1.0 - cosine;
        let inverse_cosine_squared = inverse_cosine * inverse_cosine;
        r0_squared + (1.0 - r0_squared) * inverse_cosine_squared * inverse_cosine_squared * inverse_cosine
    }
}

pub struct DiffuseLight {
    pub emission_texture_index: usize,
}

impl DiffuseLight {
    pub fn new(texture_index: usize) -> DiffuseLight {
        DiffuseLight { emission_texture_index: texture_index }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, texture_service: &TextureService, _ray:&Ray, hit: &HitRecord, u: f32, v: f32, point: &Vector3) -> Color {
        if hit.is_front_face {
            let mut color_out = Color::zero();
            texture_service.value(self.emission_texture_index, u, v, point, &mut color_out);
            return color_out;
        }
        Color{x: 0.0, y: 0.0, z: 0.0}
    }
}

pub struct Isotropic {
    pub albedo_texture_index: usize,
}

impl Isotropic {
    pub fn new(texture_index: usize) -> Isotropic {
        Isotropic { albedo_texture_index: texture_index }
    }
}

impl Material for Isotropic {
    fn scatter(&self, rng: &mut ThreadRng, texture_service: &TextureService, ray:&Ray, hit: &HitRecord, scatter_out: &mut ScatterRecord) -> bool{
        scatter_out.is_specular = true;
        scatter_out.specular_ray = Ray{ origin: hit.position, direction: Vector3::random_in_unit_sphere(rng), time: ray.time };
        texture_service.value(self.albedo_texture_index, hit.u, hit.v, &hit.position, &mut scatter_out.attenuation)
    }
}