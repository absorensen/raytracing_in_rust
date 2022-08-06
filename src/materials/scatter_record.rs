use crate::{ray::Ray, math::vector3::Color, pdfs::pdf::PDF};

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