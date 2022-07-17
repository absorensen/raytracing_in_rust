use std::f64::consts::PI;
use std::sync::Arc;

use rand::{Rng, rngs::ThreadRng};

use crate::pdf::{PDF, CosinePDF};
use crate::vector3::{Color, Vector3};
use crate::ray::Ray;
use crate::hittable::{HitRecord};
use crate::texture::{Texture, SolidColor};

pub enum MaterialEnum {
    DefaultMaterial(DefaultMaterial),
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
    Isotropic(Isotropic),
}

pub struct MaterialService {
    materials: Vec<MaterialEnum>,
}

impl MaterialService {
    pub fn new() -> MaterialService {
        let mut service = MaterialService{ materials : Vec::new() };
        
        service.add_material(MaterialEnum::DefaultMaterial(DefaultMaterial{}));

        service
    }

    pub fn add_material(&mut self, new_material: MaterialEnum) -> usize {
        self.materials.push(new_material);

        self.materials.len() - 1
    }

    #[inline]
    pub fn fetch_material(&self, index: usize) -> &MaterialEnum {
        &self.materials[index]
    }

    #[inline]
    pub fn emission(&self, ray: &Ray, hit: &HitRecord, u: f64, v: f64, point: &Vector3) -> Color {
        match &self.materials[hit.material] {
            MaterialEnum::DefaultMaterial(default) => default.emitted(ray, hit, u, v, point),
            MaterialEnum::Lambertian(lambertian) => lambertian.emitted(ray, hit, u, v, point),
            MaterialEnum::Metal(metal) => metal.emitted(ray, hit, u, v, point),
            MaterialEnum::Dielectric(dielectric) => dielectric.emitted(ray, hit, u, v, point),
            MaterialEnum::DiffuseLight(diffuse_light) => diffuse_light.emitted(ray, hit, u, v, point),
            MaterialEnum::Isotropic(isotropic) => isotropic.emitted(ray, hit, u, v, point),
        }
    }

    #[inline]
    pub fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, hit: &HitRecord, scatter_out: &mut ScatterRecord) -> bool {
        match &self.materials[hit.material] {
            MaterialEnum::DefaultMaterial(default) => default.scatter(rng, ray, &hit, scatter_out),
            MaterialEnum::Lambertian(lambertian) => lambertian.scatter(rng, ray, &hit, scatter_out),
            MaterialEnum::Metal(metal) => metal.scatter(rng, ray, &hit, scatter_out),
            MaterialEnum::Dielectric(dielectric) => dielectric.scatter(rng, ray, &hit, scatter_out),
            MaterialEnum::DiffuseLight(diffuse_light) => diffuse_light.scatter(rng, ray, &hit, scatter_out),
            MaterialEnum::Isotropic(isotropic) => isotropic.scatter(rng, ray, &hit, scatter_out),
        }
    }

    #[inline]
    pub fn scattering_pdf(&self, rng: &mut ThreadRng, ray: &Ray, hit: &HitRecord, scattered_ray:&Ray) -> f64 {
        match &self.materials[hit.material] {
            MaterialEnum::DefaultMaterial(default) => default.scattering_pdf(rng, ray, hit, scattered_ray),
            MaterialEnum::Lambertian(lambertian) => lambertian.scattering_pdf(rng, ray, hit, scattered_ray),
            MaterialEnum::Metal(metal) => metal.scattering_pdf(rng, ray, hit, scattered_ray),
            MaterialEnum::Dielectric(dielectric) => dielectric.scattering_pdf(rng, ray, hit, scattered_ray),
            MaterialEnum::DiffuseLight(diffuse_light) => diffuse_light.scattering_pdf(rng, ray, hit, scattered_ray),
            MaterialEnum::Isotropic(isotropic) => isotropic.scattering_pdf(rng, ray, hit, scattered_ray),
        }
    }

}

pub struct ScatterRecord {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Color,
    pub pdf: Option<Box<dyn PDF>>, // Try to remove this arc
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
    fn emitted(&self, _ray:&Ray, _hit: &HitRecord, _u: f64, _v: f64, _point: &Vector3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scatter(&self, _rng: &mut ThreadRng, _ray:&Ray, _hit: &HitRecord, _scatter_out: &mut ScatterRecord) -> bool {
        false
    }

    fn scattering_pdf(&self, _rng: &mut ThreadRng, _ray: &Ray, _hit: &HitRecord, _scattered_ray: &Ray) -> f64 {
        0.0
    }
}

pub struct DefaultMaterial {

}

impl Material for DefaultMaterial {}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>
}

impl Material for Lambertian {
    fn scatter(&self, _rng: &mut ThreadRng, _ray:&Ray, hit: &HitRecord, scatter_out: &mut ScatterRecord) -> bool{
        scatter_out.is_specular = false;
        self.albedo.value(hit.u, hit.v, &hit.position, &mut scatter_out.attenuation);
        scatter_out.pdf = Some(Box::new(CosinePDF::new(&hit.normal)));

        return true;
    }

    fn scattering_pdf(&self, _rng: &mut ThreadRng, _ray: &Ray, hit: &HitRecord, scattered_ray:&Ray) -> f64 {
        let cosine = Vector3::dot(&hit.normal, &(scattered_ray.direction.normalized()));

        if cosine < 0.0 { 0.0 } else { cosine / PI }
    }

}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64, // should be saturated to 1
}

impl Metal {
    pub fn new(albedo: Color, fuzz:f64) -> Metal {
        Metal { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } }
    }
}

impl Material for Metal {
    fn scatter(&self, rng: &mut ThreadRng, ray:&Ray, hit: &HitRecord, scatter_out: &mut ScatterRecord) -> bool {
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
    pub index_of_refraction: f64,
    pub inverse_index_of_refraction: f64,
}

impl Material for Dielectric {
    fn scatter(&self, rng: &mut ThreadRng, ray:&Ray, hit: &HitRecord, scatter_out: &mut ScatterRecord) -> bool {
        scatter_out.is_specular = true;
        scatter_out.pdf = None;
        scatter_out.attenuation = Color::new(1.0, 1.0, 1.0);

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

        scatter_out.specular_ray = Ray::new(hit.position, direction, ray.time);

        true
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
    fn emitted(&self, _ray:&Ray, hit: &HitRecord, u: f64, v: f64, point: &Vector3) -> Color {
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
    fn scatter(&self, rng: &mut ThreadRng, ray:&Ray, hit: &HitRecord, scatter_out: &mut ScatterRecord) -> bool{
        scatter_out.is_specular = true;
        scatter_out.specular_ray = Ray{ origin: hit.position, direction: Vector3::random_in_unit_sphere(rng), time: ray.time };
        self.albedo.value(hit.u, hit.v, &hit.position, &mut scatter_out.attenuation)
    }

    fn emitted(&self, ray:&Ray, hit: &HitRecord, u: f64, v: f64, point: &Vector3) -> Color {
        Color{x: 0.0, y: 0.0, z: 0.0}
    }
}