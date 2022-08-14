use rand::{rngs::ThreadRng, Rng};

use crate::{services::texture_service::TextureService, core::{ray::Ray, color_rgb::ColorRGB}, hittables::hit_record::HitRecord, math::vector3::{Vector3}, pdfs::pdf::PDFEnum};

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
        let unit_direction = Vector3::get_normalized(&ray.direction);
        let cos_theta = Vector3::dot(&-unit_direction, &hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = 1.0 < refraction_ratio * sin_theta;
        let mut direction = Vector3::zero();
        if cannot_refract || rng.gen::<f32>() < Dielectric::reflectance(cos_theta, refraction_ratio) {
            Vector3::reflect(&unit_direction, &hit.normal, &mut direction);
        } else {
            Vector3::refract(&unit_direction, &hit.normal, refraction_ratio, &mut direction);
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
