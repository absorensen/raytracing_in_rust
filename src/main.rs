extern crate minifb;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use rand::{Rng, random};
use std::f64;
use std::time::{Instant};
use std::sync::Arc;

mod vector3;
mod ray;
mod sphere;
mod moving_sphere;
mod hittable;
mod camera;
mod material;

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

fn random_spheres_scene() -> HittableList {
    let mut world = HittableList::default();

    world.push(Sphere::new(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, Arc::<Lambertian>::new(Lambertian{albedo: Color{x: 0.5, y: 0.5, z: 0.5}})));
    let number_of_balls = 1;
    for a in -number_of_balls..number_of_balls {
        for b in -number_of_balls..number_of_balls {
            let choose_mat = rand::random::<f64>();
            let center = Point3{x: a as f64 + 0.9 * rand::random::<f64>(), y: 0.2, z: b as f64 + 0.9 * rand::random::<f64>()};

            if (center - Point3{x: 4.0, y: 0.2, z: 0.0}).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.push(Sphere::new(center, 0.2, Arc::<Lambertian>::new(Lambertian{albedo: Color::random() * Color::random()})));
                } else if choose_mat < 0.95 {
                    world.push(Sphere::new(center, 0.2, Arc::<Metal>::new(Metal{albedo: Color::random(), fuzz: rand::random::<f64>()})));
                } else {
                    world.push(Sphere::new(center, 0.2, Arc::<Dielectric>::new(Dielectric{ref_idx: 1.5})));
                }
            }
        }
    }

    world.push(Sphere::new(Point3{x: 0.0, y: 1.0, z: 0.0}, 1.0, Arc::<Dielectric>::new(Dielectric{ref_idx: 1.5})));
    world.push(Sphere::new(Point3{x: -4.0, y: 1.0, z: 0.0}, 1.0, Arc::<Lambertian>::new(Lambertian{albedo: Color{x: 0.4, y: 0.2, z: 0.1}})));
    world.push(Sphere::new(Point3{x: 4.0, y: 1.0, z: 0.0}, 1.0, Arc::<Metal>::new(Metal{albedo: Color{x: 0.7, y: 0.6, z: 0.5}, fuzz: 0.0})));

    world
}

fn random_moving_spheres_scene() -> HittableList {
    let mut world = HittableList::default();

    world.push(Sphere::new(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, Arc::<Lambertian>::new(Lambertian{albedo: Color{x: 0.5, y: 0.5, z: 0.5}})));
    let number_of_balls = 3;
    for a in -number_of_balls..number_of_balls {
        for b in -number_of_balls..number_of_balls {
            let choose_mat = rand::random::<f64>();
            let center = Point3{x: a as f64 + 0.9 * rand::random::<f64>(), y: 0.2, z: b as f64 + 0.9 * rand::random::<f64>()};

            if (center - Point3{x: 4.0, y: 0.2, z: 0.0}).length() > 0.9 {
                if choose_mat < 0.8 {
                    let mut movement = Vector3::zero();
                    movement.y = rand::random::<f64>() * 0.5;
                    world.push(MovingSphere::new(0.2, center, center + movement,  Arc::<Lambertian>::new(Lambertian{albedo: Color::random() * Color::random()}), 0.0, 1.0));
                } else if choose_mat < 0.95 {
                    world.push(Sphere::new(center, 0.2, Arc::<Metal>::new(Metal{albedo: Color::random(), fuzz: rand::random::<f64>()})));
                } else {
                    world.push(Sphere::new(center, 0.2, Arc::<Dielectric>::new(Dielectric{ref_idx: 1.5})));
                }
            }
        }
    }

    world.push(Sphere::new(Point3{x: 0.0, y: 1.0, z: 0.0}, 1.0, Arc::<Dielectric>::new(Dielectric{ref_idx: 1.5})));
    world.push(Sphere::new(Point3{x: -4.0, y: 1.0, z: 0.0}, 1.0, Arc::<Lambertian>::new(Lambertian{albedo: Color{x: 0.4, y: 0.2, z: 0.1}})));
    world.push(Sphere::new(Point3{x: 4.0, y: 1.0, z: 0.0}, 1.0, Arc::<Metal>::new(Metal{albedo: Color{x: 0.7, y: 0.6, z: 0.5}, fuzz: 0.0})));

    world
}

fn main() {
    // Display Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = ((image_width as f64) / aspect_ratio) as usize;
    let image_color_mode = 3;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let mut image_buffer: Vec<f64> = vec![0.0; (image_width * image_height * image_color_mode) as usize];

    let world = random_moving_spheres_scene();

    // Camera
    let look_from = Point3{x: 13.0, y: 2.0, z: 3.0 };
    let look_at = Point3{x: 0.0, y: 0.0, z: 0.0};
    let v_up = Vector3{x: 0.0, y:1.0, z:0.0};
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let time0: f64 = 0.0;
    let time1: f64 = 1.0;

    let camera = Camera::new(look_from, look_at, v_up,20.0, aspect_ratio, aperture, dist_to_focus, time0, time1);

    let scale = 1.0 / (samples_per_pixel as f64);
    let mut rng = rand::thread_rng();


    let now = Instant::now();
    for row_index in 0..image_height {
        // println!("Tracing line {} of {}", row_index, image_height);
        for column_index in 0..image_width {
            let buffer_offset: usize = ((image_height - 1 - row_index) * image_width * image_color_mode + column_index * image_color_mode + 0) as usize;
            let mut color_buffer = Color{x: 0.0, y: 0.0, z: 0.0};

            for _sample_index in 0..samples_per_pixel {
                let u = (column_index as f64 + rng.gen::<f64>() ) / ((image_width - 1) as f64);
                let v = (row_index as f64 + rng.gen::<f64>() ) / ((image_height - 1) as f64);
                let ray = camera.get_ray(u, v);
                color_buffer += ray_color(&ray, &world, max_depth);
            }

            color_buffer.color_to_output(&mut image_buffer, buffer_offset, scale);
        }
    }
    println!("{} seconds elapsed", now.elapsed().as_millis());

    let window_buffer: Vec<u32> = image_buffer
        .chunks(3)
        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
        .collect();

    let mut window = Window::new(
        "Ray Tracing in Rust - Press ESC to exit",
        image_width,
        image_height,
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
            image_width,
            image_height,
        )
        .unwrap();
    }
}
