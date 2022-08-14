use crate::{core::{ray::Ray, color_rgb::ColorRGB}, pdfs::pdf::{PDF, PDFEnum}};

pub struct ScatterRecord {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: ColorRGB,
    pub pdf: PDFEnum, // Try to remove this box
}

impl ScatterRecord {
    pub fn default() -> ScatterRecord {
        ScatterRecord { 
            specular_ray: Ray::default(), 
            is_specular: false, 
            attenuation: ColorRGB::default(), 
            pdf: PDFEnum::None(),
        }
    }
}