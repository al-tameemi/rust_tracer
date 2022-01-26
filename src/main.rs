mod primitives;
mod objects;
mod shapes;
mod image;

use pixels::{Error, Pixels, SurfaceTexture};
use primitives::{color::Color, vector::{Vector, Vec3}, ray::Ray};
use crate::image::Image;

use ::image::{RgbImage, Rgb, ImageBuffer};
use rayon::prelude::*;
use shapes::{hitrecord::HitRecord, sphere::Sphere, material::{Material}};
use std::{f64::{consts::PI, INFINITY}, sync::Mutex, time::Instant};
use rand::prelude::*;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder, dpi::PhysicalPosition
};

const SAMPLES_PER_PIXEL: u32 = 50;
const MAX_DEPTH: u32 = 20;

fn main() {

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let window_size = window.inner_size();

    let mut pixels = {
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_size.width, window_size.height, surface_texture).unwrap()

    };
    // not needed anymore
    drop(window_size);

    let mut image = Image::new(window_size.width, window_size.height as u32, MAX_DEPTH, SAMPLES_PER_PIXEL);

    let mut window_focused = true;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {ref event, window_id} if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => {
                        *control_flow = ControlFlow::Exit;
                    },
                    WindowEvent::Resized(physical_size) => {
                        pixels.resize_surface(physical_size.width, physical_size.height);
                        pixels.resize_buffer(physical_size.width, physical_size.height);
                        image.resize(*physical_size);

                    },
                    WindowEvent::ScaleFactorChanged {new_inner_size, ..} => {
                        pixels.resize_surface(new_inner_size.width, new_inner_size.height);
                        pixels.resize_buffer(new_inner_size.width, new_inner_size.height);
                        image.resize(**new_inner_size);
                    },
                    WindowEvent::Focused(
                        is_focused
                    ) => {
                        window_focused = *is_focused;
                    }
                    _ => {}
                }
            },
            Event::DeviceEvent {ref event, ..} => {
                image.handle_device(event, window_focused);
            }

            Event::RedrawRequested(window_id) if window_id == window.id() => {
                image.clear(pixels.get_frame());
                image.draw(pixels.get_frame());
                pixels.render().unwrap();
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });








    // image.render();
    // let rgb_image = Mutex::new(RgbImage::new(image.width() as u32, image.height() as u32));
    // let _ = (0..image.height())
    //     .into_par_iter()
    //     .step_by(1)
    //     .for_each(|j| {
    //         // print!("line {j}\n");
    //         let _ = (0..image.width())
    //             .into_par_iter()
    //             .step_by(1)
    //             .for_each(|i| {
    //                 rgb_image.lock().unwrap().put_pixel(i as u32, j as u32  , Rgb(image.pixels.lock().unwrap()[i][image.height() - j - 1].pixels(image.max_samples() as i32)));
    //             });
    //     });

    // rgb_image.lock().unwrap().save("image.png").unwrap();
}

// fn main() {

//     let image = Image::new_with_height(3.0 / 2.0, 400);
//     let camera = Camera::from_image(
//         &image, 
//         60.0, 
//         Vector::new(5.0, 2.0, 4.0), 
//         Vector::new(0.0, 0.0, 0.0), 
//         Vector::new(0.0, 1.0, 0.0)
//     );

//     let world = random_world();


//     let start_2 = Instant::now();
//     let rgb_image_2 = multi_threaded(&image, &camera, &world);
//     let duration_2 = start_2.elapsed();

//     rgb_image_2.lock().unwrap().save("image.png").unwrap();

//     println!("Multi thread completed: {:?}", duration_2);
// }

// fn random_world() -> Vec<Box<Sphere>> {
//     let mut world: Vec<Box<Sphere>> = Vec::new();

//     // let ground = Material::new_metal(Color::new(0.8, 0.8, 0.5), 0.2);
//     let center_sphere = Material::new_dielectric(Color::new(0.8, 1.0, 0.8), 1.5);
//     let material_left = Material::new_metal(Color::new(0.5, 0.5, 0.7), 0.2);
//     let material_right = Material::new_metal(Color::new(0.8, 0.6, 0.2), 0.8);

//     // world.push(Box::new(Sphere::new(Vector::new(0.0, -1000.0, 0.0), 1000.0, ground)));
//     world.push(Box::new(Sphere::new(Vector::new(0.0, 1.0, 0.0), 1.0, center_sphere)));
//     world.push(Box::new(Sphere::new(Vector::new(-4.0, 1.0, 0.0), 1.0, material_left)));
//     world.push(Box::new(Sphere::new(Vector::new(4.0, 1.0, 0.0), 1.0, material_right)));

//     for i in -11..11 {
//         for j in -12..5 {
//             let mut rng = rand::thread_rng();
//             let mat_rng = rng.gen::<f64>();
//             let center = Vector::new(i as f64 + 0.9 * rng.gen::<f64>(), 0.2, j as f64 + 0.9 * rng.gen::<f64>());
//             if (center - Vector::new(4.0, 0.2, 0.0)).length() > 0.9 {
//                 let material = match mat_rng {
//                     r if r < 0.2 => {
//                         let albedo = Color::random() * Color::random();
//                         Material::new_lambertian(albedo)
//                     }
//                     r if r < 0.8 => {
//                         let albedo = Color::random_range(0.5,1.0);
//                         let fuzz = rng.gen_range(0.0..0.5);
//                         Material::new_metal(albedo, fuzz)
//                     }
//                     _ => {
//                         let albedo = Color::random_range(0.8,1.0);
//                         Material::new_dielectric(albedo, 1.5)
//                     }
//                 };
//                 world.push(Box::new(Sphere::new(center, 0.2, material)));

//             }
//         }
//     }

//     world
// }

// fn multi_threaded(image: &Image, camera: &Camera, world: &Vec<Box<Sphere>>) -> Mutex<ImageBuffer<Rgb<u8>, Vec<u8>>> {
//     let rgb_image = Mutex::new(RgbImage::new(image.width as u32, image.height as u32));
//     let _ = (0..image.height)
//         .into_par_iter()
//         .rev()
//         .step_by(1)
//         .for_each(|j| {
//             // print!("line {j}\n");
//             let _ = (0..image.width)
//                 .into_par_iter()
//                 .step_by(1)
//                 .for_each(|i| {
//                     let color = get_pixel_color(i, j, image, camera, world);
//                     rgb_image.lock().unwrap().put_pixel(i as u32, (image.height - 1 - j) as u32  , Rgb(color.pixels(SAMPLES_PER_PIXEL)));
//                 });
//         });

//     rgb_image
// }

// fn get_pixel_color(i: i32, j: i32, image: &Image, camera: &Camera, world: &Vec<Box<Sphere>>) -> Color {
//     let mut color = Color::new_black();
//     for _ in 0.. SAMPLES_PER_PIXEL {
//         let u = (i as f64 + rand::thread_rng().gen::<f64>()) / (image.width - 1) as f64;
//         let v = (j as f64 + rand::thread_rng().gen::<f64>()) / (image.height - 1) as f64;
//         let ray = camera.get_ray(u, v);
//         color = color + ray_color(&ray, world, MAX_DEPTH);
//     }
//     color
// }

// fn ray_color(ray: &Ray, world: &Vec<Box<Sphere>>, depth: i32) -> Color {
//     let mut rec = HitRecord::new();
//     if depth <= 0 {
//         return Color::new_white();
//     }

//     if hit(world, ray, 0.001, INFINITY, &mut rec) {
//         let mut scattered = Ray::new(Vector::new_empty(), Vector::new_empty());
//         let mut attenuation = Color::new_black();
//         if rec.material.unwrap().scatter(ray, &rec, &mut attenuation, &mut scattered) {
//             return attenuation * ray_color(&scattered, world, depth - 1);
//         }
//         return Color::new_black();
//     }

//     let unit_direction = ray.direction.unit_vector();
//     let t = 0.5 * (unit_direction.y() + 1.0);
//     return (1.0 - t) * Color::new_white() + t * Color::new(0.5, 0.7, 1.0);
// }

// fn hit(objects: &Vec<Box<Sphere>>, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
//     let mut temp_record = HitRecord::new();
//     let mut hit_anything = false;
//     let mut closest = t_max;

//     for object in objects {
//         if object.hit(ray, t_min, closest, &mut temp_record) {
//             hit_anything = true;
//             closest = temp_record.t.unwrap();
//             *hit_record = temp_record;
//         }
//     }

//     hit_anything
// }

// fn degree_to_radian(degrees: f64) -> f64 {
//     return degrees * PI / 180.0;
// }