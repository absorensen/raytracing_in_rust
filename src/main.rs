extern crate minifb;
use minifb::{Key, ScaleMode, Window, WindowOptions, clamp};
use pdf::{PDF, HittablePDF, MixturePDF};
// Look into performance optimization of the RNG
use rand::prelude::*;
use rand_chacha::{ChaCha20Rng};
use texture::{SolidColor, NoiseTexture};
use std::f64;
use std::sync::Arc;
use std::time::{Instant};
use rayon::prelude::*;

mod ortho_normal_base;
mod vector3;
mod ray;
mod sphere;
mod moving_sphere;
mod hittable;
mod camera;
mod material;
mod aabb;
mod bvh_node;
mod texture;
mod perlin;
mod pdf;

use texture::{Texture, CheckerTexture, ImageTexture};
use bvh_node::{BVHNode};
use vector3::{Vector3, Point3, Color};
use ray::Ray;
use sphere::Sphere;
use hittable::{Hittable, HittableList, XYRect, XZRect, YZRect, BoxHittable, RotateY, Translate, ConstantMedium, FlipFace, HitRecord};
use moving_sphere::MovingSphere;
use camera::Camera;
use material::{Lambertian, Metal, Dielectric, Material, DiffuseLight, Isotropic, ScatterRecord, DefaultMaterial, MaterialService, MaterialEnum};


fn random_spheres_scene(aspect_ratio: f64, number_of_balls: i32) -> (HittableList, MaterialService, HittableList, Camera, Color) {
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
                let chosen_material : Arc<dyn Material>;
                if choose_mat < 0.8 {
                    let chosen_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&(Color::random_chacha(&mut rng) * Color::random_chacha(&mut rng))));
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

    let lambertian_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.4, y: 0.2, z: 0.1}));
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


    (world, materials, _lights, camera, background)
}

fn random_moving_spheres_scene(aspect_ratio: f64, number_of_balls: i32) -> (HittableList, MaterialService, HittableList, Camera, Color) {
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

                    let chosen_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&(Color::random_chacha(&mut rng) * Color::random_chacha(&mut rng))));
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

    let lambertian_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.4, y: 0.2, z: 0.1}));
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

    (world, materials, lights, camera, background)
}

fn two_spheres_scene(aspect_ratio: f64) -> (HittableList, MaterialService, HittableList, Camera, Color) {
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


    (world, materials, lights, camera, background)
}

fn two_perlin_spheres_scene(aspect_ratio: f64, element_count: u32) -> (HittableList, MaterialService, HittableList, Camera, Color) {
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


    (world, materials, lights, camera, background)
}

fn earth_scene(aspect_ratio: f64) -> (HittableList, MaterialService, HittableList, Camera, Color) {
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

    (world, materials, _lights, camera, background)
}

fn simple_light_scene(aspect_ratio: f64, element_count: u32) -> (HittableList, MaterialService, HittableList, Camera, Color) {
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


    (world, materials, _lights, camera, background)
}

fn empty_cornell_box_scene(aspect_ratio: f64) -> (HittableList, MaterialService, HittableList, Camera, Color) {
    let mut world = HittableList::default();
    let mut materials = MaterialService::new();
    let mut lights = HittableList::default();

    let red_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.65, y: 0.05, z: 0.05}));
    let red_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{ albedo: red_texture }));

    let white_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
    let white_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{ albedo: white_texture }));

    let green_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.12, y: 0.45, z: 0.15}));
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

    (world, materials, lights, camera, background)
}

fn cornell_box_two_diffuse_boxes_scene(aspect_ratio: f64) -> (HittableList, MaterialService, HittableList, Camera, Color) {
    let (mut world, mut materials, lights, camera, background) = empty_cornell_box_scene(aspect_ratio);
    
    let white_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
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

    (world, materials, lights, camera, background)
}

fn cornell_box_diffuse_metal_boxes_scene(aspect_ratio: f64) -> (HittableList, MaterialService, HittableList, Camera, Color) {
    let (mut world, mut materials, lights, camera, background) = empty_cornell_box_scene(aspect_ratio);
    
    let metal_material_index = materials.add_material(MaterialEnum::Metal(Metal::new(Color{x: 0.8, y: 0.85, z: 0.88}, 0.0)));

    let white_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
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

    (world, materials, lights, camera, background)
}

fn cornell_box_two_smoke_boxes_scene(aspect_ratio: f64) -> (HittableList, MaterialService, HittableList, Camera, Color) {
    let mut world = HittableList::default();
    let mut materials = MaterialService::new();
    let mut lights = HittableList::default();

    let red_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.65, y: 0.05, z: 0.05}));
    let red_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{ albedo: red_texture }));

    let white_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
    let white_material_index = materials.add_material(MaterialEnum::Lambertian(Lambertian{ albedo: white_texture }));

    let green_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.12, y: 0.45, z: 0.15}));
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



    (world, materials, lights, camera, background)
}

fn final_scene_book_2(aspect_ratio: f64, perlin_element_count: u32, cube_sphere_count: u32) -> (HittableList, MaterialService, HittableList, Camera, Color) {
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

    let ground_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x:0.48, y:0.83, z:0.53}));
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
    let moving_sphere_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.7, y: 0.3, z: 0.1}));
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


    let white_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
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


    (objects, materials, lights, camera, background)
}

fn final_scene_book_3(aspect_ratio: f64) -> (HittableList, MaterialService, HittableList, Camera, Color) {
    let (mut world, mut materials, mut lights, camera, background) = empty_cornell_box_scene(aspect_ratio);
    
    let white_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
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

    (world, materials, lights, camera, background)
}

// Try splitting this into a mixture and non-mixture pdfs function, as some scenes don't have lights (though they should)
fn ray_color_recursive(
    rng: &mut ThreadRng,
    materials_service: &MaterialService,
    background: &Color, 
    ray: &Ray, 
    world: & dyn Hittable, 
    lights: &Arc<dyn Hittable>,
    lights_count: usize, 
    depth: i64) -> Color {

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec:HitRecord = HitRecord::default();
    
    
    if !world.hit(rng, ray, 0.001, f64::MAX, &mut rec) {
        return *background;
    }


    let mut scatter_record= ScatterRecord::default();
    let emitted: Color = materials_service.emission(ray, &rec, rec.u, rec.v, &rec.position);
    
    if !materials_service.scatter(rng, ray, &rec, &mut scatter_record) {
        return emitted;
    }

    if scatter_record.is_specular {
        return scatter_record.attenuation * ray_color_recursive(rng, materials_service, background, &scatter_record.specular_ray, world, lights, lights_count, depth - 1);
    }

    if 0 < lights_count {
        let light_pdf: Box<dyn PDF> = Box::new(HittablePDF::new(lights, &rec.position));
        let other_pdf: Box<dyn PDF> = 
            if scatter_record.pdf.is_some() {  // Get rid of this whole option<Arc> thing
                scatter_record.pdf.expect("Failed to unwrap pdf")
            } else {
                Box::new(HittablePDF::new(lights, &rec.position))
            };
        let mixture_pdf: MixturePDF = MixturePDF::new( light_pdf, other_pdf ); 
    
        let scattered = Ray::new(rec.position, mixture_pdf.generate(rng), ray.time);
        let pdf_val = mixture_pdf.value(rng, &scattered.direction);
    
        return 
            emitted + 
            scatter_record.attenuation * 
            materials_service.scattering_pdf(rng, ray, &rec, &scattered) *
            ray_color_recursive(rng, materials_service, background, &scattered, world, lights, lights_count, depth - 1) /
            pdf_val;
    } else {
        let pdf: Box<dyn PDF> = scatter_record.pdf.expect("Failed to unwrap pdf");
        let scattered = Ray::new(rec.position, pdf.generate(rng), ray.time);
        let pdf_val = pdf.value(rng, &scattered.direction);

        return 
            emitted + 
            scatter_record.attenuation * 
            materials_service.scattering_pdf(rng, ray, &rec, &scattered) *
            ray_color_recursive(rng, materials_service, background, &scattered, world, lights, lights_count, depth - 1) /
            pdf_val;
    }

}

fn render_pixel(
    rng: &mut ThreadRng, 
    materials_service: &MaterialService,
    background: &Color, 
    pixel_index: i64, 
    image_width: i64, 
    image_height: i64, 
    samples_per_pixel: i64, 
    camera: &Camera, 
    world: &dyn Hittable, 
    lights: &Arc<dyn Hittable>, 
    lights_count: usize,
    max_depth: i64, 
    scale: f64, 
    use_parallel: bool) 
    -> Vector3 {
    let column_index = pixel_index % image_width;
    let row_index = pixel_index / image_width;

    let mut color_buffer = Color{x: 0.0, y: 0.0, z: 0.0};
    if use_parallel {
        let seeds: Vec<(f64, f64)> = (0..samples_per_pixel).into_iter().map(|_| (rng.gen::<f64>(), rng.gen::<f64>()) ).collect();
        color_buffer = seeds.into_par_iter().map(|(seed0, seed1)| {
            let mut rng = rand::thread_rng();
            let u = (column_index as f64 + seed0 ) / ((image_width - 1) as f64);
            let v = (row_index as f64 + seed1 ) / ((image_height - 1) as f64);
            let ray = camera.get_ray(&mut rng, u, v);
            ray_color_recursive(&mut rng, materials_service, background, &ray, world, lights, lights_count, max_depth)
        }).sum();
    } else {
        for _sample_index in 0..samples_per_pixel {
            let u = (column_index as f64 + rng.gen::<f64>() ) / ((image_width - 1) as f64);
            let v = (row_index as f64 + rng.gen::<f64>() ) / ((image_height - 1) as f64);
            let ray = camera.get_ray(rng, u, v);
            color_buffer += ray_color_recursive(rng, materials_service, background, &ray, world, lights, lights_count, max_depth);
        }
    }

    if color_buffer.x != color_buffer.x { color_buffer.x = 0.0; }
    if color_buffer.y != color_buffer.y { color_buffer.y = 0.0; }
    if color_buffer.z != color_buffer.z { color_buffer.z = 0.0; }

    // Try and apply this scaling to the colors before summation
    color_buffer.x = 255.999 * clamp(0.0, (scale * color_buffer.x).sqrt(), 0.999);
    color_buffer.y = 255.999 * clamp(0.0, (scale * color_buffer.y).sqrt(), 0.999);
    color_buffer.z = 255.999 * clamp(0.0, (scale * color_buffer.z).sqrt(), 0.999);

    color_buffer
}

// TODO:
// Injest config files
// Project restructuring
// Unit testing
// Performance optimization
// Reduce the amount of ARC
// Use texture indices
// Use hittable indices
// Replace vector3 with nalgebra or something numpy-like
// Change color to its own type
// Try to convert from dynamic dispatch to static dispatch
// Try to convert to SIMD
// Refactor
// Enforce fused multiply-adds
// Gather all the scene relevant stuff into a scene struct in a different file. This is getting ridiculous
fn main() {
    // Display Image
    let mut aspect_ratio = 16.0 / 9.0;
    let image_width: i64 = 500;
    let mut image_height = ((image_width as f64) / aspect_ratio) as i64;
    image_height = image_height + image_height % 2;
    let output_path = "output.png";

    // Render Settings
    let samples_per_pixel = 100;
    let max_depth = 10;

    // Compute Settings
    let run_parallel = true;
    let run_samples_parallel = true;



    // Scene
    let random_balls_count = 11;
    let noise_points_count = 256;
    let cube_sphere_count = 1000;
    let scene_index = 11;
    let (mut world, materials_service, lights, camera, background) = match scene_index {
        0 => random_spheres_scene(aspect_ratio, random_balls_count),
        1 => random_moving_spheres_scene(aspect_ratio, random_balls_count),
        2 => two_spheres_scene(aspect_ratio),
        3 => two_perlin_spheres_scene(aspect_ratio, noise_points_count),
        4 => earth_scene(aspect_ratio),
        5 => simple_light_scene(aspect_ratio, noise_points_count),
        6 => {
            aspect_ratio = 1.0;
            image_height = ((image_width as f64) / aspect_ratio) as i64;
            image_height = image_height + image_height % 2;
            empty_cornell_box_scene(aspect_ratio)
        },
        7 => {
            aspect_ratio = 1.0;
            image_height = ((image_width as f64) / aspect_ratio) as i64;
            image_height = image_height + image_height % 2;
            cornell_box_two_diffuse_boxes_scene(aspect_ratio)
        },
        8 => {
            aspect_ratio = 1.0;
            image_height = ((image_width as f64) / aspect_ratio) as i64;
            image_height = image_height + image_height % 2;
            cornell_box_two_smoke_boxes_scene(aspect_ratio)
        },
        9 => {
            aspect_ratio = 1.0;
            image_height = ((image_width as f64) / aspect_ratio) as i64;
            image_height = image_height + image_height % 2;
            cornell_box_diffuse_metal_boxes_scene(aspect_ratio)
        },
        10 => {
            aspect_ratio = 1.0;
            image_height = ((image_width as f64) / aspect_ratio) as i64;
            image_height = image_height + image_height % 2;
            final_scene_book_2(aspect_ratio, noise_points_count, cube_sphere_count)
        },
        11 => {
            aspect_ratio = 1.0;
            image_height = ((image_width as f64) / aspect_ratio) as i64;
            image_height = image_height + image_height % 2;
            final_scene_book_3(aspect_ratio)
        },
        _ => panic!("Incorrect scene chosen!"),
    };
    let world = BVHNode::from_hittable_list(&mut world, camera.get_start_time(), camera.get_end_time());
    let lights_count = lights.len();
    let lights_arc : Arc<dyn Hittable> = Arc::new( lights );


    let scale = 1.0 / (samples_per_pixel as f64);

    let now = Instant::now();
    let total_pixels = image_height * image_width;
    let image: Vec<Vector3> = 
    if run_parallel {
        (0..total_pixels).into_par_iter().map(|pixel_index:i64| {
            let mut rng = rand::thread_rng();
            render_pixel(
                &mut rng, 
                &materials_service, 
                &background, 
                pixel_index, 
                image_width, 
                image_height, 
                samples_per_pixel, 
                &camera, 
                &world, 
                &lights_arc, 
                lights_count, 
                max_depth, 
                scale, 
                run_samples_parallel
            )
        }).collect()
    } else {
        let mut rng = rand::thread_rng();
        (0..total_pixels).into_iter().map(|pixel_index:i64| {
            render_pixel(
                &mut rng, 
                &materials_service, 
                &background, 
                pixel_index, 
                image_width, 
                image_height, 
                samples_per_pixel, 
                &camera, 
                &world, 
                &lights_arc, 
                lights_count, 
                max_depth, 
                scale, 
                run_samples_parallel
            )
        }).collect()
    };
    println!("{} seconds elapsed", now.elapsed().as_millis() as f64 * 0.001);

    let zero = Vector3{x: 0.0, y: 0.0, z: 0.0};
    let mut final_image: Vec<Vector3> = vec![zero; image.len()];

    for row_index in 0..image_height {
        for column_index in 0..(image_width / 2) {
            let column_index_left = (row_index * image_width + column_index) as usize;
            let column_index_right = (row_index * image_width + (image_width - column_index - 1)) as usize;
            final_image[column_index_left] = image[column_index_right];
            final_image[column_index_right] = image[column_index_left];
        }
    }

    
    let window_buffer: Vec<u32> = final_image
        .iter()
        .map(|v| ((v.x as u32) << 16) | ((v.y as u32) << 8) | v.z as u32)
        .rev()
        .collect();

    let mut window = Window::new(
        "Ray Tracing in Rust - Press ESC to exit",
        image_width as usize,
        image_height as usize,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Center,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");



    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
        .update_with_buffer(
            &window_buffer,
            image_width as usize,
            image_height as usize,
        )
        .unwrap();
    }

    let mut horizontally_flipped_image: Vec<Vector3> = vec![zero; image.len()];
    for row_index in 0..(image_height / 2) {
        for column_index in 0..image_width {
            let row_index_top = (row_index * image_width + column_index) as usize;
            let row_index_bottom = ((image_height - row_index - 1) * image_width + column_index) as usize;
            horizontally_flipped_image[row_index_top] = image[row_index_bottom];
            horizontally_flipped_image[row_index_bottom] = image[row_index_top];
        }
    }

    let ouput_buffer: Vec<u8> = 
        horizontally_flipped_image.iter()
            .flat_map(|vector| [vector.x as u8, vector.y as u8, vector.z as u8])
            .collect();



    let save_result = image::save_buffer_with_format(
        output_path, 
        &ouput_buffer, 
        image_width.try_into().unwrap(), 
        image_height.try_into().unwrap(), 
        image::ColorType::Rgb8, 
        image::ImageFormat::Png
    );

    if save_result.is_ok() {
        println!("Saved output image to {}", output_path);
    } else {
        let error = save_result.unwrap_err();
        panic!("{}", error.to_string());
    }

}
