use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RenderConfig {
    pub aspect_ratio: f32,
    pub image_width: usize,
    pub output_path: String,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub scene_index: usize,
}

impl ::std::default::Default for RenderConfig {
    fn default() -> 
        Self { 
            Self { 
                aspect_ratio: 16.0 / 9.0, 
                image_width: 500, 
                output_path: "output.png".to_string(), 
                samples_per_pixel: 5, 
                max_depth: 10, 
                scene_index: 7
            } 
        }
}