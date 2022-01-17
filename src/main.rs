mod primitives;
mod objects;
mod shapes;

use primitives::{color::Color, vector::{Vector, Vec3}, ray::Ray};
use objects::{image::Image, camera::Camera};
use image::{RgbImage, Rgb, ImageBuffer};
use rayon::prelude::*;
use shapes::{hittable::{Hittable, HitRecord}, sphere::Sphere};
use std::{f64::{consts::PI, INFINITY}, sync::{Arc, Mutex}, time::{Instant}};
use rand::prelude::*;

const SAMPLES_PER_PIXEL: i32 = 30;

fn main() {

    let image = Image::new_with_height(16.0 / 9.0, 1440);
    let camera = Camera::from_image(&image);
    let mut world: Vec<Box<dyn Hittable + Send + Sync>> = Vec::new();
    world.push(Box::new(Sphere::new(Vector::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vector::new(0.0, -100.5, -1.0), 100.0)));


    let start = Instant::now();
    let rgb_image = single_threaded(&image, &camera, &world);
    let duration_1 = start.elapsed();

    rgb_image.save("image.png").unwrap();

    println!("single thread completed");

    let image = Image::new_with_height(16.0 / 9.0, 1440);
    let camera = Camera::from_image(&image);

    let start_2 = Instant::now();
    let rgb_image_2 = multi_threaded(&image, &camera, &world);
    let duration_2 = start_2.elapsed();

    rgb_image_2.lock().unwrap().save("image.png").unwrap();


    println!("Single-threaded: {:?}, Multi-threaded: {:?} ", duration_1, duration_2);
}

fn single_threaded(image: &Image, camera: &Camera, world: &Vec<Box<dyn Hittable + Send + Sync>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut rgb_image = RgbImage::new(image.width as u32, image.height as u32);

    for j in (0..image.height).rev() {
        for i in 0..image.width {
            let color = get_pixel_color(i, j, image, camera, world);
            rgb_image.put_pixel(i as u32, (image.height - 1 - j) as u32  , Rgb(color.pixels(SAMPLES_PER_PIXEL)));
        }
    }

    rgb_image
}

fn multi_threaded(image: &Image, camera: &Camera, world: &Vec<Box<dyn Hittable + Send + Sync>>) -> Mutex<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let rgb_image = Mutex::new(RgbImage::new(image.width as u32, image.height as u32));
    // let world = Mutex::new(world);
    let _ = (0..image.height)
        .into_par_iter()
        .rev()
        .for_each(|j| {
            let _ = (0..image.width)
                .into_par_iter()
                .for_each(|i| {
                    let color = get_pixel_color(i, j, image, camera, world);
                    rgb_image.lock().unwrap().put_pixel(i as u32, (image.height - 1 - j) as u32  , Rgb(color.pixels(SAMPLES_PER_PIXEL)));
                });
        });

    rgb_image
}

fn get_pixel_color(i: i32, j: i32, image: &Image, camera: &Camera, world: &Vec<Box<dyn Hittable + Send + Sync>>) -> Color {
    let mut color = Color::new(0.0, 0.0, 0.0);
    for _ in 0.. SAMPLES_PER_PIXEL {
        let u = (i as f64 + rand::thread_rng().gen::<f64>()) / (image.width - 1) as f64;
        let v = (j as f64 + rand::thread_rng().gen::<f64>()) / (image.height - 1) as f64;
        let ray = camera.get_ray(u, v);
        color = color + ray_color(&ray, world);
    }
    color
}

fn ray_color(ray: &Ray, world: &Vec<Box<dyn Hittable + Send + Sync>>) -> Color {
    let mut rec = HitRecord::new();
    if hit(world, ray, 0.0, INFINITY, &mut rec) {
        return 0.5 * (Color::new(1.0, 1.0, 1.0) + rec.normal.unwrap());
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn hits_sphere(ray: &Ray, center: Vector, radius: f64) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / (a);
    }
}

fn hit(objects: &Vec<Box<dyn Hittable + Send + Sync>>,ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
    let mut temp_record = HitRecord::new();
    let mut hit_anything = false;
    let mut closest = t_max;

    for object in objects {
        if object.hit(ray, t_min, closest, &mut temp_record) {
            hit_anything = true;
            closest = temp_record.t.unwrap();
            *hit_record = temp_record;
        }
    }

    hit_anything
}

fn degree_to_radian(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}