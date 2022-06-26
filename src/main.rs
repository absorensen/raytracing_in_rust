extern crate minifb;
use minifb::{Key, ScaleMode, Window, WindowOptions, clamp};
// Look into performance optimization of the RNG
use rand::rngs::ThreadRng;
use rand::{Rng};
use texture::{SolidColor, NoiseTexture};
use std::f64;
use std::sync::Arc;
use std::time::{Instant};
use rayon::prelude::*;

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

use texture::{Texture, CheckerTexture, ImageTexture};
use bvh_node::{BVHNode};
use vector3::{Vector3, Point3, Color};
use ray::Ray;
use sphere::Sphere;
use hittable::{Hittable, HittableList, XYRect, XZRect, YZRect, Box, RotateY, Translate, ConstantMedium};
use moving_sphere::MovingSphere;
use camera::Camera;
use material::{Lambertian, Metal, Dielectric, Material, DiffuseLight, Isotropic};


fn random_spheres_scene(rng: &mut ThreadRng, aspect_ratio: f64, number_of_balls: i32) -> (HittableList, Camera, Color) {
    let mut world = HittableList::default();

    let ground_texture: Arc<dyn Texture> = Arc::new(CheckerTexture::from_colors(&Color{x:0.2, y:0.3, z:0.1}, &Color{x:0.9, y:0.9, z:0.9}));
    let ground_material: Arc<dyn Material> = Arc::new(Lambertian{albedo: ground_texture});
    world.push(Sphere::new(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, &ground_material));
    for a in -number_of_balls..number_of_balls {
        for b in -number_of_balls..number_of_balls {
            let choose_mat = rand::random::<f64>();
            let center = Point3{x: a as f64 + 0.9 * rand::random::<f64>(), y: 0.2, z: b as f64 + 0.9 * rand::random::<f64>()};

            if (center - Point3{x: 4.0, y: 0.2, z: 0.0}).length() > 0.9 {
                let chosen_material : Arc<dyn Material>;
                if choose_mat < 0.8 {
                    let chosen_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&(Color::random(rng) * Color::random(rng))));
                    chosen_material = Arc::new(Lambertian{albedo: chosen_texture});
                } else if choose_mat < 0.95 {
                    chosen_material = Arc::new(Metal{albedo: Color::random(rng), fuzz: rng.gen::<f64>()});
                } else {
                    let index_of_refraction = 1.5;
                    chosen_material = Arc::new(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction});
                }

                world.push(Sphere::new(center, 0.2, &chosen_material));
            }
        }
    }

    let index_of_refraction = 1.5;
    let glass_material: Arc<dyn Material> = Arc::new(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction});
    world.push(Sphere::new(Point3{x: 0.0, y: 1.0, z: 0.0}, 1.0, &glass_material));

    let lambertian_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.4, y: 0.2, z: 0.1}));
    let lambertian_material: Arc<dyn Material> = Arc::new(Lambertian{albedo: lambertian_texture});
    world.push(Sphere::new(Point3{x: -4.0, y: 1.0, z: 0.0}, 1.0, &lambertian_material));

    let metal_material: Arc<dyn Material> = Arc::new(Metal{albedo: Color{x: 0.7, y: 0.6, z: 0.5}, fuzz: 0.0});
    world.push(Sphere::new(Point3{x: 4.0, y: 1.0, z: 0.0}, 1.0, &metal_material));

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



    (world, camera, background)
}

fn random_moving_spheres_scene(rng: &mut ThreadRng, aspect_ratio: f64, number_of_balls: i32) -> (HittableList, Camera, Color) {
    let mut world = HittableList::default();

    let ground_texture: Arc<dyn Texture> = Arc::new(CheckerTexture::from_colors(&Color{x:0.2, y:0.3, z:0.1}, &Color{x:0.9, y:0.9, z:0.9}));
    let ground_material: Arc<dyn Material> = Arc::new(Lambertian{albedo: ground_texture});
    world.push(Sphere::new(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, &ground_material));
    for a in -number_of_balls..number_of_balls {
        for b in -number_of_balls..number_of_balls {
            let choose_mat = rand::random::<f64>();
            let center = Point3{x: a as f64 + 0.9 * rand::random::<f64>(), y: 0.2, z: b as f64 + 0.9 * rand::random::<f64>()};

            if (center - Point3{x: 4.0, y: 0.2, z: 0.0}).length() > 0.9 {
                if choose_mat < 0.8 {
                    let mut movement = Vector3::zero();
                    movement.y = rand::random::<f64>() * 0.5;

                    let chosen_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&(Color::random(rng) * Color::random(rng))));
                    let chosen_material: Arc<dyn Material> = Arc::new(Lambertian{albedo: chosen_texture});
                    world.push(MovingSphere::new(0.2, center, center + movement,  &chosen_material, 0.0, 1.0));
                } else if choose_mat < 0.95 {

                    let chosen_material: Arc<dyn Material> = Arc::new(Metal{albedo: Color::random(rng), fuzz: rand::random::<f64>()});
                    world.push(Sphere::new(center, 0.2, &chosen_material));
                } else {

                    let index_of_refraction = 1.5;
                    let chosen_material: Arc<dyn Material> = Arc::new(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction});
                    world.push(Sphere::new(center, 0.2, &chosen_material));
                }
            }
        }
    }

    let index_of_refraction = 1.5;
    let glass_material: Arc<dyn Material> = Arc::new(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction});
    world.push(Sphere::new(Point3{x: 0.0, y: 1.0, z: 0.0}, 1.0, &glass_material));

    let lambertian_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.4, y: 0.2, z: 0.1}));
    let lambertian_material: Arc<dyn Material> = Arc::new(Lambertian{albedo: lambertian_texture});
    world.push(Sphere::new(Point3{x: -4.0, y: 1.0, z: 0.0}, 1.0, &lambertian_material));

    let metal_material: Arc<dyn Material> = Arc::new(Metal{albedo: Color{x: 0.7, y: 0.6, z: 0.5}, fuzz: 0.0});
    world.push(Sphere::new(Point3{x: 4.0, y: 1.0, z: 0.0}, 1.0, &metal_material));

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

    (world, camera, background)
}

fn two_spheres_scene(aspect_ratio: f64) -> (HittableList, Camera, Color) {
    let mut world = HittableList::default();

    let checker_texture: Arc<dyn Texture> = Arc::new(CheckerTexture::from_colors(&Color{x:0.2, y:0.3, z:0.1}, &Color{x:0.9, y:0.9, z:0.9}));
    let checker_material: Arc<dyn Material> = Arc::new(Lambertian{albedo: checker_texture});
    world.push(Sphere::new(Point3{x: 0.0, y: -10.0, z: 0.0}, 10.0, &checker_material));
    world.push(Sphere::new(Point3{x: 0.0, y: 10.0, z: 0.0}, 10.0, &checker_material));

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


    (world, camera, background)
}

fn two_perlin_spheres_scene(rng: &mut ThreadRng, aspect_ratio: f64, element_count: u32) -> (HittableList, Camera, Color) {
    let mut world = HittableList::default();

    let perlin_texture: Arc<dyn Texture> = Arc::new(NoiseTexture::new(rng, element_count, 4.0));
    let perlin_material: Arc<dyn Material> = Arc::new(Lambertian{albedo: perlin_texture});
    world.push(Sphere::new(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, &perlin_material));
    world.push(Sphere::new(Point3{x: 0.0, y: 2.0, z: 0.0}, 2.0, &perlin_material));

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


    (world, camera, background)
}

fn earth_scene(aspect_ratio: f64) -> (HittableList, Camera, Color) {
    let mut world = HittableList::default();
    let texture: Arc<dyn Texture> = Arc::new(ImageTexture::new("earthmap.png"));
    let material: Arc<dyn Material> = Arc::new(Lambertian{ albedo: texture });
    world.push(Sphere::new(Vector3::new(0.0, 0.0, 0.0), 2.0, &material));

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

    (world, camera, background)
}

fn simple_light_scene(rng: &mut ThreadRng, aspect_ratio: f64, element_count: u32) -> (HittableList, Camera, Color) {
    let mut world = HittableList::default();

    let perlin_texture: Arc<dyn Texture> = Arc::new(NoiseTexture::new(rng, element_count, 4.0));
    let perlin_material: Arc<dyn Material> = Arc::new(Lambertian{albedo: perlin_texture});
    world.push(Sphere::new(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, &perlin_material));
    world.push(Sphere::new(Point3{x: 0.0, y: 2.0, z: 0.0}, 2.0, &perlin_material));

    let diffuse_light_material: Arc<dyn Material> = Arc::new(DiffuseLight::from_color( &Color{x: 4.0, y: 4.0, z: 4.0 } ));
    world.push(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, &diffuse_light_material));
    world.push(Sphere::new(Point3{x: 0.0, y: 7.0, z: 0.0}, 2.0, &diffuse_light_material));

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


    (world, camera, background)
}

fn empty_cornell_box_scene(aspect_ratio: f64) -> (HittableList, Camera, Color) {
    let mut world = HittableList::default();

    let red_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.65, y: 0.05, z: 0.05}));
    let red_material: Arc<dyn Material> = Arc::new(Lambertian{ albedo: red_texture });

    let white_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
    let white_material: Arc<dyn Material> = Arc::new(Lambertian{ albedo: white_texture });

    let green_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.12, y: 0.45, z: 0.15}));
    let green_material: Arc<dyn Material> = Arc::new(Lambertian{ albedo: green_texture });

    let diffuse_light_material: Arc<dyn Material> = Arc::new(DiffuseLight::from_color( &Color{x: 15.0, y: 15.0, z: 15.0 } ));

    world.push(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &green_material));
    world.push(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, &red_material));
    world.push(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, &diffuse_light_material));
    world.push(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, &white_material));
    world.push(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &white_material));
    world.push(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &white_material));

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

    (world, camera, background)
}

fn cornell_box_two_diffuse_boxes_scene(aspect_ratio: f64) -> (HittableList, Camera, Color) {
    let (mut world, camera, background) = empty_cornell_box_scene(aspect_ratio);
    
    let white_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
    let white_material: Arc<dyn Material> = Arc::new(Lambertian{ albedo: white_texture });

    let box_1 = Box::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 165.0, y: 330.0, z: 165.0}, &white_material);
    let box_1_arc : Arc<dyn Hittable> = Arc::new(box_1);
    let box_1_rotation: Arc<dyn Hittable> = Arc::new(RotateY::new(15.0, &box_1_arc));
    let box_1_translated = Translate::new(Vector3 { x: 265.0, y: 0.0, z: 295.0 }, &box_1_rotation);
    world.push(box_1_translated);

    let box_2 = Box::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 165.0, y: 165.0, z: 165.0}, &white_material);
    let box_2_arc : Arc<dyn Hittable> = Arc::new(box_2);
    let box_2_rotation: Arc<dyn Hittable> = Arc::new(RotateY::new(-18.0, &box_2_arc));
    let box_2_translated = Translate::new(Vector3 { x: 130.0, y: 0.0, z: 65.0 }, &box_2_rotation);
    world.push(box_2_translated);

    (world, camera, background)
}

fn cornell_box_two_smoke_boxes_scene(aspect_ratio: f64) -> (HittableList, Camera, Color) {
    let mut world = HittableList::default();

    let red_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.65, y: 0.05, z: 0.05}));
    let red_material: Arc<dyn Material> = Arc::new(Lambertian{ albedo: red_texture });

    let white_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
    let white_material: Arc<dyn Material> = Arc::new(Lambertian{ albedo: white_texture });

    let green_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.12, y: 0.45, z: 0.15}));
    let green_material: Arc<dyn Material> = Arc::new(Lambertian{ albedo: green_texture });

    let diffuse_light_material: Arc<dyn Material> = Arc::new(DiffuseLight::from_color( &Color{x: 7.0, y: 7.0, z: 7.0 } ));

    let dark_phase_function: Arc<dyn Material> = Arc::new(Isotropic::from_color( &Color{x: 0.0, y: 0.0, z: 0.0} ));
    let light_phase_function: Arc<dyn Material> = Arc::new(Isotropic::from_color( &Color{x: 1.0, y: 1.0, z: 1.0} ));

    world.push(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &green_material));
    world.push(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, &red_material));
    world.push(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, &diffuse_light_material));
    world.push(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, &white_material));
    world.push(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &white_material));
    world.push(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &white_material));

    let box_1 = Box::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 165.0, y: 330.0, z: 165.0}, &white_material);
    let box_1_arc : Arc<dyn Hittable> = Arc::new(box_1);
    let box_1_rotation: Arc<dyn Hittable> = Arc::new(RotateY::new(15.0, &box_1_arc));
    let box_1_translated: Arc<dyn Hittable> = Arc::new(Translate::new(Vector3 { x: 265.0, y: 0.0, z: 295.0 }, &box_1_rotation));
    let box_1_smoke = ConstantMedium::new(&box_1_translated, &dark_phase_function, 0.01);
    world.push(box_1_smoke);

    let box_2 = Box::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 165.0, y: 165.0, z: 165.0}, &white_material);
    let box_2_arc : Arc<dyn Hittable> = Arc::new(box_2);
    let box_2_rotation: Arc<dyn Hittable> = Arc::new(RotateY::new(-18.0, &box_2_arc));
    let box_2_translated: Arc<dyn Hittable> = Arc::new(Translate::new(Vector3 { x: 130.0, y: 0.0, z: 65.0 }, &box_2_rotation));
    let box_2_smoke = ConstantMedium::new(&box_2_translated, &light_phase_function, 0.01);
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



    (world, camera, background)
}

fn final_scene_book_2(rng: &mut ThreadRng, aspect_ratio: f64, perlin_element_count: u32, cube_sphere_count: u32) -> (HittableList, Camera, Color) {
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
    
    
    let mut floor_cubes = HittableList::default();

    let ground_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x:0.48, y:0.83, z:0.53}));
    let ground_material: Arc<dyn Material> = Arc::new(Lambertian{albedo: ground_texture});

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
            let y1: f64 = rng.gen_range(1.0, 101.0);
            let z1: f64 = z0 + w;

            floor_cubes.push(
                Box::new(
                    Vector3{x: x0, y: y0, z: z0}, 
                    Vector3{x: x1, y: y1, z: z1}, 
                    &ground_material)
                );
        }
    }
    let mut objects = HittableList::default();
    let floor_cubes_bvh = BVHNode::from_hittable_list(&mut floor_cubes, camera.get_start_time(), camera.get_end_time());
    objects.push(floor_cubes_bvh);

    let diffuse_light_material: Arc<dyn Material> = Arc::new(DiffuseLight::from_color( &Color{x: 7.0, y: 7.0, z: 7.0 } ));
    objects.push(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, &diffuse_light_material));

    let center_0 = Vector3{x: 400.0, y: 400.0, z: 200.0};
    let center_1 = center_0 + Vector3{x: 30.0, y: 0.0, z: 0.0};
    let moving_sphere_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.7, y: 0.3, z: 0.1}));
    let moving_sphere_material: Arc<dyn Material> = Arc::new(Lambertian{ albedo: moving_sphere_texture });
    objects.push(MovingSphere{ radius: 50.0, center_0, center_1, material: moving_sphere_material, time_0: 0.0, time_1: 1.0 });

    let index_of_refraction = 1.5;
    let glass_material: Arc<dyn Material> = Arc::new(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction});
    objects.push(Sphere::new(Point3{x: 260.0, y: 150.0, z: 45.0}, 50.0, &glass_material));

    let metal_material: Arc<dyn Material> = Arc::new(Metal{albedo: Color{x: 0.8, y: 0.8, z: 0.9}, fuzz: 1.0});
    objects.push(Sphere::new(Point3{x: 0.0, y: 150.0, z: 145.0}, 50.0, &metal_material));

    // Volume sphere
    let index_of_refraction = 1.5;
    let volume_sphere_boundary_material: Arc<dyn Material> = Arc::new(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction});
    let boundary = Sphere::new(Point3{x: 360.0, y: 150.0, z: 145.0}, 70.0, &volume_sphere_boundary_material);
    let boundary_arc: Arc<dyn Hittable> = Arc::new(boundary);
    objects.push_arc(&boundary_arc);

    let light_phase_function: Arc<dyn Material> = Arc::new(Isotropic::from_color( &Color{x: 0.2, y: 0.4, z: 0.9} ));
    let volume_sphere= ConstantMedium::new(&boundary_arc, &light_phase_function, 0.2);
    objects.push(volume_sphere);

    let global_volume_material: Arc<dyn Material> = Arc::new(Dielectric{index_of_refraction, inverse_index_of_refraction: 1.0 / index_of_refraction});
    let global_volume_sphere: Arc<dyn Hittable> = Arc::new(Sphere::new(Point3{x: 0.0, y: 0.0, z: 0.0}, 5000.0, &global_volume_material));
    let global_volume = ConstantMedium::new(&global_volume_sphere, &light_phase_function, 0.0001);
    objects.push(global_volume);

    let earth_texture: Arc<dyn Texture> = Arc::new(ImageTexture::new("earthmap.png"));
    let earth_material: Arc<dyn Material> = Arc::new(Lambertian{ albedo: earth_texture });
    objects.push(Sphere::new(Vector3::new(400.0, 200.0, 400.0), 100.0, &earth_material));

    let perlin_texture: Arc<dyn Texture> = Arc::new(NoiseTexture::new(rng, perlin_element_count, 0.1));
    let perlin_material: Arc<dyn Material> = Arc::new(Lambertian{albedo: perlin_texture});
    objects.push(Sphere::new(Point3{x: 220.0, y: 280.0, z: 300.0}, 80.0, &perlin_material));


    let mut cube_spheres = HittableList::default();
    let white_texture: Arc<dyn Texture> = Arc::new(SolidColor::from_color(&Color{x: 0.73, y: 0.73, z: 0.73}));
    let white_material: Arc<dyn Material> = Arc::new(Lambertian{ albedo: white_texture });

    for _j in 0..cube_sphere_count {
        cube_spheres.push(Sphere::new(Vector3::random_range(rng, 0.0, 165.0), 10.0, &white_material));
    }
    let cube_spheres_bvh = BVHNode::from_hittable_list(&mut cube_spheres, camera.get_start_time(), camera.get_end_time());
    let cube_spheres_arc: Arc <dyn Hittable> = Arc::new(cube_spheres_bvh);
    let cube_spheres_rotation: Arc<dyn Hittable> = Arc::new(RotateY::new(15.0, &cube_spheres_arc));
    let cube_spheres_translated: Arc<dyn Hittable> = Arc::new(Translate::new(Vector3 { x: -100.0, y: 270.0, z: 395.0 }, &cube_spheres_rotation));
    objects.push_arc(&cube_spheres_translated);

    let background = Color{x:0.0, y:0.0, z: 0.0};


    (objects, camera, background)
}

fn ray_color(rng: &mut ThreadRng, background: &Color, ray: &Ray, world: & dyn Hittable, depth: i64) -> Color {
    if depth <= 0 {
        return Color{x: 0.0, y: 0.0, z: 0.0};
    }

    if let Some(hit) = world.hit(rng, ray, 0.001, f64::MAX) {
        let mut attenuation: Color = Color::zero();
        let mut scattered: Ray = Ray::new(Vector3::zero(), Vector3::zero(), ray.time);
        let emitted: Color = hit.material.emitted(hit.u, hit.v, &hit.position);

        if hit.material.scatter(rng, ray, &hit, &mut attenuation, &mut scattered) {
            return emitted + attenuation * ray_color(rng, background, &scattered, world, depth - 1);
        } else {
            return emitted;
        }
    } 

    *background
}

fn render_pixel(rng: &mut ThreadRng, background: &Color, pixel_index: i64, image_width: i64, image_height: i64, samples_per_pixel: i64, camera: &Camera, world: &dyn Hittable, max_depth: i64, scale: f64, use_parallel: bool) -> Vector3{
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
            ray_color(&mut rng, background, &ray, world, max_depth)
        }).sum();
    } else {
        for _sample_index in 0..samples_per_pixel {
            let u = (column_index as f64 + rng.gen::<f64>() ) / ((image_width - 1) as f64);
            let v = (row_index as f64 + rng.gen::<f64>() ) / ((image_height - 1) as f64);
            let ray = camera.get_ray(rng, u, v);
            color_buffer += ray_color(rng, background, &ray, world, max_depth);
        }
    }

    color_buffer.x = 255.999 * clamp(0.0, (scale * color_buffer.x).sqrt(), 0.999);
    color_buffer.y = 255.999 * clamp(0.0, (scale * color_buffer.y).sqrt(), 0.999);
    color_buffer.z = 255.999 * clamp(0.0, (scale * color_buffer.z).sqrt(), 0.999);

    color_buffer
}

fn main() {
    // Display Image
    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_width: i64 = 600;
    let mut image_height = ((image_width as f64) / aspect_ratio) as i64;
    let output_path = "output.png";

    // Render Settings
    let samples_per_pixel = 100;
    let max_depth = 150;

    // Compute Settings
    let run_parallel = true;
    let run_samples_parallel = true;



    // Scene
    let mut rng = rand::thread_rng();
    let random_balls_count = 6;
    let noise_points_count = 256;
    let cube_sphere_count = 1000;
    let scene_index = 6;
    let (mut world, camera, background) = match scene_index {
        0 => random_spheres_scene(&mut rng, aspect_ratio, random_balls_count),
        1 => random_moving_spheres_scene(&mut rng, aspect_ratio, random_balls_count),
        2 => two_spheres_scene(aspect_ratio),
        3 => two_perlin_spheres_scene(&mut rng, aspect_ratio, noise_points_count),
        4 => earth_scene(aspect_ratio),
        5 => simple_light_scene(&mut rng, aspect_ratio, noise_points_count),
        6 => {
            aspect_ratio = 1.0;
            image_height = ((image_width as f64) / aspect_ratio) as i64;
            empty_cornell_box_scene(aspect_ratio)
        },
        7 => {
            aspect_ratio = 1.0;
            image_height = ((image_width as f64) / aspect_ratio) as i64;
            cornell_box_two_diffuse_boxes_scene(aspect_ratio)
        },
        8 => {
            aspect_ratio = 1.0;
            image_height = ((image_width as f64) / aspect_ratio) as i64;
            cornell_box_two_smoke_boxes_scene(aspect_ratio)
        },
        9 => {
            aspect_ratio = 1.0;
            image_height = ((image_width as f64) / aspect_ratio) as i64;
            final_scene_book_2(&mut rng, aspect_ratio, noise_points_count, cube_sphere_count)
        },
        _ => panic!("Incorrect scene chosen!"),
    };
    let world = BVHNode::from_hittable_list(&mut world, camera.get_start_time(), camera.get_end_time());






    let scale = 1.0 / (samples_per_pixel as f64);

    let now = Instant::now();
    let total_pixels = image_height * image_width;
    let image: Vec<Vector3> = 
    if run_parallel {
        (0..total_pixels).into_par_iter().map(|pixel_index:i64| {
            let mut rng = rand::thread_rng();
            render_pixel(&mut rng, &background, pixel_index, image_width, image_height, samples_per_pixel, &camera, &world, max_depth, scale, run_samples_parallel)
        }).collect()
    } else {
        (0..total_pixels).into_iter().map(|pixel_index:i64| {
            render_pixel(&mut rng, &background, pixel_index, image_width, image_height, samples_per_pixel, &camera, &world, max_depth, scale, run_samples_parallel)
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
