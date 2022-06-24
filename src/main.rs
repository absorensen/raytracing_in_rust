extern crate minifb;
use minifb::{Key, ScaleMode, Window, WindowOptions, clamp};
use rand::{Rng};
use std::f64;
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

use vector3::{Vector3, Point3, Color};
use ray::Ray;
use sphere::Sphere;
use hittable::{Hittable, HittableList};
use moving_sphere::MovingSphere;
use camera::Camera;
use material::{Lambertian, Metal, Dielectric};

fn ray_color(ray: &Ray, world: & dyn Hittable, depth: i64) -> Color {
    if depth <= 0 {
        return Color{x: 0.0, y: 0.0, z: 0.0};
    }

    let record_option = world.hit(ray, 0.0000001, f64::INFINITY);
    if record_option.is_some() {
        let record = record_option.unwrap();
        
        let mut attenuation: Color = Color::zero();
        let mut scattered: Ray = Ray::new(Vector3::zero(), Vector3::zero(), ray.time);
        if record.material.scatter(ray, &record, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } 
        return Color{x: 0.0, y: 0.0, z: 0.0};
    }

    let unit_direction: Vector3 = ray.direction.normalized();
    let t : f64 = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vector3{x: 1.0, y: 1.0, z: 1.0} + t * Vector3{x:0.5, y:0.7, z: 1.0}
}

fn random_spheres_scene(number_of_balls: i32) -> HittableList {
    let mut world = HittableList::default();

    world.push(Sphere::new(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, Lambertian{albedo: Color{x: 0.5, y: 0.5, z: 0.5}}));
    for a in -number_of_balls..number_of_balls {
        for b in -number_of_balls..number_of_balls {
            let choose_mat = rand::random::<f64>();
            let center = Point3{x: a as f64 + 0.9 * rand::random::<f64>(), y: 0.2, z: b as f64 + 0.9 * rand::random::<f64>()};

            if (center - Point3{x: 4.0, y: 0.2, z: 0.0}).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.push(Sphere::new(center, 0.2, Lambertian{albedo: Color::random() * Color::random()}));
                } else if choose_mat < 0.95 {
                    world.push(Sphere::new(center, 0.2, Metal{albedo: Color::random(), fuzz: rand::random::<f64>()}));
                } else {
                    world.push(Sphere::new(center, 0.2, Dielectric{ref_idx: 1.5}));
                }
            }
        }
    }

    world.push(Sphere::new(Point3{x: 0.0, y: 1.0, z: 0.0}, 1.0, Dielectric{ref_idx: 1.5}));
    world.push(Sphere::new(Point3{x: -4.0, y: 1.0, z: 0.0}, 1.0, Lambertian{albedo: Color{x: 0.4, y: 0.2, z: 0.1}}));
    world.push(Sphere::new(Point3{x: 4.0, y: 1.0, z: 0.0}, 1.0, Metal{albedo: Color{x: 0.7, y: 0.6, z: 0.5}, fuzz: 0.0}));

    world
}

fn random_moving_spheres_scene() -> HittableList {
    let mut world = HittableList::default();

    world.push(Sphere::new(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, Lambertian{albedo: Color{x: 0.5, y: 0.5, z: 0.5}}));
    let number_of_balls = 3;
    for a in -number_of_balls..number_of_balls {
        for b in -number_of_balls..number_of_balls {
            let choose_mat = rand::random::<f64>();
            let center = Point3{x: a as f64 + 0.9 * rand::random::<f64>(), y: 0.2, z: b as f64 + 0.9 * rand::random::<f64>()};

            if (center - Point3{x: 4.0, y: 0.2, z: 0.0}).length() > 0.9 {
                if choose_mat < 0.8 {
                    let mut movement = Vector3::zero();
                    movement.y = rand::random::<f64>() * 0.5;
                    world.push(MovingSphere::new(0.2, center, center + movement,  Lambertian{albedo: Color::random() * Color::random()}, 0.0, 1.0));
                } else if choose_mat < 0.95 {
                    world.push(Sphere::new(center, 0.2, Metal{albedo: Color::random(), fuzz: rand::random::<f64>()}));
                } else {
                    world.push(Sphere::new(center, 0.2, Dielectric{ref_idx: 1.5}));
                }
            }
        }
    }

    world.push(Sphere::new(Point3{x: 0.0, y: 1.0, z: 0.0}, 1.0, Dielectric{ref_idx: 1.5}));
    world.push(Sphere::new(Point3{x: -4.0, y: 1.0, z: 0.0}, 1.0, Lambertian{albedo: Color{x: 0.4, y: 0.2, z: 0.1}}));
    world.push(Sphere::new(Point3{x: 4.0, y: 1.0, z: 0.0}, 1.0, Metal{albedo: Color{x: 0.7, y: 0.6, z: 0.5}, fuzz: 0.0}));

    world
}

fn render_pixel(pixel_index: i64, image_width: i64, image_height: i64, samples_per_pixel: i64, camera: &Camera, world: &dyn Hittable, max_depth: i64, scale: f64, use_parallel: bool) -> Vector3{
    let column_index = pixel_index % image_width;
    let row_index = pixel_index / image_width;

    let mut color_buffer = Color{x: 0.0, y: 0.0, z: 0.0};
    if use_parallel {
        let mut rng = rand::thread_rng();
        let seeds: Vec<(f64, f64)> = (0..samples_per_pixel).into_iter().map(|_| (rng.gen::<f64>(), rng.gen::<f64>()) ).collect();
        color_buffer = seeds.into_par_iter().map(|(seed0, seed1)| {
            let u = (column_index as f64 + seed0 ) / ((image_width - 1) as f64);
            let v = (row_index as f64 + seed1 ) / ((image_height - 1) as f64);
            let ray = camera.get_ray(u, v);
            ray_color(&ray, world, max_depth)
        }).sum();
    } else {
        let mut rng = rand::thread_rng();

        for _sample_index in 0..samples_per_pixel {
            let u = (column_index as f64 + rng.gen::<f64>() ) / ((image_width - 1) as f64);
            let v = (row_index as f64 + rng.gen::<f64>() ) / ((image_height - 1) as f64);
            let ray = camera.get_ray(u, v);
            color_buffer += ray_color(&ray, world, max_depth);
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
    let image_width: i64 = 200;
    let image_height = ((image_width as f64) / aspect_ratio) as i64;
    let samples_per_pixel = 800;
    let max_depth = 30;
    let run_parallel = false;
    let run_samples_parallel = true;
    let random_balls_count = 1;

    let world = random_spheres_scene(random_balls_count);

    // Camera
    let look_from = Point3{x: 13.0, y: 2.0, z: 3.0 };
    let look_at = Point3{x: 0.0, y: 0.0, z: 0.0};
    let v_up = Vector3{x: 0.0, y:1.0, z:0.0};
    let dist_to_focus = 15.0;
    let aperture = 0.1;
    let time0: f64 = 0.0;
    let time1: f64 = 1.0;

    let camera = Camera::new(look_from, look_at, v_up,20.0, aspect_ratio, aperture, dist_to_focus, time0, time1);

    let scale = 1.0 / (samples_per_pixel as f64);

    let now = Instant::now();
    let total_pixels = image_height * image_width;
    let image: Vec<Vector3> = 
    if run_parallel {
        (0..total_pixels).into_par_iter().map(|pixel_index:i64| {
            render_pixel(pixel_index, image_width, image_height, samples_per_pixel, &camera, &world, max_depth, scale, run_samples_parallel)
        }).collect()
    } else {
        (0..total_pixels).into_iter().map(|pixel_index:i64| {
            render_pixel(pixel_index, image_width, image_height, samples_per_pixel, &camera, &world, max_depth, scale, run_samples_parallel)
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
