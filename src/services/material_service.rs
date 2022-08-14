use rand::prelude::ThreadRng;

use crate::core::color_rgb::ColorRGB;
use crate::hittables::hit_record::HitRecord;
use crate::materials::default_material::DefaultMaterial;
use crate::materials::material::Material;
use crate::materials::material_enum::MaterialEnum;
use crate::materials::scatter_record::ScatterRecord;
use crate::core::ray::Ray;
use crate::services::texture_service::TextureService;
use crate::math::vector3::{Vector3};

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
    pub fn emitted(&self, texture_service: &TextureService, ray: &Ray, hit: &HitRecord, u: f32, v: f32, point: &Vector3) -> ColorRGB {
        self.materials[hit.material].emitted(texture_service, ray, hit, u, v, point)
    }

    #[inline]
    pub fn scatter(&self, rng: &mut ThreadRng, texture_service: &TextureService, ray: &Ray, hit: &HitRecord, scatter_out: &mut ScatterRecord) -> bool {
        self.materials[hit.material].scatter(rng, texture_service, ray, hit, scatter_out)
    }

    #[inline]
    pub fn scattering_pdf(&self, rng: &mut ThreadRng, ray: &Ray, hit: &HitRecord, scattered_ray:&Ray) -> f32 {
        self.materials[hit.material].scattering_pdf(rng, ray, hit, scattered_ray)
    }

}
