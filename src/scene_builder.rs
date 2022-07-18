use std::sync::Arc;

use rand::{SeedableRng, Rng};
use rand_chacha::ChaCha20Rng;

use crate::{hittable::{HittableList, XYRect, YZRect, Hittable, FlipFace, XZRect, BoxHittable, RotateY, Translate, ConstantMedium}, material_service::{MaterialService, MaterialEnum}, camera::Camera, vector3::{Color, Point3, Vector3}, texture::{CheckerTexture, Texture, SolidColorTexture, NoiseTexture, ImageTexture}, sphere::Sphere, material::{Lambertian, Dielectric, Metal, DiffuseLight, Isotropic}, moving_sphere::MovingSphere, bvh_node::BVHNode, scene_service::{SceneService, self}};

pub struct SceneBuilder {

}

impl SceneBuilder {
    pub fn build_scene(scene_index: usize) -> (HittableList, MaterialService, HittableList, SceneService) {
        // Display Image
        let mut aspect_ratio = 16.0 / 9.0;
        let image_width: i64 = 500;
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

        match scene_index {
            0 => Self::random_spheres_scene(aspect_ratio, random_balls_count),
            1 => Self::random_moving_spheres_scene(aspect_ratio, random_balls_count),
            2 => Self::two_spheres_scene(aspect_ratio),
            3 => Self::two_perlin_spheres_scene(aspect_ratio, noise_points_count),
            4 => Self::earth_scene(aspect_ratio),
            5 => Self::simple_light_scene(aspect_ratio, noise_points_count),
            6 => { Self::empty_cornell_box_scene(aspect_ratio) },
            7 => { Self::cornell_box_two_diffuse_boxes_scene(aspect_ratio) },
            8 => { Self::cornell_box_two_smoke_boxes_scene(aspect_ratio) },
            9 => { Self::cornell_box_diffuse_metal_boxes_scene(aspect_ratio) },
            10 => { Self::final_scene_book_2(aspect_ratio, noise_points_count, cube_sphere_count) },
            11 => { Self::final_scene_book_3(aspect_ratio) },
            _ => panic!("Incorrect scene chosen!"),
        }
    }

    fn random_spheres_scene(aspect_ratio: f64, number_of_balls: i32) -> (HittableList, MaterialService, HittableList, SceneService) {
        let seed: u64 = 13371337;
        let mut rng = ChaCha20Rng::seed_from_u64(seed);
    
        let mut world = HittableList::default();
        let mut _lights = HittableList::default();
        let mut materials = MaterialService::new();
    
        let ground_texture: Arc<dyn Texture> = Arc::new(CheckerTexture::from_colors(&Color{x:0.2, y:0.3, z:0.1}, &Color{x:0.9, y:0.9, z:0.9}));
        let ground_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{albedo: ground_texture}));
        world.push(Sphere::new(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, ground_material_index));
    
        let index_of_refraction = 1.5;
        let glass_material_index = materials.add_material(MaterialEnum::Dielectric(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction}));
    
        for a in -number_of_balls..number_of_balls {
            for b in -number_of_balls..number_of_balls {
                let choose_mat = rng.gen::<f64>();
                let center = Point3{x: a as f64 + 0.9 * rng.gen::<f64>(), y: 0.2, z: b as f64 + 0.9 * rng.gen::<f64>()};
    
                if 0.9 < (center - Point3{x: 4.0, y: 0.2, z: 0.0}).length() {
                    if choose_mat < 0.8 {
                        let chosen_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&(Color::random_chacha(&mut rng) * Color::random_chacha(&mut rng))));
                        let chosen_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{albedo: chosen_texture}));
                        world.push(Sphere::new(center, 0.2, chosen_material_index));
                    } else if choose_mat < 0.95 {
                        let chosen_material_index = materials.add_material(MaterialEnum::Metal(Metal{albedo: Color::random_chacha(&mut rng), fuzz: rng.gen::<f64>()}));
                        world.push(Sphere::new(center, 0.2, chosen_material_index));
                    } else {
                        world.push(Sphere::new(center, 0.2, glass_material_index));
                    }
                }
            }
        }
        
        world.push(Sphere::new(Point3{x: 0.0, y: 1.0, z: 0.0}, 1.0, glass_material_index));
    
        let lambertian_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.4, y: 0.2, z: 0.1}));
        let lambertian_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{albedo: lambertian_texture}));
        world.push(Sphere::new(Point3{x: -4.0, y: 1.0, z: 0.0}, 1.0, lambertian_material_index));
    
        let metal_material_index = materials.add_material(MaterialEnum::Metal(Metal{albedo: Color{x: 0.7, y: 0.6, z: 0.5}, fuzz: 0.0}));
        world.push(Sphere::new(Point3{x: 4.0, y: 1.0, z: 0.0}, 1.0, metal_material_index));
    
        let background = Color{x:0.7, y:0.8, z: 1.0};
    
    
    
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
    


        (world, materials, _lights, SceneService::new(camera, background))
    }
    
    fn random_moving_spheres_scene(aspect_ratio: f64, number_of_balls: i32) -> (HittableList, MaterialService, HittableList, SceneService) {
        let seed: u64 = 13371337;
        let mut rng = ChaCha20Rng::seed_from_u64(seed);
    
        let mut world = HittableList::default();
        let mut materials = MaterialService::new();
        let lights = HittableList::default();
    
        let ground_texture: Arc<dyn Texture> = Arc::new(CheckerTexture::from_colors(&Color{x:0.2, y:0.3, z:0.1}, &Color{x:0.9, y:0.9, z:0.9}));
        let ground_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{albedo: ground_texture}));
        world.push(Sphere::new(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, ground_material_index));
    
        let index_of_refraction = 1.5;
        let glass_material_index = materials.add_material(MaterialEnum::Dielectric(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction}));
        
        for a in -number_of_balls..number_of_balls {
            for b in -number_of_balls..number_of_balls {
                let choose_mat = rng.gen::<f64>();
                let center = Point3{x: a as f64 + 0.9 * rng.gen::<f64>(), y: 0.2, z: b as f64 + 0.9 * rng.gen::<f64>()};
    
                if (center - Point3{x: 4.0, y: 0.2, z: 0.0}).length() > 0.9 {
                    if choose_mat < 0.8 {
                        let mut movement = Vector3::zero();
                        movement.y = rng.gen::<f64>() * 0.5;
    
                        let chosen_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&(Color::random_chacha(&mut rng) * Color::random_chacha(&mut rng))));
                        let chosen_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{albedo: chosen_texture}));
                        world.push(MovingSphere::new(0.2, center, center + movement,  chosen_material_index, 0.0, 1.0));
                    } else if choose_mat < 0.95 {
    
                        let chosen_material_index = materials.add_material(MaterialEnum::Metal(Metal{albedo: Color::random_chacha(&mut rng), fuzz: rng.gen::<f64>()}));
                        world.push(Sphere::new(center, 0.2, chosen_material_index));
                    } else {
    
                        world.push(Sphere::new(center, 0.2, glass_material_index));
                    }
                }
            }
        }
    
        world.push(Sphere::new(Point3{x: 0.0, y: 1.0, z: 0.0}, 1.0, glass_material_index));
    
        let lambertian_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.4, y: 0.2, z: 0.1}));
        let lambertian_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{albedo: lambertian_texture}));
        world.push(Sphere::new(Point3{x: -4.0, y: 1.0, z: 0.0}, 1.0, lambertian_material_index));
    
        let metal_material_index = materials.add_material(MaterialEnum::Metal(Metal{albedo: Color{x: 0.7, y: 0.6, z: 0.5}, fuzz: 0.0}));
        world.push(Sphere::new(Point3{x: 4.0, y: 1.0, z: 0.0}, 1.0, metal_material_index));
    
        let background = Color{x:0.7, y:0.8, z: 1.0};
    
        // Camera
        let look_from = Point3{x: 13.0, y: 2.0, z: 3.0 };
        let look_at = Point3{x: 0.0, y: 0.0, z: 0.0};
        let v_up = Vector3{x: 0.0, y:1.0, z:0.0};
        let dist_to_focus = 15.0;
        let aperture = 0.1;
        let time_0: f64 = 0.0;
        let time_1: f64 = 1.0;
        let camera = Camera::new(look_from, look_at, v_up,20.0, aspect_ratio, aperture, dist_to_focus, time_0, time_1);
    
        (world, materials, lights, SceneService::new(camera, background))
    }
    
    fn two_spheres_scene(aspect_ratio: f64) -> (HittableList, MaterialService, HittableList, SceneService) {
        let mut world = HittableList::default();
        let mut materials = MaterialService::new();
        let lights = HittableList::default();
    
        let checker_texture: Arc<dyn Texture> = Arc::new(CheckerTexture::from_colors(&Color{x:0.2, y:0.3, z:0.1}, &Color{x:0.9, y:0.9, z:0.9}));
        let checker_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{albedo: checker_texture}));
        world.push(Sphere::new(Point3{x: 0.0, y: -10.0, z: 0.0}, 10.0, checker_material_index));
        world.push(Sphere::new(Point3{x: 0.0, y: 10.0, z: 0.0}, 10.0, checker_material_index));
    
        let background = Color{x:0.7, y:0.8, z: 1.0};
    
        // Camera
        let look_from = Point3{x: 13.0, y: 2.0, z: 3.0 };
        let look_at = Point3{x: 0.0, y: 0.0, z: 0.0};
        let v_up = Vector3{x: 0.0, y:1.0, z:0.0};
        let dist_to_focus = 15.0;
        let aperture = 0.05;
        let time_0: f64 = 0.0;
        let time_1: f64 = 1.0;
        let camera = Camera::new(look_from, look_at, v_up,20.0, aspect_ratio, aperture, dist_to_focus, time_0, time_1);
    
    
        (world, materials, lights, SceneService::new(camera, background))
    }
    
    fn two_perlin_spheres_scene(aspect_ratio: f64, element_count: u32) -> (HittableList, MaterialService, HittableList, SceneService) {
        let mut world = HittableList::default();
        let mut materials = MaterialService::new();
        let lights = HittableList::default();
    
        // The Noise Texture runs pretty deep
        // I just need some determinism, not all the way
        let mut thread_rng = rand::thread_rng();
        let perlin_texture: Arc<dyn Texture> = Arc::new(NoiseTexture::new(&mut thread_rng, element_count, 4.0));
        let perlin_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{albedo: perlin_texture}));
        world.push(Sphere::new(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, perlin_material_index));
        world.push(Sphere::new(Point3{x: 0.0, y: 2.0, z: 0.0}, 2.0, perlin_material_index));
    
        let background = Color{x:0.7, y:0.8, z: 1.0};
    
        // Camera
        let look_from = Point3{x: 13.0, y: 2.0, z: 3.0 };
        let look_at = Point3{x: 0.0, y: 0.0, z: 0.0};
        let v_up = Vector3{x: 0.0, y:1.0, z:0.0};
        let dist_to_focus = 15.0;
        let aperture = 0.05;
        let time_0: f64 = 0.0;
        let time_1: f64 = 1.0;
        let camera = Camera::new(look_from, look_at, v_up,20.0, aspect_ratio, aperture, dist_to_focus, time_0, time_1);
    
    
        (world, materials, lights, SceneService::new(camera, background))
    }
    
    fn earth_scene(aspect_ratio: f64) -> (HittableList, MaterialService, HittableList, SceneService) {
        let mut world = HittableList::default();
        let mut materials = MaterialService::new();
        let mut _lights = HittableList::default();
    
        let texture: Arc<dyn Texture> = Arc::new(ImageTexture::new("earthmap.png"));
        let material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{albedo: texture}));
        world.push(Sphere::new(Vector3::new(0.0, 0.0, 0.0), 2.0, material_index));
    
        let background = Color{x:0.7, y:0.8, z: 1.0};
    
        // Camera
        let look_from = Point3{x: 13.0, y: 2.0, z: 3.0 };
        let look_at = Point3{x: 0.0, y: 0.0, z: 0.0};
        let v_up = Vector3{x: 0.0, y:1.0, z:0.0};
        let dist_to_focus = 15.0;
        let aperture = 0.05;
        let time_0: f64 = 0.0;
        let time_1: f64 = 1.0;
        let camera = Camera::new(look_from, look_at, v_up,20.0, aspect_ratio, aperture, dist_to_focus, time_0, time_1);
    
        (world, materials, _lights, SceneService::new(camera, background))
    }
    
    fn simple_light_scene(aspect_ratio: f64, element_count: u32) -> (HittableList, MaterialService, HittableList, SceneService) {
        let mut world = HittableList::default();
        let mut materials = MaterialService::new();
        let mut _lights = HittableList::default();
    
        // The Noise Texture runs pretty deep
        // I just need some determinism, not all the way
        let mut thread_rng = rand::thread_rng();
        let perlin_texture: Arc<dyn Texture> = Arc::new(NoiseTexture::new(&mut thread_rng, element_count, 4.0));
        let perlin_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{albedo: perlin_texture}));
        world.push(Sphere::new(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, perlin_material_index));
        world.push(Sphere::new(Point3{x: 0.0, y: 2.0, z: 0.0}, 2.0, perlin_material_index));
    
        let diffuse_light_material_index = materials.add_material(MaterialEnum::DiffuseLight(DiffuseLight::from_color(&Color{x: 4.0, y: 4.0, z: 4.0 })));
        world.push(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, diffuse_light_material_index));
        world.push(Sphere::new(Point3{x: 0.0, y: 7.0, z: 0.0}, 2.0, diffuse_light_material_index));
    
        let background = Color{x:0.0, y:0.0, z: 0.0};
    
        // Camera
        let look_from = Point3{x: 26.0, y: 3.0, z: 6.0 };
        let look_at = Point3{x: 0.0, y: 2.0, z: 0.0};
        let v_up = Vector3{x: 0.0, y:1.0, z:0.0};
        let dist_to_focus = 15.0;
        let aperture = 0.05;
        let time_0: f64 = 0.0;
        let time_1: f64 = 1.0;
        let camera = Camera::new(look_from, look_at, v_up,20.0, aspect_ratio, aperture, dist_to_focus, time_0, time_1);
    
    
        (world, materials, _lights, SceneService::new(camera, background))
    }
    
    fn empty_cornell_box_scene(aspect_ratio: f64) -> (HittableList, MaterialService, HittableList, SceneService) {
        let mut world = HittableList::default();
        let mut materials = MaterialService::new();
        let mut lights = HittableList::default();
    
        let red_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.65, y: 0.05, z: 0.05}));
        let red_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{ albedo: red_texture }));
    
        let white_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
        let white_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{ albedo: white_texture }));
    
        let green_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.12, y: 0.45, z: 0.15}));
        let green_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{ albedo: green_texture }));
    
        let diffuse_light_material_index = materials.add_material(MaterialEnum::DiffuseLight(DiffuseLight::from_color( &Color{x: 15.0, y: 15.0, z: 15.0 } )));
    
    
        let green_wall: Arc<dyn Hittable> = Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green_material_index));
        world.push(FlipFace::new(&green_wall));
    
        world.push(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red_material_index));
    
        let white_wall_1: Arc<dyn Hittable> = Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white_material_index));
        world.push(FlipFace::new(&white_wall_1));
    
        world.push(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white_material_index));
    
        let white_wall_3: Arc<dyn Hittable> = Arc::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white_material_index));
        world.push(FlipFace::new(&white_wall_3));
    
    
        let unflipped_light: Arc<dyn Hittable> = Arc::new(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, diffuse_light_material_index));
        world.push(FlipFace::new(&unflipped_light));
        lights.push(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, diffuse_light_material_index));
    
        let background = Color{x:0.0, y:0.0, z: 0.0};
    
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
    
        (world, materials, lights, SceneService::new(camera, background))
    }
    
    fn cornell_box_two_diffuse_boxes_scene(aspect_ratio: f64) -> (HittableList, MaterialService, HittableList, SceneService) {
        let (mut world, mut materials, lights, scene_service) = Self::empty_cornell_box_scene(aspect_ratio);
        
        let white_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
        let white_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{ albedo: white_texture }));
    
        let box_1 = BoxHittable::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 165.0, y: 330.0, z: 165.0}, white_material_index);
        let box_1_arc : Arc<dyn Hittable> = Arc::new(box_1);
        let box_1_rotation: Arc<dyn Hittable> = Arc::new(RotateY::new(15.0, &box_1_arc));
        let box_1_translated = Translate::new(Vector3 { x: 265.0, y: 0.0, z: 295.0 }, &box_1_rotation);
        world.push(box_1_translated);
    
        let box_2 = BoxHittable::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 165.0, y: 165.0, z: 165.0}, white_material_index);
        let box_2_arc : Arc<dyn Hittable> = Arc::new(box_2);
        let box_2_rotation: Arc<dyn Hittable> = Arc::new(RotateY::new(-18.0, &box_2_arc));
        let box_2_translated = Translate::new(Vector3 { x: 130.0, y: 0.0, z: 65.0 }, &box_2_rotation);
        world.push(box_2_translated);
    
        (world, materials, lights, scene_service)
    }
    
    fn cornell_box_diffuse_metal_boxes_scene(aspect_ratio: f64) -> (HittableList, MaterialService, HittableList, SceneService) {
        let (mut world, mut materials, lights, scene_service) = Self::empty_cornell_box_scene(aspect_ratio);
        
        let metal_material_index = materials.add_material(MaterialEnum::Metal(Metal::new(Color{x: 0.8, y: 0.85, z: 0.88}, 0.0)));
    
        let white_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
        let white_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{ albedo: white_texture }));
    
        let box_1 = BoxHittable::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 165.0, y: 330.0, z: 165.0}, metal_material_index);
        let box_1_arc : Arc<dyn Hittable> = Arc::new(box_1);
        let box_1_rotation: Arc<dyn Hittable> = Arc::new(RotateY::new(15.0, &box_1_arc));
        let box_1_translated = Translate::new(Vector3 { x: 265.0, y: 0.0, z: 295.0 }, &box_1_rotation);
        world.push(box_1_translated);
    
        let box_2 = BoxHittable::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 165.0, y: 165.0, z: 165.0}, white_material_index);
        let box_2_arc : Arc<dyn Hittable> = Arc::new(box_2);
        let box_2_rotation: Arc<dyn Hittable> = Arc::new(RotateY::new(-18.0, &box_2_arc));
        let box_2_translated = Translate::new(Vector3 { x: 130.0, y: 0.0, z: 65.0 }, &box_2_rotation);
        world.push(box_2_translated);
    
        (world, materials, lights, scene_service)
    }
    
    fn cornell_box_two_smoke_boxes_scene(aspect_ratio: f64) -> (HittableList, MaterialService, HittableList, SceneService) {
        let mut world = HittableList::default();
        let mut materials = MaterialService::new();
        let mut lights = HittableList::default();
    
        let red_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.65, y: 0.05, z: 0.05}));
        let red_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{ albedo: red_texture }));
    
        let white_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
        let white_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{ albedo: white_texture }));
    
        let green_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.12, y: 0.45, z: 0.15}));
        let green_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{ albedo: green_texture }));
    
        let diffuse_light_material_index = materials.add_material(MaterialEnum::DiffuseLight(DiffuseLight::from_color( &Color{x: 7.0, y: 7.0, z: 7.0 } )));
    
        let dark_phase_function_index = materials.add_material(MaterialEnum::Isotropic(Isotropic::from_color( &Color{x: 0.0, y: 0.0, z: 0.0} )));
        let light_phase_function_index = materials.add_material(MaterialEnum::Isotropic(Isotropic::from_color( &Color{x: 1.0, y: 1.0, z: 1.0} )));
    
        let green_wall: Arc<dyn Hittable> = Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green_material_index));
        world.push(FlipFace::new(&green_wall));
    
        world.push(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red_material_index));
    
        let white_wall_1: Arc<dyn Hittable> = Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white_material_index));
        world.push(FlipFace::new(&white_wall_1));
    
        world.push(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white_material_index));
    
        let white_wall_3: Arc<dyn Hittable> = Arc::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white_material_index));
        world.push(FlipFace::new(&white_wall_3));
    
    
        let unflipped_light: Arc<dyn Hittable> = Arc::new(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, diffuse_light_material_index));
        world.push(FlipFace::new(&unflipped_light));
        lights.push(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, diffuse_light_material_index));
    
    
        let box_1 = BoxHittable::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 165.0, y: 330.0, z: 165.0}, white_material_index);
        let box_1_arc : Arc<dyn Hittable> = Arc::new(box_1);
        let box_1_rotation: Arc<dyn Hittable> = Arc::new(RotateY::new(15.0, &box_1_arc));
        let box_1_translated: Arc <dyn Hittable> = Arc::new(Translate::new(Vector3 { x: 265.0, y: 0.0, z: 295.0 }, &box_1_rotation));
        let box_1_smoke = ConstantMedium::new(&box_1_translated, dark_phase_function_index, 0.01);
        world.push(box_1_smoke);
    
        let box_2 = BoxHittable::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 165.0, y: 165.0, z: 165.0}, white_material_index);
        let box_2_arc : Arc<dyn Hittable> = Arc::new(box_2);
        let box_2_rotation: Arc<dyn Hittable> = Arc::new(RotateY::new(-18.0, &box_2_arc));
        let box_2_translated: Arc<dyn Hittable> = Arc::new(Translate::new(Vector3 { x: 130.0, y: 0.0, z: 65.0 }, &box_2_rotation));
        let box_2_smoke = ConstantMedium::new(&box_2_translated, light_phase_function_index, 0.01);
        world.push(box_2_smoke);
    
        let background = Color{x:0.0, y:0.0, z: 0.0};
    
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
    
    
    
        (world, materials, lights, SceneService::new(camera, background))
    }
    
    fn final_scene_book_2(aspect_ratio: f64, perlin_element_count: u32, cube_sphere_count: u32) -> (HittableList, MaterialService, HittableList, SceneService) {
        let seed: u64 = 919;
        let mut rng = ChaCha20Rng::seed_from_u64(seed);
        
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
        
        let mut objects = HittableList::default();
        let mut lights = HittableList::default();
        let mut materials = MaterialService::new();
        let mut floor_cubes = HittableList::default();
    
        let ground_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x:0.48, y:0.83, z:0.53}));
        let ground_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{albedo: ground_texture}));
    
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
    
                floor_cubes.push(
                    BoxHittable::new(
                        Vector3{x: x0, y: y0, z: z0}, 
                        Vector3{x: x1, y: y1, z: z1}, 
                        ground_material_index
                        )
                    );
            }
        }
    
        let floor_cubes_bvh = BVHNode::from_hittable_list(&mut floor_cubes, camera.get_start_time(), camera.get_end_time());
        objects.push(floor_cubes_bvh);
    
    
    
    
    
        let diffuse_light_material_index = materials.add_material(MaterialEnum::DiffuseLight(DiffuseLight::from_color( &Color{x: 7.0, y: 7.0, z: 7.0 } )));
        let unflipped_light: Arc<dyn Hittable> = Arc::new(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, diffuse_light_material_index));
        objects.push(FlipFace::new(&unflipped_light));
        lights.push(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, diffuse_light_material_index));
    
        let center_0 = Vector3{x: 400.0, y: 400.0, z: 200.0};
        let center_1 = center_0 + Vector3{x: 30.0, y: 0.0, z: 0.0};
        let moving_sphere_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.7, y: 0.3, z: 0.1}));
        let moving_sphere_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{ albedo: moving_sphere_texture }));
        objects.push(MovingSphere{ radius: 50.0, center_0, center_1, material: moving_sphere_material_index, time_0: 0.0, time_1: 1.0 });
    
    
        let index_of_refraction = 1.5;
        let glass_material_index = materials.add_material(MaterialEnum::Dielectric(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction}));
        objects.push(Sphere::new(Point3{x: 260.0, y: 150.0, z: 45.0}, 50.0, glass_material_index));
        lights.push(Sphere::new(Point3{x: 260.0, y: 150.0, z: 45.0}, 50.0, diffuse_light_material_index));
    
        let metal_material_index = materials.add_material(MaterialEnum::Metal(Metal{albedo: Color{x: 0.8, y: 0.8, z: 0.9}, fuzz: 1.0}));
        objects.push(Sphere::new(Point3{x: 0.0, y: 150.0, z: 145.0}, 50.0, metal_material_index));
    
        // Volume sphere
        let boundary = Sphere::new(Point3{x: 360.0, y: 150.0, z: 145.0}, 70.0, glass_material_index);
        let boundary_arc: Arc<dyn Hittable> = Arc::new(boundary);
        objects.push_arc(&boundary_arc);
    
        let blue_phase_function_index = materials.add_material(MaterialEnum::Isotropic(Isotropic::from_color( &Color{x: 0.2, y: 0.4, z: 0.9} )));
        let volume_sphere= ConstantMedium::new(&boundary_arc, blue_phase_function_index, 0.2);
        objects.push(volume_sphere);
    
        let global_phase_function_index = materials.add_material(MaterialEnum::Isotropic(Isotropic::from_color( &Color{x: 1.0, y: 1.0, z: 1.0} )));
        let global_volume_sphere: Arc<dyn Hittable> = Arc::new(Sphere::new(Point3{x: 0.0, y: 0.0, z: 0.0}, 5000.0, glass_material_index));
        let global_volume = ConstantMedium::new(&global_volume_sphere, global_phase_function_index, 0.0001);
        objects.push(global_volume);
    
        let earth_texture: Arc<dyn Texture> = Arc::new(ImageTexture::new("earthmap.png"));
        let earth_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{albedo: earth_texture}));
        objects.push(Sphere::new(Vector3::new(400.0, 200.0, 400.0), 100.0, earth_material_index));
    
        
        // The Noise Texture runs pretty deep
        // I just need some determinism, not all the way
        let mut thread_rng = rand::thread_rng();
        let perlin_texture: Arc<dyn Texture> = Arc::new(NoiseTexture::new(&mut thread_rng, perlin_element_count, 0.1));
        let perlin_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{albedo: perlin_texture}));
        objects.push(Sphere::new(Point3{x: 220.0, y: 280.0, z: 300.0}, 80.0, perlin_material_index));
    
    
        let white_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
        let white_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{ albedo: white_texture }));
        let mut cube_spheres = HittableList::default();
        for _j in 0..cube_sphere_count {
            cube_spheres.push(Sphere::new(Vector3::random_range_chacha(&mut rng, 0.0, 165.0), 10.0, white_material_index));
        }
        let cube_spheres_bvh = BVHNode::from_hittable_list(&mut cube_spheres, camera.get_start_time(), camera.get_end_time());
        let cube_spheres_arc: Arc <dyn Hittable> = Arc::new(cube_spheres_bvh);
        let cube_spheres_rotation: Arc<dyn Hittable> = Arc::new(RotateY::new(15.0, &cube_spheres_arc));
        let cube_spheres_translated: Arc<dyn Hittable> = Arc::new(Translate::new(Vector3 { x: -100.0, y: 270.0, z: 395.0 }, &cube_spheres_rotation));
        objects.push_arc(&cube_spheres_translated);
    
        let background = Color{x:0.0, y:0.0, z: 0.0};
    
    
        (objects, materials, lights, SceneService::new(camera, background))
    }
    
    fn final_scene_book_3(aspect_ratio: f64) -> (HittableList, MaterialService, HittableList, SceneService) {
        let (mut world, mut materials, mut lights, scene_service) = Self::empty_cornell_box_scene(aspect_ratio);
        
        let white_texture: Arc<dyn Texture> = Arc::new(SolidColorTexture::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
        let white_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{ albedo: white_texture }));
    
        let diffuse_light_material_index = materials.add_material(MaterialEnum::DiffuseLight(DiffuseLight::from_color( &Color{x: 7.0, y: 7.0, z: 7.0 } )));
    
        let index_of_refraction = 1.5;
        let glass_material_index = materials.add_material(MaterialEnum::Dielectric(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction}));
    
        let box_1 = BoxHittable::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 165.0, y: 330.0, z: 165.0}, white_material_index);
        let box_1_arc : Arc<dyn Hittable> = Arc::new(box_1);
        let box_1_rotation: Arc<dyn Hittable> = Arc::new(RotateY::new(15.0, &box_1_arc));
        let box_1_translated = Translate::new(Vector3 { x: 265.0, y: 0.0, z: 295.0 }, &box_1_rotation);
        world.push(box_1_translated);
    
        world.push(Sphere::new(Point3{x: 190.0, y: 90.0, z: 190.0}, 90.0, glass_material_index));
        lights.push(Sphere::new(Point3{x: 190.0, y: 90.0, z: 190.0}, 90.0, diffuse_light_material_index));
    
        (world, materials, lights, scene_service)
    }
}