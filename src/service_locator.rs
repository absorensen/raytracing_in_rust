use crate::{material_service::MaterialService, hittable_service::HittableService, texture_service::TextureService};

pub struct ServiceLocator {
    material_service: MaterialService,
    hittable_service: HittableService,
    texture_service: TextureService,
}

impl ServiceLocator {
    pub fn new() -> Self {
        let material_service = MaterialService::new();
        let hittable_service = HittableService::new();
        let texture_service = TextureService::new();

        ServiceLocator{ material_service, hittable_service, texture_service }
    }

    pub fn get_material_service(&self) -> &MaterialService {
        &self.material_service
    }

    pub fn get_material_service_mut(&mut self) -> &mut MaterialService {
        &mut self.material_service
    }

    pub fn get_hittable_service(&self) -> &HittableService {
        &self.hittable_service
    }

    pub fn get_hittable_service_mut(&mut self) -> &mut HittableService {
        &mut self.hittable_service
    }

    pub fn get_texture_service(&self) -> &TextureService {
        &self.texture_service
    }

    pub fn get_texture_service_mut(&mut self) -> &mut TextureService {
        &mut self.texture_service
    }

}