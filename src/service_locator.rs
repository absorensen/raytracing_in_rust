use crate::{material_service::MaterialService, hittable_service::HittableService, texture_service::TextureService, scene_service::SceneService};

pub struct ServiceLocator {
    hittable_service: HittableService,
    material_service: MaterialService,
    texture_service: TextureService,
    scene_service: SceneService
}

impl ServiceLocator {
    pub fn new(scene_service: SceneService) -> Self {
        let hittable_service = HittableService::new();
        let material_service = MaterialService::new();
        let texture_service = TextureService::new();

        ServiceLocator{ hittable_service, material_service, texture_service, scene_service }
    }

    pub fn get_hittable_service(&self) -> &HittableService {
        &self.hittable_service
    }

    pub fn get_hittable_service_mut(&mut self) -> &mut HittableService {
        &mut self.hittable_service
    }

    pub fn get_material_service(&self) -> &MaterialService {
        &self.material_service
    }

    pub fn get_material_service_mut(&mut self) -> &mut MaterialService {
        &mut self.material_service
    }

    pub fn get_texture_service(&self) -> &TextureService {
        &self.texture_service
    }

    pub fn get_texture_service_mut(&mut self) -> &mut TextureService {
        &mut self.texture_service
    }

    pub fn get_scene_service(&self) -> &SceneService {
        &self.scene_service
    }

    pub fn _get_scene_service_mut(&mut self) -> &mut SceneService {
        &mut self.scene_service
    }

}