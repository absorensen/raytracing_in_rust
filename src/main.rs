extern crate minifb;
use minifb::{Key, ScaleMode, Window, WindowOptions, clamp};
use rand::rngs::ThreadRng;
use rand::{Rng};
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

use bvh_node::{BVHNode};
use vector3::{Vector3, Point3, Color};
use ray::Ray;
use sphere::Sphere;
use hittable::{Hittable, HittableList};
use moving_sphere::MovingSphere;
use camera::Camera;
use material::{Lambertian, Metal, Dielectric, Material};

fn random_spheres_scene(rng: &mut ThreadRng, number_of_balls: i32) -> HittableList {
    let mut world = HittableList::default();

    let ground_material: Arc<dyn Material> = Arc::new(Lambertian{albedo: Color{x: 0.5, y: 0.5, z: 0.5}});
    world.push(Sphere::new(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, &ground_material));
    for a in -number_of_balls..number_of_balls {
        for b in -number_of_balls..number_of_balls {
            let choose_mat = rand::random::<f64>();
            let center = Point3{x: a as f64 + 0.9 * rand::random::<f64>(), y: 0.2, z: b as f64 + 0.9 * rand::random::<f64>()};

            if (center - Point3{x: 4.0, y: 0.2, z: 0.0}).length() > 0.9 {
                let chosen_material : Arc<dyn Material>;
                if choose_mat < 0.8 {
                    chosen_material = Arc::new(Lambertian{albedo: Color::random(rng) * Color::random(rng)});
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

    let lambertian_material: Arc<dyn Material> = Arc::new(Lambertian{albedo: Color{x: 0.4, y: 0.2, z: 0.1}});
    world.push(Sphere::new(Point3{x: -4.0, y: 1.0, z: 0.0}, 1.0, &lambertian_material));

    let metal_material: Arc<dyn Material> = Arc::new(Metal{albedo: Color{x: 0.7, y: 0.6, z: 0.5}, fuzz: 0.0});
    world.push(Sphere::new(Point3{x: 4.0, y: 1.0, z: 0.0}, 1.0, &metal_material));

    world
}

fn random_moving_spheres_scene(rng: &mut ThreadRng, number_of_balls: i32) -> HittableList {
    let mut world = HittableList::default();

    let ground_material: Arc<dyn Material> = Arc::new(Lambertian{albedo: Color{x: 0.5, y: 0.5, z: 0.5}});
    world.push(Sphere::new(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, &ground_material));
    for a in -number_of_balls..number_of_balls {
        for b in -number_of_balls..number_of_balls {
            let choose_mat = rand::random::<f64>();
            let center = Point3{x: a as f64 + 0.9 * rand::random::<f64>(), y: 0.2, z: b as f64 + 0.9 * rand::random::<f64>()};

            if (center - Point3{x: 4.0, y: 0.2, z: 0.0}).length() > 0.9 {
                if choose_mat < 0.8 {
                    let mut movement = Vector3::zero();
                    movement.y = rand::random::<f64>() * 0.5;

                    let chosen_material: Arc<dyn Material> = Arc::new(Lambertian{albedo: Color::random(rng) * Color::random(rng)});
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

    let lambertian_material: Arc<dyn Material> = Arc::new(Lambertian{albedo: Color{x: 0.4, y: 0.2, z: 0.1}});
    world.push(Sphere::new(Point3{x: -4.0, y: 1.0, z: 0.0}, 1.0, &lambertian_material));

    let metal_material: Arc<dyn Material> = Arc::new(Metal{albedo: Color{x: 0.7, y: 0.6, z: 0.5}, fuzz: 0.0});
    world.push(Sphere::new(Point3{x: 4.0, y: 1.0, z: 0.0}, 1.0, &metal_material));

    world
}

fn ray_color(rng: &mut ThreadRng, background: &Color, ray: &Ray, world: & dyn Hittable, depth: i64) -> Color {
    if depth <= 0 {
        return Color{x: 0.0, y: 0.0, z: 0.0};
    }

    if let Some(hit) = world.hit(ray, 0.001, f64::MAX) {
        let mut attenuation: Color = Color::zero();
        let mut scattered: Ray = Ray::new(Vector3::zero(), Vector3::zero(), ray.time);
        if hit.material.scatter(rng, ray, &hit, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(rng, background, &scattered, world, depth - 1);
        } 
        return Color{x: 0.0, y: 0.0, z: 0.0};
    } 

    let unit_direction = ray.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    ((1.0 - t) * Color::new(1.0, 1.0, 1.0)) + (t * Color::new(0.5, 0.7, 1.0))
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
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i64 = 800;
    let image_height = ((image_width as f64) / aspect_ratio) as i64;

    // Render Settings
    let samples_per_pixel = 10;
    let max_depth = 50;
    
    // Compute Settings
    let run_parallel = true;
    let run_samples_parallel = true;

    // Camera
    let look_from = Point3{x: 13.0, y: 2.0, z: 3.0 };
    let look_at = Point3{x: 0.0, y: 0.0, z: 0.0};
    let v_up = Vector3{x: 0.0, y:1.0, z:0.0};
    let dist_to_focus = 15.0;
    let aperture = 0.1;
    let time_0: f64 = 0.0;
    let time_1: f64 = 1.0;
    let camera = Camera::new(look_from, look_at, v_up,20.0, aspect_ratio, aperture, dist_to_focus, time_0, time_1);

    // Scene
    let mut rng = rand::thread_rng();
    let random_balls_count = 3;
    let mut world = random_moving_spheres_scene(&mut rng, random_balls_count);
    let world = BVHNode::from_hittable_list(&mut world, time_0, time_1);
    let background = Color{x:0.5, y:0.7, z: 1.0};




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
}
