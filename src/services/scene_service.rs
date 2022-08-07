use crate::{scene::camera::Camera, core::color_rgb::ColorRGB};

pub struct SceneService {
    camera: Camera,
    background: ColorRGB,
    // config loaded from config file goes here
    // dynamic global state goes here
}

impl SceneService {
    pub fn new(camera: Camera, background: ColorRGB) -> SceneService {
        SceneService{ camera, background }
    }

    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn get_background(&self) -> &ColorRGB {
        &self.background
    }
}
