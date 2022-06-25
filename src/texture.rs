use std::sync::Arc;

use rand::rngs::ThreadRng;

use crate::{vector3::{Vector3, Color}, perlin::Perlin};

pub trait Texture : Sync + Send {
    fn value(&self, u: f64, v: f64, p: &Vector3, color_out: &mut Color) -> bool;
}


pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn default() -> Self {
        SolidColor{color: Color{x: 0.0, y: 0.0, z: 0.0}}
    }

    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        SolidColor{color: Color{x: red, y: blue, z: green}}
    }

    pub fn from_color(color: &Color) -> Self {
        SolidColor{ color: color.clone() }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Vector3, color_out: &mut Color) -> bool {
        color_out.x = self.color.x;
        color_out.y = self.color.y;
        color_out.z = self.color.z;

        true
    }
}

pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn from_colors(c1: &Color, c2: &Color) -> Self {
        let odd: Arc<dyn Texture> = Arc::new(SolidColor::from_color(c1));
        let even: Arc<dyn Texture> = Arc::new(SolidColor::from_color(c2));
        CheckerTexture{odd, even}
    }

    pub fn from_texture(odd: &Arc<dyn Texture>, even: &Arc<dyn Texture>) -> Self {
        CheckerTexture { odd: Arc::clone(odd), even: Arc::clone(even) }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vector3, color_out: &mut Color) -> bool {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, p, color_out)
        } else {
            return self.even.value(u, v, p, color_out)
        }
    }
}

pub struct NoiseTexture {
    perlin: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(rng: &mut ThreadRng, point_count: u32, scale: f64) -> Self {
        NoiseTexture{perlin: Perlin::new(rng, point_count), scale}
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, point: &Vector3, color_out: &mut Color) -> bool {
        color_out.x = 1.0;
        color_out.y = 1.0;
        color_out.z = 1.0;

        *color_out = *color_out * 0.5 * (1.0 + (self.scale * point.z + 10.0 * self.perlin.turbulence_default(point)).sin());
        true
    }
}