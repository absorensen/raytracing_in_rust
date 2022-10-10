use ultraviolet::Vec3;
use rand::rngs::ThreadRng;

use crate::{services::texture_service::TextureService, core::{ray::Ray, color_rgb::ColorRGB}, hittables::hit_record::HitRecord, pdfs::pdf_enum::PDFEnum, math::utility::{reflect, random_in_unit_sphere}};

use super::{material::Material, scatter_record::ScatterRecord};

pub struct Metal {
    pub albedo: ColorRGB,
    pub fuzz: f32, // should be saturated to 1
}

impl Metal {
    pub fn new(albedo: ColorRGB, fuzz:f32) -> Metal {
        Metal { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } }
    }
}

impl Material for Metal {
    fn scatter(&self, rng: &mut ThreadRng, _texture_service: &TextureService, ray:&Ray, hit: &HitRecord, scatter_out: &mut ScatterRecord) -> bool {
        let mut reflected: Vec3 = Vec3::default(); 
        reflect(&ray.direction.normalized(), &hit.normal, &mut reflected);
        scatter_out.specular_ray = Ray::new_normalized(hit.position, reflected + random_in_unit_sphere(rng) * self.fuzz, ray.time);
        scatter_out.attenuation = self.albedo;
        scatter_out.is_specular = true;
        scatter_out.pdf = PDFEnum::None();
        true
    }
}
