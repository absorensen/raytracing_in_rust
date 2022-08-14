use rand::rngs::ThreadRng;

use crate::{services::texture_service::TextureService, core::{ray::Ray, color_rgb::ColorRGB}, hittables::hit_record::HitRecord, math::vector3::Vector3};

use super::{default_material::DefaultMaterial, lambertian::Lambertian, metal::Metal, dielectric::Dielectric, diffuse_light::DiffuseLight, isotropic::Isotropic, material::Material, scatter_record::ScatterRecord};

pub enum MaterialEnum {
    DefaultMaterial(DefaultMaterial),
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
    Isotropic(Isotropic),
}

impl Material for MaterialEnum {
    #[inline]
    fn emitted(&self, texture_service: &TextureService, ray:&Ray, hit: &HitRecord, u: f32, v: f32, point: &Vector3) -> ColorRGB {
        match self {
            MaterialEnum::DefaultMaterial(default) => default.emitted(texture_service, ray, hit, u, v, point),
            MaterialEnum::Lambertian(lambertian) => lambertian.emitted(texture_service, ray, hit, u, v, point),
            MaterialEnum::Metal(metal) => metal.emitted(texture_service, ray, hit, u, v, point),
            MaterialEnum::Dielectric(dielectric) => dielectric.emitted(texture_service, ray, hit, u, v, point),
            MaterialEnum::DiffuseLight(diffuse_light) => diffuse_light.emitted(texture_service, ray, hit, u, v, point),
            MaterialEnum::Isotropic(isotropic) => isotropic.emitted(texture_service, ray, hit, u, v, point),
        }
    }

    #[inline]
    fn scatter(&self, rng: &mut ThreadRng, texture_service: &TextureService, ray:&Ray, hit: &HitRecord, scatter_out: &mut ScatterRecord) -> bool {
        match self {
            MaterialEnum::DefaultMaterial(default) => default.scatter(rng, texture_service, ray, &hit, scatter_out),
            MaterialEnum::Lambertian(lambertian) => lambertian.scatter(rng, texture_service, ray, &hit, scatter_out),
            MaterialEnum::Metal(metal) => metal.scatter(rng, texture_service, ray, &hit, scatter_out),
            MaterialEnum::Dielectric(dielectric) => dielectric.scatter(rng, texture_service, ray, &hit, scatter_out),
            MaterialEnum::DiffuseLight(diffuse_light) => diffuse_light.scatter(rng, texture_service, ray, &hit, scatter_out),
            MaterialEnum::Isotropic(isotropic) => isotropic.scatter(rng, texture_service, ray, &hit, scatter_out),
        }
    }

    #[inline]
    fn scattering_pdf(&self, rng: &mut ThreadRng, ray: &Ray, hit: &HitRecord, scattered_ray: &Ray) -> f32 {
        match self {
            MaterialEnum::DefaultMaterial(default) => default.scattering_pdf(rng, ray, hit, scattered_ray),
            MaterialEnum::Lambertian(lambertian) => lambertian.scattering_pdf(rng, ray, hit, scattered_ray),
            MaterialEnum::Metal(metal) => metal.scattering_pdf(rng, ray, hit, scattered_ray),
            MaterialEnum::Dielectric(dielectric) => dielectric.scattering_pdf(rng, ray, hit, scattered_ray),
            MaterialEnum::DiffuseLight(diffuse_light) => diffuse_light.scattering_pdf(rng, ray, hit, scattered_ray),
            MaterialEnum::Isotropic(isotropic) => isotropic.scattering_pdf(rng, ray, hit, scattered_ray),
        }
    }
}