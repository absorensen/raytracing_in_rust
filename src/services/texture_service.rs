use crate::{textures::{texture_enum::TextureEnum, default_texture::DefaultTexture, texture::Texture}, core::color_rgb::ColorRGB};
use ultraviolet::Vec3;

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
    pub fn _fetch_texture(&self, index: usize) -> &TextureEnum {
        &self.textures[index]
    }

    #[inline]
    pub fn value(&self, texture_index: usize, u: f32, v: f32, p: &Vec3, color_out: &mut ColorRGB) -> bool {
        self.textures[texture_index].value(&self, u, v, p, color_out)
    }

}
