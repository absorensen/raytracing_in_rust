use crate::{camera::Camera, vector3::Color};

pub struct SceneService {
    camera: Camera,
    background: Color,
    // config loaded from config file goes here
    // dynamic global state goes here
}

impl SceneService {
    pub fn new(camera: Camera, background: Color) -> SceneService {
        SceneService{ camera, background }
    }

    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn get_background(&self) -> &Color {
        &self.background
    }
}
