use crate::texture::{Texture, DefaultTexture, SolidColorTexture, CheckerTexture, NoiseTexture, ImageTexture};
use crate::vector3::{Vector3, Color};

pub enum TextureEnum {
    DefaultTexture(DefaultTexture),
    SolidColorTexture(SolidColorTexture),
    CheckerTexture(CheckerTexture),
    NoiseTexture(NoiseTexture),
    ImageTexture(ImageTexture),
}

pub struct TextureService {
    textures: Vec<TextureEnum>,
}

impl TextureService {
    pub fn new() -> TextureService {
        let mut service = TextureService{ textures : Vec::new() };
        
        service.add_texture(TextureEnum::DefaultTexture(DefaultTexture{}));

        service
    }

    pub fn add_texture(&mut self, new_texture: TextureEnum) -> usize {
        self.textures.push(new_texture);

        self.textures.len() - 1
    }

    #[inline]
    pub fn fetch_texture(&self, index: usize) -> &TextureEnum {
        &self.textures[index]
    }

    #[inline]
    fn value(&self, texture_index: usize, u: f64, v: f64, p: &Vector3, color_out: &mut Color) -> bool {
        match &self.textures[texture_index] {
            TextureEnum::DefaultTexture(default) => default.value(u, v, p, color_out),
            TextureEnum::SolidColorTexture(solid_color) => solid_color.value(u, v, p, color_out),
            TextureEnum::CheckerTexture(checker) => checker.value(u, v, p, color_out),
            TextureEnum::NoiseTexture(noise) => noise.value(u, v, p, color_out),
            TextureEnum::ImageTexture(image) => image.value(u, v, p, color_out),
        }
    }
}
