// REFACTOR THIS LIKE THERE IS NO TOMORROW.
// IT IS AN ABSOLUTE PAIN TO CHANGE ANYTHING.

use std::sync::Arc;

use rand::{SeedableRng, Rng};
use rand_chacha::ChaCha20Rng;

use crate::{
    hittable::{XYRect, YZRect, FlipFace, XZRect, BoxHittable, RotateY, Translate, ConstantMedium, HittableList}, 
    material_service::{MaterialEnum}, 
    camera::Camera, 
    vector3::{Color, Point3, Vector3}, 
    texture::{CheckerTexture, Texture, SolidColorTexture, NoiseTexture, ImageTexture}, 
    sphere::Sphere, 
    material::{Lambertian, Dielectric, Metal, DiffuseLight, Isotropic}, 
    moving_sphere::MovingSphere, bvh_node::BVHNode, 
    scene_service::{SceneService},
    service_locator::{ServiceLocator}, hittable_service::{HittableEnum}
};

pub struct SceneBuilder {

}

fn init_build_resources(camera: Camera, background: Color) -> (ChaCha20Rng, ServiceLocator, Vec<usize>, Vec<usize>){
    let seed: u64 = 13371337;
    let rng = ChaCha20Rng::seed_from_u64(seed);

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
    pub fn build_scene(mut aspect_ratio: f64, image_width: i64, scene_index: usize) -> (f64, i64, ServiceLocator) {
        // Display Image
        let mut image_height = ((image_width as f64) / aspect_ratio) as i64;
        image_height = image_height + image_height % 2;


        // Scene
        let random_balls_count = 11;
        let noise_points_count = 256;
        let cube_sphere_count = 1000;

        if 5 < scene_index {
            aspect_ratio = 1.0;
            image_height = ((image_width as f64) / aspect_ratio) as i64;
            image_height = image_height + image_height % 2;
        }

        let service_locator = match scene_index {
            0 => Self::random_spheres_scene(aspect_ratio, random_balls_count),
            1 => Self::random_moving_spheres_scene(aspect_ratio, random_balls_count),
            2 => Self::two_spheres_scene(aspect_ratio),
            3 => Self::two_perlin_spheres_scene(aspect_ratio, noise_points_count),
            4 => Self::earth_scene(aspect_ratio),
            5 => Self::simple_light_scene(aspect_ratio, noise_points_count),
            6 => Self::empty_cornell_box_scene(aspect_ratio),
            7 => Self::cornell_box_two_diffuse_boxes_scene(aspect_ratio),
            8 => Self::cornell_box_two_smoke_boxes_scene(aspect_ratio),
            9 => Self::cornell_box_diffuse_metal_boxes_scene(aspect_ratio),
            10 => Self::final_scene_book_2(aspect_ratio, noise_points_count, cube_sphere_count),
            11 => Self::final_scene_book_3(aspect_ratio),
            _ => panic!("Incorrect scene chosen!"),
        };

        (aspect_ratio, image_height, service_locator)
    }

    fn random_spheres_scene(aspect_ratio: f64, number_of_balls: i32) -> ServiceLocator {
        // Camera
        let look_from = Point3{x: 13.0, y: 2.0, z: 3.0 };
        let look_at = Point3{x: 0.0, y: 0.0, z: 0.0};
        let v_up = Vector3{x: 0.0, y:1.0, z:0.0};
        let dist_to_focus = 15.0;
        let aperture = 0.05;
        let time_0: f64 = 0.0;
        let time_1: f64 = 1.0;
        let vfov = 20.0;
        let camera = Camera::new(look_from, look_at, v_up, vfov, aspect_ratio, aperture, dist_to_focus, time_0, time_1);

        let background = Color{x:0.7, y:0.8, z: 1.0};

        let (mut rng, mut service_locator, mut hittable_index_list, light_index_list) = init_build_resources(camera, background);


        let ground_texture: Arc<dyn Texture> = Arc::new(CheckerTexture::from_colors(&Color{x:0.2, y:0.3, z:0.1}, &Color{x:0.9, y:0.9, z:0.9}));
        let ground_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{albedo: ground_texture}));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, ground_material_index))));
    
        let index_of_refraction = 1.5;
        let glass_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Dielectric(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction}));
    
        for a in -number_of_balls..number_of_balls {
            for b in -number_of_balls..number_of_balls {
                let choose_mat = rng.gen::<f64>();
                let center = Point3{x: a as f64 + 0.9 * rng.gen::<f64>(), y: 0.2, z: b as f64 + 0.9 * rng.gen::<f64>()};
    
                if 0.9 < (center - Point3{x: 4.0, y: 0.2, z: 0.0}).length() {
                    if choose_mat < 0.8 {
                        let chosen_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&(Color::random_chacha(&mut rng) * Color::random_chacha(&mut rng))));
                        let chosen_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{albedo: chosen_texture}));
                        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(center, 0.2, chosen_material_index))));
                    } else if choose_mat < 0.95 {
                        let chosen_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Metal(Metal{albedo: Color::random_chacha(&mut rng), fuzz: rng.gen::<f64>()}));
                        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(center, 0.2, chosen_material_index))));
                    } else {
                        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(center, 0.2, glass_material_index))));
                    }
                }
            }
        }
        
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 0.0, y: 1.0, z: 0.0}, 1.0, glass_material_index))));
    
        let lambertian_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.4, y: 0.2, z: 0.1}));
        let lambertian_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{albedo: lambertian_texture}));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: -4.0, y: 1.0, z: 0.0}, 1.0, lambertian_material_index))));
    
        let metal_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Metal(Metal{albedo: Color{x: 0.7, y: 0.6, z: 0.5}, fuzz: 0.0}));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 4.0, y: 1.0, z: 0.0}, 1.0, metal_material_index))));

        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, light_index_list);

        service_locator
    }
    
    fn random_moving_spheres_scene(aspect_ratio: f64, number_of_balls: i32) -> ServiceLocator {
        // Camera
        let look_from = Point3{x: 13.0, y: 2.0, z: 3.0 };
        let look_at = Point3{x: 0.0, y: 0.0, z: 0.0};
        let v_up = Vector3{x: 0.0, y:1.0, z:0.0};
        let dist_to_focus = 15.0;
        let aperture = 0.1;
        let time_0: f64 = 0.0;
        let time_1: f64 = 1.0;
        let camera = Camera::new(look_from, look_at, v_up,20.0, aspect_ratio, aperture, dist_to_focus, time_0, time_1);

        let background = Color{x:0.7, y:0.8, z: 1.0};

        let (mut rng, mut service_locator, mut hittable_index_list, light_index_list) = init_build_resources(camera, background);


        let ground_texture: Arc<dyn Texture> = Arc::new(CheckerTexture::from_colors(&Color{x:0.2, y:0.3, z:0.1}, &Color{x:0.9, y:0.9, z:0.9}));
        let ground_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{albedo: ground_texture}));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, ground_material_index))));
    
        let index_of_refraction = 1.5;
        let glass_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Dielectric(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction}));
        
        for a in -number_of_balls..number_of_balls {
            for b in -number_of_balls..number_of_balls {
                let choose_mat = rng.gen::<f64>();
                let center = Point3{x: a as f64 + 0.9 * rng.gen::<f64>(), y: 0.2, z: b as f64 + 0.9 * rng.gen::<f64>()};
    
                if (center - Point3{x: 4.0, y: 0.2, z: 0.0}).length() > 0.9 {
                    if choose_mat < 0.8 {
                        let mut movement = Vector3::zero();
                        movement.y = rng.gen::<f64>() * 0.5;
    
                        let chosen_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&(Color::random_chacha(&mut rng) * Color::random_chacha(&mut rng))));
                        let chosen_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{albedo: chosen_texture}));
                        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::MovingSphere(MovingSphere::new(0.2, center, center + movement,  chosen_material_index, 0.0, 1.0))));
                    } else if choose_mat < 0.95 {    
                        let chosen_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Metal(Metal{albedo: Color::random_chacha(&mut rng), fuzz: rng.gen::<f64>()}));
                        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(center, 0.2, chosen_material_index))));
                    } else {
                        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(center, 0.2, glass_material_index))));
                    }
                }
            }
        }
    
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 0.0, y: 1.0, z: 0.0}, 1.0, glass_material_index))));
    
        let lambertian_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.4, y: 0.2, z: 0.1}));
        let lambertian_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{albedo: lambertian_texture}));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: -4.0, y: 1.0, z: 0.0}, 1.0, lambertian_material_index))));
    
        let metal_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Metal(Metal{albedo: Color{x: 0.7, y: 0.6, z: 0.5}, fuzz: 0.0}));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 4.0, y: 1.0, z: 0.0}, 1.0, metal_material_index))));

        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, light_index_list);

        service_locator
    }
    
    fn two_spheres_scene(aspect_ratio: f64) -> ServiceLocator {
        // Camera
        let look_from = Point3{x: 13.0, y: 2.0, z: 3.0 };
        let look_at = Point3{x: 0.0, y: 0.0, z: 0.0};
        let v_up = Vector3{x: 0.0, y:1.0, z:0.0};
        let dist_to_focus = 15.0;
        let aperture = 0.05;
        let time_0: f64 = 0.0;
        let time_1: f64 = 1.0;
        let camera = Camera::new(look_from, look_at, v_up,20.0, aspect_ratio, aperture, dist_to_focus, time_0, time_1);

        let background = Color{x:0.7, y:0.8, z: 1.0};

        let (mut rng, mut service_locator, mut hittable_index_list, mut _light_index_list) = init_build_resources(camera, background);

    
        let checker_texture: Arc<dyn Texture> = Arc::new(CheckerTexture::from_colors(&Color{x:0.2, y:0.3, z:0.1}, &Color{x:0.9, y:0.9, z:0.9}));
        let checker_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{albedo: checker_texture}));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 0.0, y: -10.0, z: 0.0}, 10.0, checker_material_index))));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 0.0, y: 10.0, z: 0.0}, 10.0, checker_material_index))));
    
        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, _light_index_list);
    
        service_locator
    }
    
    fn two_perlin_spheres_scene(aspect_ratio: f64, element_count: u32) -> ServiceLocator {    
        // Camera
        let look_from = Point3{x: 13.0, y: 2.0, z: 3.0 };
        let look_at = Point3{x: 0.0, y: 0.0, z: 0.0};
        let v_up = Vector3{x: 0.0, y:1.0, z:0.0};
        let dist_to_focus = 15.0;
        let aperture = 0.05;
        let time_0: f64 = 0.0;
        let time_1: f64 = 1.0;
        let camera = Camera::new(look_from, look_at, v_up,20.0, aspect_ratio, aperture, dist_to_focus, time_0, time_1);

        let background = Color{x:0.7, y:0.8, z: 1.0};

        let (mut rng, mut service_locator, mut hittable_index_list, mut _light_index_list) = init_build_resources(camera, background);

    
        // The Noise Texture runs pretty deep
        // I just need some determinism, not all the way
        let mut thread_rng = rand::thread_rng();
        let perlin_texture: Arc<dyn Texture> = Arc::new(NoiseTexture::new(&mut thread_rng, element_count, 4.0));
        let perlin_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{albedo: perlin_texture}));

        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, perlin_material_index))));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 0.0, y: 2.0, z: 0.0}, 2.0, perlin_material_index))));
        
        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, _light_index_list);
        
        service_locator
    }
    
    fn earth_scene(aspect_ratio: f64) -> ServiceLocator {
        // Camera
        let look_from = Point3{x: 13.0, y: 2.0, z: 3.0 };
        let look_at = Point3{x: 0.0, y: 0.0, z: 0.0};
        let v_up = Vector3{x: 0.0, y:1.0, z:0.0};
        let dist_to_focus = 15.0;
        let aperture = 0.05;
        let time_0: f64 = 0.0;
        let time_1: f64 = 1.0;
        let camera = Camera::new(look_from, look_at, v_up,20.0, aspect_ratio, aperture, dist_to_focus, time_0, time_1);

        let background = Color{x:0.7, y:0.8, z: 1.0};

        let (mut rng, mut service_locator, mut hittable_index_list, mut _light_index_list) = init_build_resources(camera, background);
    
        let texture: Arc<dyn Texture> = Arc::new(ImageTexture::new("earthmap.png"));
        let material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{albedo: texture}));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vector3::new(0.0, 0.0, 0.0), 2.0, material_index))));
    
        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, _light_index_list);

        service_locator
    }
    
    fn simple_light_scene(aspect_ratio: f64, element_count: u32) -> ServiceLocator {    
        // Camera
        let look_from = Point3{x: 26.0, y: 3.0, z: 6.0 };
        let look_at = Point3{x: 0.0, y: 2.0, z: 0.0};
        let v_up = Vector3{x: 0.0, y:1.0, z:0.0};
        let dist_to_focus = 15.0;
        let aperture = 0.05;
        let time_0: f64 = 0.0;
        let time_1: f64 = 1.0;
        let camera = Camera::new(look_from, look_at, v_up,20.0, aspect_ratio, aperture, dist_to_focus, time_0, time_1);
    
        let background = Color{x:0.0, y:0.0, z: 0.0};

        let (mut rng, mut service_locator, mut hittable_index_list, mut light_index_list) = init_build_resources(camera, background);

        // The Noise Texture runs pretty deep
        // I just need some determinism, not all the way
        let mut thread_rng = rand::thread_rng();
        let perlin_texture: Arc<dyn Texture> = Arc::new(NoiseTexture::new(&mut thread_rng, element_count, 4.0));
        let perlin_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{albedo: perlin_texture}));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, perlin_material_index))));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 0.0, y: 2.0, z: 0.0}, 2.0, perlin_material_index))));
    
        let diffuse_light_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::DiffuseLight(DiffuseLight::from_color(&Color{x: 4.0, y: 4.0, z: 4.0 })));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XYRect(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, diffuse_light_material_index))));
        light_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XYRect(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, diffuse_light_material_index))));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 0.0, y: 7.0, z: 0.0}, 2.0, diffuse_light_material_index))));
        light_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 0.0, y: 7.0, z: 0.0}, 2.0, diffuse_light_material_index))));
        
        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, light_index_list);

        service_locator
    }
    
    fn empty_cornell_box_scene_prebuild(aspect_ratio: f64) -> (ChaCha20Rng, ServiceLocator, Vec<usize>, Vec<usize>) {
        // Camera
        let look_from = Point3{x: 278.0, y: 278.0, z: -800.0 };
        let look_at = Point3{x: 278.0, y: 278.0, z: 0.0};
        let v_up = Vector3{x: 0.0, y:1.0, z:0.0};
        let dist_to_focus = 15.0;
        let aperture = 0.0;
        let time_0: f64 = 0.0;
        let time_1: f64 = 1.0;
        let vfov = 40.0;
        let camera = Camera::new(look_from, look_at, v_up, vfov, aspect_ratio, aperture, dist_to_focus, time_0, time_1);

        let background = Color{x:0.0, y:0.0, z: 0.0};

        let (rng, mut service_locator, mut hittable_index_list, mut light_index_list) = init_build_resources(camera, background);


    
        let red_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.65, y: 0.05, z: 0.05}));
        let red_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{ albedo: red_texture }));
    
        let white_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
        let white_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{ albedo: white_texture }));
    
        let green_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.12, y: 0.45, z: 0.15}));
        let green_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{ albedo: green_texture }));
    
        let diffuse_light_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::DiffuseLight(DiffuseLight::from_color( &Color{x: 15.0, y: 15.0, z: 15.0 } )));
    
    
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
        light_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XZRect(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, diffuse_light_material_index))));
    
        (rng, service_locator, hittable_index_list, light_index_list)
    }
    
    fn empty_cornell_box_scene(aspect_ratio: f64) -> ServiceLocator {
        let (mut rng, mut service_locator, hittable_index_list, light_index_list) = Self::empty_cornell_box_scene_prebuild(aspect_ratio);

        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, light_index_list);

        service_locator
    }

    fn cornell_box_two_diffuse_boxes_scene(aspect_ratio: f64) -> ServiceLocator {
        let (mut rng, mut service_locator, mut hittable_index_list, light_index_list) = Self::empty_cornell_box_scene_prebuild(aspect_ratio);

        let white_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
        let white_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{ albedo: white_texture }));

        let box_1 = BoxHittable::new(&mut rng, service_locator.get_hittable_service_mut(), Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 165.0, y: 330.0, z: 165.0}, white_material_index);
        let box_1_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::BoxHittable(box_1));
        let box_1_rotation = RotateY::new(service_locator.get_hittable_service_mut(), 15.0, box_1_index);
        let box_1_rotation_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::RotateY(box_1_rotation));
        let box_1_translated_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Translate(Translate::new(Vector3 { x: 265.0, y: 0.0, z: 295.0 }, box_1_rotation_index)));
        hittable_index_list.push(box_1_translated_index);
    
        let box_2 = BoxHittable::new(&mut rng, service_locator.get_hittable_service_mut(), Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 165.0, y: 165.0, z: 165.0}, white_material_index);
        let box_2_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::BoxHittable(box_2));
        let box_2_rotation = RotateY::new(service_locator.get_hittable_service_mut(), -18.0, box_2_index);
        let box_2_rotation_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::RotateY(box_2_rotation));
        let box_2_translated_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Translate(Translate::new(Vector3 { x: 130.0, y: 0.0, z: 65.0 }, box_2_rotation_index)));
        hittable_index_list.push(box_2_translated_index);

        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, light_index_list);

        service_locator
    }
    
    fn cornell_box_diffuse_metal_boxes_scene(aspect_ratio: f64) -> ServiceLocator {
        let (mut rng, mut service_locator, mut hittable_index_list, light_index_list) = Self::empty_cornell_box_scene_prebuild(aspect_ratio);

        let metal_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Metal(Metal::new(Color{x: 0.8, y: 0.85, z: 0.88}, 0.0)));
    
        let white_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
        let white_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{ albedo: white_texture }));
    
        let box_1 = BoxHittable::new(&mut rng, service_locator.get_hittable_service_mut(), Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 165.0, y: 330.0, z: 165.0}, metal_material_index);
        let box_1_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::BoxHittable(box_1));
        let box_1_rotation = RotateY::new(service_locator.get_hittable_service_mut(), 15.0, box_1_index);
        let box_1_rotation_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::RotateY(box_1_rotation));
        let box_1_translated_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Translate(Translate::new(Vector3 { x: 265.0, y: 0.0, z: 295.0 }, box_1_rotation_index)));
        hittable_index_list.push(box_1_translated_index);
    
        let box_2 = BoxHittable::new(&mut rng, service_locator.get_hittable_service_mut(), Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 165.0, y: 165.0, z: 165.0}, white_material_index);
        let box_2_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::BoxHittable(box_2));
        let box_2_rotation = RotateY::new(service_locator.get_hittable_service_mut(), -18.0, box_2_index);
        let box_2_rotation_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::RotateY(box_2_rotation));
        let box_2_translated_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Translate(Translate::new(Vector3 { x: 130.0, y: 0.0, z: 65.0 }, box_2_rotation_index)));
        hittable_index_list.push(box_2_translated_index);
        
        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, light_index_list);

        service_locator
    }
    
    fn cornell_box_two_smoke_boxes_scene(aspect_ratio: f64) -> ServiceLocator {    
        // Camera
        let look_from = Point3{x: 278.0, y: 278.0, z: -800.0 };
        let look_at = Point3{x: 278.0, y: 278.0, z: 0.0};
        let v_up = Vector3{x: 0.0, y:1.0, z:0.0};
        let dist_to_focus = 15.0;
        let aperture = 0.0;
        let time_0: f64 = 0.0;
        let time_1: f64 = 1.0;
        let vfov = 40.0;
        let camera = Camera::new(look_from, look_at, v_up, vfov, aspect_ratio, aperture, dist_to_focus, time_0, time_1);

        let background = Color{x:0.0, y:0.0, z: 0.0};


        let (mut rng, mut service_locator, mut hittable_index_list, mut light_index_list) = init_build_resources(camera, background);

    
        let red_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.65, y: 0.05, z: 0.05}));
        let red_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{ albedo: red_texture }));
    
        let white_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
        let white_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{ albedo: white_texture }));
    
        let green_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.12, y: 0.45, z: 0.15}));
        let green_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{ albedo: green_texture }));
    
        let diffuse_light_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::DiffuseLight(DiffuseLight::from_color( &Color{x: 7.0, y: 7.0, z: 7.0 } )));
    
        let dark_phase_function_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Isotropic(Isotropic::from_color( &Color{x: 0.0, y: 0.0, z: 0.0} )));
        let light_phase_function_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Isotropic(Isotropic::from_color( &Color{x: 1.0, y: 1.0, z: 1.0} )));
    
        let green_wall = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::YZRect(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green_material_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::FlipFace(FlipFace::new(green_wall))));
        
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::YZRect(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red_material_index))));
    
        let white_wall_1_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XZRect(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white_material_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::FlipFace(FlipFace::new(white_wall_1_index))));


        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XZRect(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white_material_index))));


        let white_wall_3_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XYRect(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white_material_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::FlipFace(FlipFace::new(white_wall_3_index))));
    
    
        let unflipped_light_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XZRect(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, diffuse_light_material_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::FlipFace(FlipFace::new(unflipped_light_index))));
        light_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XZRect(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, diffuse_light_material_index))));
    
    
        let box_1 = BoxHittable::new(&mut rng, service_locator.get_hittable_service_mut(), Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 165.0, y: 330.0, z: 165.0}, white_material_index);
        let box_1_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::BoxHittable(box_1));
        let box_1_rotation = RotateY::new(service_locator.get_hittable_service_mut(), 15.0, box_1_index);
        let box_1_rotation_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::RotateY(box_1_rotation));
        let box_1_translated_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Translate(Translate::new(Vector3 { x: 265.0, y: 0.0, z: 295.0 }, box_1_rotation_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::ConstantMedium(ConstantMedium::new(box_1_translated_index, dark_phase_function_index, 0.01))));
    
        let box_2 = BoxHittable::new(&mut rng, service_locator.get_hittable_service_mut(), Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 165.0, y: 165.0, z: 165.0}, white_material_index);
        let box_2_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::BoxHittable(box_2));
        let box_2_rotation = RotateY::new(service_locator.get_hittable_service_mut(), -18.0, box_2_index);
        let box_2_rotation_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::RotateY(box_2_rotation));
        let box_2_translated_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Translate(Translate::new(Vector3 { x: 130.0, y: 0.0, z: 65.0 }, box_2_rotation_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::ConstantMedium(ConstantMedium::new(box_2_translated_index, light_phase_function_index, 0.01))));
    
        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, light_index_list);
    
    
        service_locator
    }
    
    fn final_scene_book_2(aspect_ratio: f64, perlin_element_count: u32, cube_sphere_count: u32) -> ServiceLocator {
        //let seed: u64 = 919;

        
        // Camera
        let look_from = Point3{x: 478.0, y: 278.0, z: -600.0 };
        let look_at = Point3{x: 278.0, y: 278.0, z: 0.0};
        let v_up = Vector3{x: 0.0, y:1.0, z:0.0};
        let dist_to_focus = 15.0;
        let aperture = 0.0;
        let time_0: f64 = 0.0;
        let time_1: f64 = 1.0;
        let vfov = 40.0;
        let camera = Camera::new(look_from, look_at, v_up, vfov, aspect_ratio, aperture, dist_to_focus, time_0, time_1);

        let background = Color{x:0.0, y:0.0, z: 0.0};


        let (mut rng, mut service_locator, mut hittable_index_list, mut light_index_list) = init_build_resources(camera, background);
        let start_time = service_locator.get_scene_service().get_camera().get_start_time();
        let end_time = service_locator.get_scene_service().get_camera().get_end_time();
        
        let mut floor_cubes_indices: Vec<usize> = Vec::new();
    
        let ground_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x:0.48, y:0.83, z:0.53}));
        let ground_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{albedo: ground_texture}));
    
        let boxes_per_side = 20;
        for i in 0..boxes_per_side {
            let i_f = i as f64;
            for j in 0..boxes_per_side {
                let j_f = j as f64;
                let w : f64 = 100.0;
                let x0: f64 = -1000.0 + i_f * w;
                let z0: f64 = -1000.0 + j_f * w;
                let y0: f64 = 0.0;
                let x1: f64 = x0 + w;
                let y1: f64 = rng.gen_range(1.0..101.0);
                let z1: f64 = z0 + w;
    
                let element = HittableEnum::BoxHittable(
                    BoxHittable::new(
                            &mut rng,
            service_locator.get_hittable_service_mut(),
                    Vector3{x: x0, y: y0, z: z0}, 
                    Vector3{x: x1, y: y1, z: z1}, 
                    ground_material_index
                    )
                );

                    floor_cubes_indices.push(
                        service_locator.get_hittable_service_mut().add_hittable(
                            element
                    ));
            }
        }
    
        let floor_cubes_bvh = BVHNode::from_index_list(&mut rng, service_locator.get_hittable_service_mut(), &mut floor_cubes_indices, start_time, end_time);
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::BVHNode(floor_cubes_bvh)));
    
    
    
        let diffuse_light_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::DiffuseLight(DiffuseLight::from_color( &Color{x: 7.0, y: 7.0, z: 7.0 } )));
        let unflipped_light_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XZRect(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, diffuse_light_material_index)));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::FlipFace(FlipFace::new(unflipped_light_index))));
        light_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::XZRect(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, diffuse_light_material_index))));
    
        let center_0 = Vector3{x: 400.0, y: 400.0, z: 200.0};
        let center_1 = center_0 + Vector3{x: 30.0, y: 0.0, z: 0.0};
        let moving_sphere_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.7, y: 0.3, z: 0.1}));
        let moving_sphere_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{ albedo: moving_sphere_texture }));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::MovingSphere(MovingSphere{ radius: 50.0, center_0, center_1, material: moving_sphere_material_index, time_0: 0.0, time_1: 1.0 })));
    
    
        let index_of_refraction = 1.5;
        let glass_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Dielectric(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction}));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 260.0, y: 150.0, z: 45.0}, 50.0, glass_material_index))));
        light_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 260.0, y: 150.0, z: 45.0}, 50.0, diffuse_light_material_index))));
    
        let metal_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Metal(Metal{albedo: Color{x: 0.8, y: 0.8, z: 0.9}, fuzz: 1.0}));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 0.0, y: 150.0, z: 145.0}, 50.0, metal_material_index))));
    
        // Volume sphere
        let boundary_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3
            {x: 360.0, y: 150.0, z: 145.0}, 70.0, glass_material_index)));

        let blue_phase_function_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Isotropic(Isotropic::from_color( &Color{x: 0.2, y: 0.4, z: 0.9} )));
        let volume_sphere= ConstantMedium::new(boundary_index, blue_phase_function_index, 0.2);
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::ConstantMedium(volume_sphere)));
    
        let global_phase_function_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Isotropic(Isotropic::from_color( &Color{x: 1.0, y: 1.0, z: 1.0} )));
        let global_volume_sphere_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 0.0, y: 0.0, z: 0.0}, 5000.0, glass_material_index)));
        let global_volume = HittableEnum::ConstantMedium(ConstantMedium::new(global_volume_sphere_index, global_phase_function_index, 0.0001));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(global_volume));
    
        let earth_texture: Arc<dyn Texture> = Arc::new(ImageTexture::new("earthmap.png"));
        let earth_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{albedo: earth_texture}));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vector3::new(400.0, 200.0, 400.0), 100.0, earth_material_index))));
    
        
        // The Noise Texture runs pretty deep
        // I just need some determinism, not all the way
        let mut thread_rng = rand::thread_rng();
        let perlin_texture: Arc<dyn Texture> = Arc::new(NoiseTexture::new(&mut thread_rng, perlin_element_count, 0.1));
        let perlin_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{albedo: perlin_texture}));
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 220.0, y: 280.0, z: 300.0}, 80.0, perlin_material_index))));
    
    
        let white_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
        let white_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{ albedo: white_texture }));
        let mut cube_spheres_indices: Vec<usize> = Vec::new();
        for _j in 0..cube_sphere_count {
            cube_spheres_indices.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Vector3::random_range_chacha(&mut rng, 0.0, 165.0), 10.0, white_material_index))));
        }
        let cube_spheres_bvh = BVHNode::from_index_list(&mut rng, service_locator.get_hittable_service_mut(), &mut cube_spheres_indices, start_time, end_time);
        let cube_spheres_bvh_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::BVHNode(cube_spheres_bvh));
        let cube_spheres_rotation = RotateY::new(service_locator.get_hittable_service_mut(), 15.0, cube_spheres_bvh_index);
        let cube_spheres_rotation_index = service_locator.get_hittable_service_mut().add_hittable(HittableEnum::RotateY(cube_spheres_rotation));
        let cube_spheres_translated = service_locator.get_hittable_service_mut().add_hittable( HittableEnum::Translate(Translate::new(Vector3 { x: -100.0, y: 270.0, z: 395.0 }, cube_spheres_rotation_index)));
        hittable_index_list.push(cube_spheres_translated);
        

        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, light_index_list);
    
        service_locator
    }
    
    fn final_scene_book_3(aspect_ratio: f64) -> ServiceLocator {
        let (mut rng, mut service_locator, mut hittable_index_list, mut light_index_list) = Self::empty_cornell_box_scene_prebuild(aspect_ratio);

        let white_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
        let white_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Lambertian(Lambertian{ albedo: white_texture }));
    
        let diffuse_light_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::DiffuseLight(DiffuseLight::from_color( &Color{x: 7.0, y: 7.0, z: 7.0 } )));
    
        let index_of_refraction = 1.5;
        let glass_material_index = service_locator.get_material_service_mut().add_material(MaterialEnum::Dielectric(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction}));
    
        let box_1 = BoxHittable::new(&mut rng, service_locator.get_hittable_service_mut(), Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 165.0, y: 330.0, z: 165.0}, white_material_index);
        let box_1_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::BoxHittable(box_1));
        let box_1_rotation = RotateY::new(service_locator.get_hittable_service_mut(), 15.0, box_1_index);
        let box_1_rotation_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::RotateY(box_1_rotation));
        let box_1_translated_index =  service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Translate(Translate::new(Vector3 { x: 265.0, y: 0.0, z: 295.0 }, box_1_rotation_index)));
        hittable_index_list.push(box_1_translated_index);
    
    
        hittable_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 190.0, y: 90.0, z: 190.0}, 90.0, glass_material_index))));
        light_index_list.push(service_locator.get_hittable_service_mut().add_hittable(HittableEnum::Sphere(Sphere::new(Point3{x: 190.0, y: 90.0, z: 190.0}, 90.0, diffuse_light_material_index))));
    
        build_acceleration_structures(&mut rng, &mut service_locator, hittable_index_list, light_index_list);

        service_locator
    }
}