// REFACTOR THIS LIKE THERE IS NO TOMORROW.
// IT IS AN ABSOLUTE PAIN TO CHANGE ANYTHING.

use ultraviolet::Vec3;
use rand::{SeedableRng, Rng, rngs::ThreadRng};
use rand_chacha::ChaCha20Rng;

use crate::{
    scene::camera::Camera, 
    hittables::{sphere::Sphere, hittable_list::HittableList}, 
    hittables::{moving_sphere::MovingSphere, yz_rect::YZRect, flip_face::FlipFace, xz_rect::XZRect, translate::Translate, constant_medium::ConstantMedium, box_hittable::BoxHittable, rotate_y::RotateY}, hittables::{bvh_node::BVHNode, xy_rect::XYRect, hittable_enum::HittableEnum}, 
    services::scene_service::{SceneService},
    services::service_locator::{ServiceLocator}, materials::{lambertian::Lambertian, dielectric::Dielectric, metal::Metal, diffuse_light::DiffuseLight, isotropic::Isotropic, material_enum::MaterialEnum}, textures::{solid_color_texture::SolidColorTexture, checker_texture::CheckerTexture, noise_texture::NoiseTexture, image_texture::ImageTexture, texture_enum::TextureEnum}, core::color_rgb::ColorRGB, math::utility::random_range_chacha, utility::render_config::RenderConfig
};

pub struct SceneBuilder {

}

fn init_build_resources(config: &RenderConfig, camera: Camera, background: ColorRGB) -> (ChaCha20Rng, ServiceLocator, Vec<usize>, Vec<usize>){
    let rng: ChaCha20Rng = ChaCha20Rng::seed_from_u64(config.seed.try_into().unwrap());

    let service_locator: ServiceLocator = ServiceLocator::new(SceneService::new(camera, background));
    let hittable_index_list: Vec<usize> = Vec::new();
    let light_index_list: Vec<usize> = Vec::new();

    (rng, service_locator, hittable_index_list, light_index_list)
}

fn build_acceleration_structures(rng: &mut ChaCha20Rng, service_locator: &mut ServiceLocator, mut hittable_index_list: Vec<usize>, light_index_list: Vec<usize>) {
    let start_time = service_locator.get_scene_service().get_camera().get_start_time();
    let end_time = service_locator.get_scene_service().get_camera().get_end_time();

    if 0 < hittable_index_list.len() {
        let hittable_service = service_locator.get_hittable_service_mut();
        let node = 
            HittableEnum::BVHNode(
                BVHNode::from_index_list(
                    rng, 
                    hittable_service, 
                    &mut hittable_index_list, 
                    start_time, 
                    end_time
                )
            );
        let root_node_index = hittable_service.add_hittable(node);
        hittable_service.set_bvh_root_index(root_node_index);
    }

    if 0 < light_index_list.len() {
        let hittable_service = service_locator.get_hittable_service_mut();
        // PDF related functions don't work for BVH currently
        // let node = 
        //     HittableEnum::BVHNode(
        //         BVHNode::from_index_list(
        //             rng, 
        //             hittable_service, 
        //             &mut light_index_list, 
        //             start_time, 
        //             end_time
        //         )
        //     );
            
        let light_list = HittableEnum::HittableList(HittableList::from_list(light_index_list));
        let root_node_index = hittable_service.add_hittable(light_list);
        hittable_service.set_lights_root_index(root_node_index);
    }

}

// Conver to output image description service
impl SceneBuilder {
    pub fn build_scene(config: &RenderConfig) -> ServiceLocator {
        // Scene
        let random_balls_count = 11;
        let noise_points_count = 256;
        let cube_sphere_count = 1000;

        let service_locator = match &config.scene_index {
            0 => Self::random_spheres_scene(config, random_balls_count),
            1 => Self::random_moving_spheres_scene(config, random_balls_count),
            2 => Self::two_spheres_scene(config),
            3 => Self::two_perlin_spheres_scene(config, noise_points_count),
            4 => Self::earth_scene(config),
            5 => Self::simple_light_scene(config, noise_points_count),
            6 => Self::empty_cornell_box_scene(config),
            7 => Self::cornell_box_two_diffuse_boxes_scene(config),
            8 => Self::cornell_box_two_smoke_boxes_scene(config),
            9 => Self::cornell_box_diffuse_metal_boxes_scene(config),
            10 => Self::final_scene_book_2(config, noise_points_count, cube_sphere_count),
            11 => Self::final_scene_book_3(config),
            _ => panic!("Incorrect scene chosen!"),
        };

        service_locator
    }

    fn random_spheres_scene(config: &RenderConfig, number_of_balls: i32) -> ServiceLocator {
        // Camera
        let look_from: Vec3 = Vec3::new(13.0, 2.0, 3.0 );
        let look_at: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let v_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = 15.0;
        let aperture = 0.05;
        let time_0: f32 = 0.0;
        let time_1: f32 = 1.0;
        let vfov = 20.0;
        let camera = Camera::new(look_from, look_at, v_up, vfov, config.aspect_ratio, aperture, dist_to_focus, time_0, time_1);

        let background = ColorRGB::new(0.7, 0.8, 1.0);

        let (mut rng, mut service_locator, mut hittable_index_list, light_index_list) = init_build_resources(config, camera, background);

        let ground_odd_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new(0.2,0.3,0.1))));
        let ground_even_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new(0.9, 0.9, 0.9))));
        let ground_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::CheckerTexture(CheckerTexture::new(ground_odd_texture_index, ground_even_texture_index)));
        let ground_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(ground_texture_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new( 0.0, -1000.0, 0.0), 1000.0, ground_material_index))));
    
        let index_of_refraction = 1.5;
        let glass_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Dielectric(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction}));
    
        for a in -number_of_balls..number_of_balls {
            for b in -number_of_balls..number_of_balls {
                let choose_mat = rng.gen::<f32>();
                let center: Vec3 = Vec3::new( a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());
    
                if 0.9 < (center - Vec3::new(4.0, 0.2, 0.0)).mag() {
                    if choose_mat < 0.8 {
                        let chosen_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&(ColorRGB::random_chacha(&mut rng) * ColorRGB::random_chacha(&mut rng)))));
                        let chosen_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(chosen_texture_index)));
                        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(center, 0.2, chosen_material_index))));
                    } else if choose_mat < 0.95 {
                        let chosen_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Metal(Metal{albedo: ColorRGB::random_chacha(&mut rng), fuzz: rng.gen::<f32>()}));
                        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(center, 0.2, chosen_material_index))));
                    } else {
                        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(center, 0.2, glass_material_index))));
                    }
                }
            }
        }
        
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new( 0.0, 1.0, 0.0), 1.0, glass_material_index))));
    
        let lambertian_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new(0.4, 0.2, 0.1))));
        let lambertian_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(lambertian_texture_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, lambertian_material_index))));
    
        let metal_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Metal(Metal{albedo: ColorRGB::new(0.7, 0.6, 0.5), fuzz: 0.0}));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, metal_material_index))));

        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, light_index_list);

        service_locator
    }
    
    fn random_moving_spheres_scene(config: &RenderConfig, number_of_balls: i32) -> ServiceLocator {
        // Camera
        let look_from: Vec3 = Vec3::new(13.0, 2.0, 3.0);
        let look_at: Vec3 = Vec3::new( 0.0, 0.0, 0.0);
        let v_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = 15.0;
        let aperture = 0.1;
        let time_0: f32 = 0.0;
        let time_1: f32 = 1.0;
        let camera = Camera::new(look_from, look_at, v_up,20.0, config.aspect_ratio, aperture, dist_to_focus, time_0, time_1);

        let background = ColorRGB::new(0.7, 0.8, 1.0);

        let (mut rng, mut service_locator, mut hittable_index_list, light_index_list) = init_build_resources(config, camera, background);


        let ground_odd_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new(0.2, 0.3, 0.1))));
        let ground_even_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new(0.9, 0.9, 0.9))));
        let ground_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::CheckerTexture(CheckerTexture::new(ground_odd_texture_index, ground_even_texture_index)));
        let ground_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(ground_texture_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material_index))));
    
        let index_of_refraction = 1.5;
        let glass_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Dielectric(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction}));
        
        for a in -number_of_balls..number_of_balls {
            for b in -number_of_balls..number_of_balls {
                let choose_mat = rng.gen::<f32>();
                let center: Vec3 = Vec3::new( a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());
    
                if (center - Vec3::new(4.0, 0.2, 0.0)).mag() > 0.9 {
                    if choose_mat < 0.8 {
                        let mut movement = Vec3::zero();
                        movement.y = rng.gen::<f32>() * 0.5;
    
                        let chosen_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&(ColorRGB::random_chacha(&mut rng) * ColorRGB::random_chacha(&mut rng)))));
                        let chosen_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(chosen_texture_index)));
                        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::MovingSphere(MovingSphere::new(0.2, center, center + movement,  chosen_material_index, 0.0, 1.0))));
                    } else if choose_mat < 0.95 {    
                        let chosen_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Metal(Metal{albedo: ColorRGB::random_chacha(&mut rng), fuzz: rng.gen::<f32>()}));
                        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(center, 0.2, chosen_material_index))));
                    } else {
                        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(center, 0.2, glass_material_index))));
                    }
                }
            }
        }
    
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new( 0.0, 1.0, 0.0), 1.0, glass_material_index))));
    
        let lambertian_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new( 0.4, 0.2, 0.1))));
        let lambertian_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(lambertian_texture_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new( -4.0, 1.0, 0.0), 1.0, lambertian_material_index))));
    
        let metal_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Metal(Metal{albedo: ColorRGB::new( 0.7, 0.6, 0.5), fuzz: 0.0}));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new( 4.0, 1.0, 0.0), 1.0, metal_material_index))));

        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, light_index_list);

        service_locator
    }
    
    fn two_spheres_scene(config: &RenderConfig) -> ServiceLocator {
        // Camera
        let look_from: Vec3 = Vec3::new(13.0, 2.0, 3.0);
        let look_at: Vec3 = Vec3::new( 0.0, 0.0, 0.0);
        let v_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus: f32 = 15.0;
        let aperture: f32 = 0.05;
        let time_0: f32 = 0.0;
        let time_1: f32 = 1.0;
        let camera = Camera::new(look_from, look_at, v_up,20.0, config.aspect_ratio, aperture, dist_to_focus, time_0, time_1);

        let background = ColorRGB::new(0.7, 0.8, 1.0);

        let (mut rng, mut service_locator, mut hittable_index_list, mut _light_index_list) = init_build_resources(config, camera, background);

    
        let checker_odd_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new(0.2, 0.3, 0.1))));
        let checker_even_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new(0.9, 0.9, 0.9))));
        let checker_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::CheckerTexture(CheckerTexture::new(checker_odd_texture_index, checker_even_texture_index)));
        let checker_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(checker_texture_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new( 0.0, -10.0, 0.0), 10.0, checker_material_index))));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new( 0.0, 10.0, 0.0), 10.0, checker_material_index))));
    
        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, _light_index_list);
    
        service_locator
    }
    
    fn two_perlin_spheres_scene(config: &RenderConfig, element_count: u32) -> ServiceLocator {    
        // Camera
        let look_from: Vec3 = Vec3::new(13.0, 2.0, 3.0);
        let look_at: Vec3 = Vec3::new( 0.0, 0.0, 0.0);
        let v_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus: f32 = 15.0;
        let aperture: f32 = 0.05;
        let time_0: f32 = 0.0;
        let time_1: f32 = 1.0;
        let camera: Camera = Camera::new(look_from, look_at, v_up,20.0, config.aspect_ratio, aperture, dist_to_focus, time_0, time_1);

        let background: ColorRGB = ColorRGB::new(0.7, 0.8, 1.0);

        let (mut rng, mut service_locator, mut hittable_index_list, mut _light_index_list) = init_build_resources(config, camera, background);

    
        // The Noise Texture runs pretty deep
        // I just need some determinism, not all the way
        let mut thread_rng = rand::thread_rng();
        let perlin_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::NoiseTexture(NoiseTexture::new(&mut thread_rng, element_count, 4.0)));
        let perlin_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(perlin_texture_index)));

        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new( 0.0, -1000.0, 0.0), 1000.0, perlin_material_index))));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new( 0.0, 2.0, 0.0), 2.0, perlin_material_index))));
        
        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, _light_index_list);
        
        service_locator
    }
    
    fn earth_scene(config: &RenderConfig) -> ServiceLocator {
        // Camera
        let look_from: Vec3 = Vec3::new(13.0, 2.0, 3.0);
        let look_at: Vec3 = Vec3::new( 0.0, 0.0, 0.0);
        let v_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus: f32 = 15.0;
        let aperture: f32 = 0.05;
        let time_0: f32 = 0.0;
        let time_1: f32 = 1.0;
        let camera: Camera = Camera::new(look_from, look_at, v_up,20.0, config.aspect_ratio, aperture, dist_to_focus, time_0, time_1);

        let background = ColorRGB::new(0.7, 0.8, 1.0);

        let (mut rng, mut service_locator, mut hittable_index_list, mut _light_index_list) = init_build_resources(config, camera, background);
    
        let texture_index:usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::ImageTexture(ImageTexture::new("earthmap.png")));
        let material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(texture_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, material_index))));
    
        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, _light_index_list);

        service_locator
    }
    
    fn simple_light_scene(config: &RenderConfig, element_count: u32) -> ServiceLocator {    
        // Camera
        let look_from: Vec3 = Vec3::new( 26.0, 3.0, 6.0 );
        let look_at: Vec3 = Vec3::new( 0.0, 2.0, 0.0);
        let v_up: Vec3 = Vec3::new( 0.0, 1.0, 0.0);
        let dist_to_focus: f32 = 15.0;
        let aperture: f32 = 0.05;
        let time_0: f32 = 0.0;
        let time_1: f32 = 1.0;
        let camera: Camera = Camera::new(look_from, look_at, v_up,20.0, config.aspect_ratio, aperture, dist_to_focus, time_0, time_1);
    
        let background: ColorRGB = ColorRGB::black();

        let (mut rng, mut service_locator, mut hittable_index_list, mut light_index_list) = init_build_resources(config, camera, background);

        // The Noise Texture runs pretty deep
        // I just need some determinism, not all the way
        let mut thread_rng = rand::thread_rng();
        let perlin_texture_index: usize =  service_locator.get_texture_service_mut().add_texture(TextureEnum::NoiseTexture(NoiseTexture::new(&mut thread_rng, element_count, 4.0)));
        let perlin_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(perlin_texture_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, perlin_material_index))));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new( 0.0, 2.0, 0.0), 2.0, perlin_material_index))));
    
        let diffuse_ligt_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new( 4.0, 4.0, 4.0 ))));
        let diffuse_light_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::DiffuseLight(DiffuseLight::new(diffuse_ligt_texture_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XYRect(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, diffuse_light_material_index))));
        light_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XYRect(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, diffuse_light_material_index))));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new( 0.0, 7.0, 0.0), 2.0, diffuse_light_material_index))));
        light_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new( 0.0, 7.0, 0.0), 2.0, diffuse_light_material_index))));
        
        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, light_index_list);

        service_locator
    }
    
    fn empty_cornell_box_scene_prebuild(config: &RenderConfig) -> (ChaCha20Rng, ServiceLocator, Vec<usize>, Vec<usize>) {
        // Camera
        let look_from: Vec3 = Vec3::new( 278.0, 278.0, -800.0);
        let look_at: Vec3 = Vec3::new(278.0, 278.0, 0.0);
        let v_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus: f32 = 15.0;
        let aperture: f32 = 0.0;
        let time_0: f32 = 0.0;
        let time_1: f32 = 1.0;
        let vfov: f32 = 40.0;
        let camera: Camera = Camera::new(look_from, look_at, v_up, vfov, config.aspect_ratio, aperture, dist_to_focus, time_0, time_1);

        let background: ColorRGB = ColorRGB::black();

        let (rng, mut service_locator, mut hittable_index_list, mut light_index_list) = init_build_resources(config, camera, background);


    
        let red_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new(0.65, 0.05, 0.05))));
        let red_material_index: usize  = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(red_texture_index)));
    
        let white_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new(0.73, 0.73, 0.73))));
        let white_material_index: usize  = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(white_texture_index)));
    
        let green_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new( 0.12, 0.45, 0.15))));
        let green_material_index: usize = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(green_texture_index)));
    
        let diffuse_light_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new( 15.0, 15.0, 15.0 ))));
        let diffuse_light_material_index: usize = service_locator.get_material_service_mut().add_material(MaterialEnum::DiffuseLight(DiffuseLight::new( diffuse_light_texture_index )));
    
    
        let green_wall_index: usize = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::YZRect(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green_material_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::FlipFace(FlipFace::new(green_wall_index))));

        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::YZRect(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red_material_index))));
    
        let white_wall_index: usize = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XZRect(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white_material_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::FlipFace(FlipFace::new(white_wall_index))));
    
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XZRect(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white_material_index))));

        let white_wall_index: usize = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XYRect(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white_material_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::FlipFace(FlipFace::new(white_wall_index))));
    
        let unflipped_light_index: usize = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XZRect(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, diffuse_light_material_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::FlipFace(FlipFace::new(unflipped_light_index))));
        light_index_list.push(unflipped_light_index);
    
        (rng, service_locator, hittable_index_list, light_index_list)
    }
    
    fn empty_cornell_box_scene(config: &RenderConfig) -> ServiceLocator {
        let (mut rng, mut service_locator, hittable_index_list, light_index_list) = Self::empty_cornell_box_scene_prebuild(config);

        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, light_index_list);

        service_locator
    }

    fn cornell_box_two_diffuse_boxes_scene(config: &RenderConfig) -> ServiceLocator {
        let (mut rng, mut service_locator, mut hittable_index_list, light_index_list) = Self::empty_cornell_box_scene_prebuild(config);

        let white_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new(0.73, 0.73, 0.73))));
        let white_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(white_texture_index)));

        let box_1 = BoxHittable::new(&mut rng, service_locator.get_hittable_service_mut(), Vec3::new( 0.0, 0.0, 0.0), Vec3::new( 165.0, 330.0, 165.0), white_material_index);
        let box_1_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::BoxHittable(box_1));
        let box_1_rotation = RotateY::new(service_locator.get_hittable_service_mut(), 15.0, box_1_index);
        let box_1_rotation_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::RotateY(box_1_rotation));
        let box_1_translated_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Translate(Translate::new(Vec3::new( 265.0, 0.0, 295.0 ), box_1_rotation_index)));
        hittable_index_list.push(box_1_translated_index);
    
        let box_2 = BoxHittable::new(&mut rng, service_locator.get_hittable_service_mut(), Vec3::zero(), Vec3::new(165.0, 165.0, 165.0), white_material_index);
        let box_2_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::BoxHittable(box_2));
        let box_2_rotation = RotateY::new(service_locator.get_hittable_service_mut(), -18.0, box_2_index);
        let box_2_rotation_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::RotateY(box_2_rotation));
        let box_2_translated_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Translate(Translate::new(Vec3::new( 130.0, 0.0, 65.0 ), box_2_rotation_index)));
        hittable_index_list.push(box_2_translated_index);

        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, light_index_list);

        service_locator
    }
    
    fn cornell_box_diffuse_metal_boxes_scene(config: &RenderConfig) -> ServiceLocator {
        let (mut rng, mut service_locator, mut hittable_index_list, light_index_list) = Self::empty_cornell_box_scene_prebuild(config);

        let metal_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Metal(Metal::new(ColorRGB::new(0.8, 0.85, 0.88), 0.0)));
    
        let white_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new( 0.73, 0.73, 0.73))));
        let white_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(white_texture_index)));
    
        let box_1 = BoxHittable::new(&mut rng, service_locator.get_hittable_service_mut(), Vec3::zero(), Vec3::new( 165.0, 330.0, 165.0), metal_material_index);
        let box_1_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::BoxHittable(box_1));
        let box_1_rotation = RotateY::new(service_locator.get_hittable_service_mut(), 15.0, box_1_index);
        let box_1_rotation_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::RotateY(box_1_rotation));
        let box_1_translated_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Translate(Translate::new(Vec3::new( 265.0, 0.0, 295.0 ), box_1_rotation_index)));
        hittable_index_list.push(box_1_translated_index);
    
        let box_2 = BoxHittable::new(&mut rng, service_locator.get_hittable_service_mut(), Vec3::zero(), Vec3::new(165.0, 165.0, 165.0), white_material_index);
        let box_2_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::BoxHittable(box_2));
        let box_2_rotation = RotateY::new(service_locator.get_hittable_service_mut(), -18.0, box_2_index);
        let box_2_rotation_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::RotateY(box_2_rotation));
        let box_2_translated_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Translate(Translate::new(Vec3::new( 130.0, 0.0, 65.0 ), box_2_rotation_index)));
        hittable_index_list.push(box_2_translated_index);
        
        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, light_index_list);

        service_locator
    }
    
    fn cornell_box_two_smoke_boxes_scene(config: &RenderConfig) -> ServiceLocator {    
        // Camera
        let look_from: Vec3 = Vec3::new( 278.0, 278.0, -800.0);
        let look_at: Vec3 = Vec3::new( 278.0, 278.0, 0.0);
        let v_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus: f32 = 15.0;
        let aperture: f32 = 0.0;
        let time_0: f32 = 0.0;
        let time_1: f32 = 1.0;
        let vfov: f32 = 40.0;
        let camera: Camera = Camera::new(look_from, look_at, v_up, vfov, config.aspect_ratio, aperture, dist_to_focus, time_0, time_1);

        let background: ColorRGB = ColorRGB::black();


        let (mut rng, mut service_locator, mut hittable_index_list, mut light_index_list) = init_build_resources(config, camera, background);

        let red_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new(0.65, 0.05, 0.05))));
        let red_material_index: usize  = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(red_texture_index)));
    
        let white_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new(0.73, 0.73, 0.73))));
        let white_material_index: usize  = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(white_texture_index)));
    
        let green_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new(0.12, 0.45, 0.15))));
        let green_material_index: usize = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(green_texture_index)));
    
        let diffuse_light_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new(7.0, 7.0, 7.0 ))));
        let diffuse_light_material_index: usize = service_locator.get_material_service_mut().add_material(MaterialEnum::DiffuseLight(DiffuseLight::new( diffuse_light_texture_index )));
    
        let dark_phase_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new(0.0, 0.0, 0.0))));
        let dark_phase_function_index: usize = service_locator.get_material_service_mut().add_material(MaterialEnum::Isotropic(Isotropic::new(dark_phase_texture_index)));

        let light_phase_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new(1.0, 1.0, 1.0))));
        let light_phase_function_index: usize = service_locator.get_material_service_mut().add_material(MaterialEnum::Isotropic(Isotropic::new(light_phase_texture_index)));


        let green_wall = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::YZRect(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green_material_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::FlipFace(FlipFace::new(green_wall))));
        
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::YZRect(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red_material_index))));
    
        let white_wall_1_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XZRect(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white_material_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::FlipFace(FlipFace::new(white_wall_1_index))));


        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XZRect(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white_material_index))));


        let white_wall_3_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XYRect(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white_material_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::FlipFace(FlipFace::new(white_wall_3_index))));
    
    
        let unflipped_light_index: usize = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XZRect(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, diffuse_light_material_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::FlipFace(FlipFace::new(unflipped_light_index))));
        light_index_list.push(unflipped_light_index);


        let box_1 = BoxHittable::new(&mut rng, service_locator.get_hittable_service_mut(), Vec3::zero(), Vec3::new(165.0, 330.0, 165.0), white_material_index);
        let box_1_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::BoxHittable(box_1));
        let box_1_rotation = RotateY::new(service_locator.get_hittable_service_mut(), 15.0, box_1_index);
        let box_1_rotation_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::RotateY(box_1_rotation));
        let box_1_translated_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Translate(Translate::new(Vec3::new(265.0, 0.0, 295.0), box_1_rotation_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::ConstantMedium(ConstantMedium::new(box_1_translated_index, dark_phase_function_index, 0.01))));
    
        let box_2 = BoxHittable::new(&mut rng, service_locator.get_hittable_service_mut(), Vec3::zero(), Vec3::new(165.0, 165.0, 165.0), white_material_index);
        let box_2_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::BoxHittable(box_2));
        let box_2_rotation = RotateY::new(service_locator.get_hittable_service_mut(), -18.0, box_2_index);
        let box_2_rotation_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::RotateY(box_2_rotation));
        let box_2_translated_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Translate(Translate::new(Vec3::new( 130.0, 0.0, 65.0 ), box_2_rotation_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::ConstantMedium(ConstantMedium::new(box_2_translated_index, light_phase_function_index, 0.01))));
    
        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, light_index_list);
    
    
        service_locator
    }
    
    fn final_scene_book_2(config: &RenderConfig,  perlin_element_count: u32, cube_sphere_count: u32) -> ServiceLocator {
        // Camera
        let look_from: Vec3 = Vec3::new( 478.0, 278.0, -600.0);
        let look_at: Vec3 = Vec3::new(278.0, 278.0, 0.0);
        let v_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus: f32 = 15.0;
        let aperture: f32 = 0.0;
        let time_0: f32 = 0.0;
        let time_1: f32 = 1.0;
        let vfov: f32 = 40.0;
        let camera: Camera = Camera::new(look_from, look_at, v_up, vfov, config.aspect_ratio, aperture, dist_to_focus, time_0, time_1);

        let background: ColorRGB = ColorRGB::black();


        let (mut rng, mut service_locator, mut hittable_index_list, mut light_index_list) = init_build_resources(config, camera, background);
        let start_time: f32 = service_locator.get_scene_service().get_camera().get_start_time();
        let end_time: f32 = service_locator.get_scene_service().get_camera().get_end_time();
        
        let mut floor_cubes_indices: Vec<usize> = Vec::new();
    
        let ground_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new(0.48, 0.83, 0.53))));
        let ground_material_index: usize = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(ground_texture_index)));


        let boxes_per_side: u32 = 20;
        for i in 0..boxes_per_side {
            let i_f: f32 = i as f32;
            for j in 0..boxes_per_side {
                let j_f: f32 = j as f32;
                let w : f32 = 100.0;
                let x0: f32 = -1000.0 + i_f * w;
                let z0: f32 = -1000.0 + j_f * w;
                let y0: f32 = 0.0;
                let x1: f32 = x0 + w;
                let y1: f32 = rng.gen_range(1.0..101.0);
                let z1: f32 = z0 + w;
    
                let element: HittableEnum = HittableEnum::BoxHittable(
                    BoxHittable::new(
                            &mut rng,
            service_locator.get_hittable_service_mut(),
                    Vec3::new( x0, y0, z0), 
                    Vec3::new( x1, y1, z1), 
                    ground_material_index
                    )
                );

                floor_cubes_indices.push( service_locator.get_hittable_service_mut().add_hittable( element ));
            }
        }
    
        let floor_cubes_bvh: BVHNode = BVHNode::from_index_list(&mut rng, service_locator.get_hittable_service_mut(), &mut floor_cubes_indices, start_time, end_time);
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::BVHNode(floor_cubes_bvh)));
    
    
        let diffuse_light_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new( 3.5, 3.5, 3.5 ))));
        let diffuse_light_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::DiffuseLight(DiffuseLight::new( diffuse_light_texture_index )));
        let unflipped_light_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XZRect(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, diffuse_light_material_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::FlipFace(FlipFace::new(unflipped_light_index))));
        light_index_list.push(unflipped_light_index);
    
        let center_0: Vec3 = Vec3::new(400.0, 400.0, 200.0);
        let center_1: Vec3 = center_0 + Vec3::new( 30.0, 0.0, 0.0);
        let moving_sphere_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new( 0.7, 0.3, 0.1))));
        let moving_sphere_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(moving_sphere_texture_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::MovingSphere(MovingSphere{ radius: 50.0, center_0, center_1, material: moving_sphere_material_index, time_0: 0.0, time_1: 1.0 })));
    
    
        let index_of_refraction: f32 = 1.5;
        let glass_material_index: usize = service_locator.get_material_service_mut().add_material(MaterialEnum::Dielectric(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction}));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new( 260.0, 150.0, 45.0), 50.0, glass_material_index))));
        light_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new(260.0, 150.0, 45.0), 50.0, diffuse_light_material_index))));
    
        let metal_material_index: usize = service_locator.get_material_service_mut().add_material(MaterialEnum::Metal(Metal{albedo: ColorRGB::new(0.8, 0.8, 0.9), fuzz: 1.0}));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new(0.0, 150.0, 145.0), 50.0, metal_material_index))));
    
        // Volume sphere
        let white_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new( 0.73, 0.73, 0.73))));
        let white_material_index: usize = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(white_texture_index)));

        let boundary_index: usize = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, glass_material_index)));
        hittable_index_list.push(boundary_index);
        let blue_phase_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new( 0.2, 0.4,  0.9))));
        let blue_phase_function_index: usize = service_locator.get_material_service_mut().add_material(MaterialEnum::Isotropic(Isotropic::new(blue_phase_texture_index)));
        let volume_sphere: ConstantMedium = ConstantMedium::new(boundary_index, blue_phase_function_index, 0.2);
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::ConstantMedium(volume_sphere)));
    
        let global_phase_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new( 1.0, 1.0, 1.0))));
        let global_phase_function_index: usize = service_locator.get_material_service_mut().add_material(MaterialEnum::Isotropic(Isotropic::new(global_phase_texture_index)));
        let global_volume_sphere_index: usize = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::zero(), 5000.0, white_material_index)));
        let global_volume: HittableEnum = HittableEnum::ConstantMedium(ConstantMedium::new(global_volume_sphere_index, global_phase_function_index, 0.0001));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(global_volume));
    
        let earth_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::ImageTexture(ImageTexture::new("earthmap.png")));
        let earth_material_index: usize = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(earth_texture_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new(400.0, 200.0, 400.0), 100.0, earth_material_index))));
    
        
        // The Noise Texture runs pretty deep
        // I just need some determinism, not all the way
        let mut thread_rng: ThreadRng = rand::thread_rng();
        let perlin_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::NoiseTexture(NoiseTexture::new(&mut thread_rng, perlin_element_count, 0.1)));
        let perlin_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(perlin_texture_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new( 220.0, 280.0, 300.0), 80.0, perlin_material_index))));
    
    

        let mut cube_spheres_indices: Vec<usize> = Vec::new();
        for _j in 0..cube_sphere_count {
            cube_spheres_indices.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(random_range_chacha(&mut rng, 0.0, 165.0), 10.0, white_material_index))));
        }
        let cube_spheres_bvh: BVHNode = BVHNode::from_index_list(&mut rng, service_locator.get_hittable_service_mut(), &mut cube_spheres_indices, start_time, end_time);
        let cube_spheres_bvh_index: usize = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::BVHNode(cube_spheres_bvh));
        let cube_spheres_rotation: RotateY = RotateY::new(service_locator.get_hittable_service_mut(), 15.0, cube_spheres_bvh_index);
        let cube_spheres_rotation_index: usize = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::RotateY(cube_spheres_rotation));
        let cube_spheres_translated: usize = service_locator.get_hittable_service_mut().add_hittable( HittableEnum::Translate(Translate::new(Vec3::new( -100.0, 270.0, 395.0 ), cube_spheres_rotation_index)));
        hittable_index_list.push(cube_spheres_translated);
        

        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, light_index_list);
    
        service_locator
    }
    
    fn final_scene_book_3(config: &RenderConfig) -> ServiceLocator {
        let (mut rng, mut service_locator, mut hittable_index_list, mut light_index_list) = Self::empty_cornell_box_scene_prebuild(config);

        let white_texture_index: usize = service_locator.get_texture_service_mut().add_texture(TextureEnum::SolidColorTexture(SolidColorTexture::from_color(&ColorRGB::new( 0.73, 0.73, 0.73))));
        let white_material_index: usize = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian::new(white_texture_index)));
    
        let index_of_refraction: f32 = 1.5;
        let glass_material_index: usize = service_locator.get_material_service_mut().add_material(MaterialEnum::Dielectric(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction}));
    
        let box_1: BoxHittable = BoxHittable::new(&mut rng, service_locator.get_hittable_service_mut(), Vec3::zero(), Vec3::new(165.0, 330.0, 165.0), white_material_index);
        let box_1_index: usize =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::BoxHittable(box_1));
        let box_1_rotation: RotateY = RotateY::new(service_locator.get_hittable_service_mut(), 15.0, box_1_index);
        let box_1_rotation_index: usize =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::RotateY(box_1_rotation));
        let box_1_translated_index: usize =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Translate(Translate::new(Vec3::new( 265.0, 0.0, 295.0 ), box_1_rotation_index)));
        hittable_index_list.push(box_1_translated_index);
    
        let sphere_index: usize = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vec3::new( 190.0, 90.0, 190.0), 90.0, glass_material_index)));
        hittable_index_list.push(sphere_index);
        light_index_list.push(sphere_index);
    
        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, light_index_list);

        service_locator
    }
}