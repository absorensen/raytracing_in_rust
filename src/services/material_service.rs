use rand::prelude::ThreadRng;

use crate::materials::material::{DefaultMaterial, Lambertian, Metal, Dielectric, DiffuseLight, Isotropic, ScatterRecord, Material};
use crate::ray::Ray;
use crate::hittables::hittable::HitRecord;
use crate::services::texture_service::TextureService;
use crate::math::vector3::{Vector3, Color};

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
    pub fn emission(&self, texture_service: &TextureService, ray: &Ray, hit: &HitRecord, u: f32, v: f32, point: &Vector3) -> Color {
        match &self.materials[hit.material] {
            MaterialEnum::DefaultMaterial(default) => default.emitted(texture_service, ray, hit, u, v, point),
            MaterialEnum::Lambertian(lambertian) => lambertian.emitted(texture_service, ray, hit, u, v, point),
            MaterialEnum::Metal(metal) => metal.emitted(texture_service, ray, hit, u, v, point),
            MaterialEnum::Dielectric(dielectric) => dielectric.emitted(texture_service, ray, hit, u, v, point),
            MaterialEnum::DiffuseLight(diffuse_light) => diffuse_light.emitted(texture_service, ray, hit, u, v, point),
            MaterialEnum::Isotropic(isotropic) => isotropic.emitted(texture_service, ray, hit, u, v, point),
        }
    }

    #[inline]
    pub fn scatter(&self, rng: &mut ThreadRng, texture_service: &TextureService, ray: &Ray, hit: &HitRecord, scatter_out: &mut ScatterRecord) -> bool {
        match &self.materials[hit.material] {
            MaterialEnum::DefaultMaterial(default) => default.scatter(rng, texture_service, ray, &hit, scatter_out),
            MaterialEnum::Lambertian(lambertian) => lambertian.scatter(rng, texture_service, ray, &hit, scatter_out),
            MaterialEnum::Metal(metal) => metal.scatter(rng, texture_service, ray, &hit, scatter_out),
            MaterialEnum::Dielectric(dielectric) => dielectric.scatter(rng, texture_service, ray, &hit, scatter_out),
            MaterialEnum::DiffuseLight(diffuse_light) => diffuse_light.scatter(rng, texture_service, ray, &hit, scatter_out),
            MaterialEnum::Isotropic(isotropic) => isotropic.scatter(rng, texture_service, ray, &hit, scatter_out),
        }
    }

    #[inline]
    pub fn scattering_pdf(&self, rng: &mut ThreadRng, ray: &Ray, hit: &HitRecord, scattered_ray:&Ray) -> f32 {
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
