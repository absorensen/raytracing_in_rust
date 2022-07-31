use rand::rngs::ThreadRng;

use crate::{vector3::{Vector3, Color}, perlin::Perlin, texture_service::TextureService};

pub trait Texture : Sync + Send {
    fn value(&self, texture_service: &TextureService, u: f64, v: f64, p: &Vector3, color_out: &mut Color) -> bool;
}

pub struct DefaultTexture {
}

impl DefaultTexture {
    pub fn _default() -> Self {
        DefaultTexture{}
    }
}

impl Texture for DefaultTexture {
    fn value(&self, _texture_service: &TextureService, _u: f64, _v: f64, _p: &Vector3, color_out: &mut Color) -> bool {
        color_out.x = 1.0;
        color_out.y = 0.0;
        color_out.z = 0.0;

        true
    }
}



pub struct SolidColorTexture {
    color: Color,
}

impl SolidColorTexture {
    pub fn from_color(color: &Color) -> Self {
        SolidColorTexture{ color: color.clone() }
    }
}

impl Texture for SolidColorTexture {
    fn value(&self, _texture_service: &TextureService, _u: f64, _v: f64, _p: &Vector3, color_out: &mut Color) -> bool {
        color_out.x = self.color.x;
        color_out.y = self.color.y;
        color_out.z = self.color.z;

        true
    }
}

pub struct CheckerTexture {
    odd: usize,
    even: usize,
}

impl CheckerTexture {
    pub fn new(odd: usize, even: usize) -> Self {
        CheckerTexture{odd, even}
    }

}

impl Texture for CheckerTexture {
    fn value(&self, texture_service: &TextureService, u: f64, v: f64, p: &Vector3, color_out: &mut Color) -> bool {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            return texture_service.value(self.odd, u, v, p, color_out);
        } else {
            return texture_service.value(self.even, u, v, p, color_out);
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
    fn value(&self, _texture_service: &TextureService, _u: f64, _v: f64, point: &Vector3, color_out: &mut Color) -> bool {
        color_out.x = 1.0;
        color_out.y = 1.0;
        color_out.z = 1.0;

        *color_out = *color_out * 0.5 * (1.0 + (self.scale * point.z + 10.0 * self.perlin.turbulence_default(point)).sin());
        true
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
    bytes_per_pixel: usize,
    bytes_per_scanline: usize,
}

impl ImageTexture {
    pub fn new(path: &str) -> Self {
        let bytes_per_pixel: usize = 3;

        let image = image::open(path).expect("image not found").flipv().to_rgb8();
        let (width, height) = image.dimensions();
        let data = image.into_raw();

        ImageTexture{data, width: width as usize, height: height as usize, bytes_per_pixel, bytes_per_scanline: bytes_per_pixel * width as usize}
    }
}

impl Texture for ImageTexture {
    fn value(&self, _texture_service: &TextureService, u: f64, v: f64, _point: &Vector3, color_out: &mut Color) -> bool {
        if self.data.len() < 1 {
            color_out.x = 0.0;
            color_out.y = 1.0;
            color_out.z = 1.0;
            
            return false;
        }

        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        let mut i = ( u * self.width as f64 ) as usize;
        let mut j = ( v * self.height as f64 ) as usize;

        if self.width <= i { i = self.width - 1; }
        if self.height <= j { j = self.height - 1; } 

        let color_scale = 1.0 / 255.0;
        let pixel_index = j * self.bytes_per_scanline + i * self.bytes_per_pixel;

        color_out.x = color_scale * self.data[pixel_index    ] as f64;
        color_out.y = color_scale * self.data[pixel_index + 1] as f64;
        color_out.z = color_scale * self.data[pixel_index + 2] as f64;


        true
    }
}