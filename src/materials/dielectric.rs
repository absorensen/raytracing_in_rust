use ultraviolet::Vec3;
use rand::{rngs::ThreadRng, Rng};

use crate::{services::texture_service::TextureService, core::{ray::Ray, color_rgb::ColorRGB}, hittables::hit_record::HitRecord, pdfs::pdf_enum::PDFEnum, math::utility::{reflect, refract}};

use super::{material::Material, scatter_record::ScatterRecord};

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub index_of_refraction: f32,
    pub inverse_index_of_refraction: f32,
}

impl Material for Dielectric {
    fn scatter(&self, rng: &mut ThreadRng, _texture_service: &TextureService, ray:&Ray, hit: &HitRecord, scatter_out: &mut ScatterRecord) -> bool {
        scatter_out.is_specular = true;
        scatter_out.pdf = PDFEnum::None();
        scatter_out.attenuation = ColorRGB::new(1.0, 1.0, 1.0);

        let refraction_ratio = if hit.is_front_face { self.inverse_index_of_refraction } else { self.index_of_refraction };
        let unit_direction: Vec3 = ray.direction.normalized();
        let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = 1.0 < refraction_ratio * sin_theta;
        let mut direction: Vec3 = Vec3::zero();
        if cannot_refract || rng.gen::<f32>() < Dielectric::reflectance(cos_theta, refraction_ratio) {
            reflect(&unit_direction, &hit.normal, &mut direction);
        } else {
            refract(&unit_direction, &hit.normal, refraction_ratio, &mut direction);
        };

        scatter_out.specular_ray = Ray::new_normalized(hit.position, direction, ray.time);

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
