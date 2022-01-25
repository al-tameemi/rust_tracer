use std::{sync::Mutex, f64::INFINITY, fmt};

use crate::primitives::{color::Color, vector::{Vector, Vec3}, ray::Ray};
use crate::shapes::{sphere::Sphere, material::Material, hitrecord::HitRecord};
use crate::objects::camera::Camera;

use rayon::prelude::*;
use rand::{self, Rng};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{event::{DeviceEvent, MouseScrollDelta, WindowEvent, MouseButton, ElementState}, dpi::PhysicalPosition};

#[derive(Clone, Copy, PartialEq)]
enum State {
    Panning,
    Rotating,
    Static
}

pub struct Image {
    width: u32,
    height: u32,
    world: Vec<Sphere>,
    camera: Camera,
    max_depth: u32,
    max_samples: u32,
    fov: f64,
    look_from: Vector,
    look_at: Vector,
    up: Vector,
    current_depth: u32,
    current_sample: u32,
    state: State,
    steps: usize,
    full_rendered: bool
}

impl Image {
    pub fn new(width: u32, height: u32, max_samples: u32, max_depth: u32) -> Image {
        let aspect_ratio = width as f64 / height as f64;
        let world = random_world();
        let fov = 60.0;
        let look_from = Vector::new(0.0, 2.0, 4.0);
        let look_at = Vector::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let camera = Camera::from_ratio(
            aspect_ratio, 
            fov,
            look_from,
            look_at,
            up
        );

        Image {
            width,
            height,
            world,
            camera,
            max_samples,
            max_depth,
            fov,
            look_from,
            look_at,
            up,
            current_depth: 0,
            current_sample: 0,
            state: State::Static,
            steps: 3,
            full_rendered: false,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.width = new_size.width;
            self.height = new_size.height;
            self.camera = Camera::from_ratio(
                self.width as f64 / self.height as f64, 
                self.fov, 
                self.look_from, 
                self.look_at, 
                self.up
            );
        }
    }

    pub fn handle_device(&mut self, event: &DeviceEvent) {
        match event {
            DeviceEvent::Button {
                state, button, 
            } => {
                match button {
                    3 /* right click */ => {
                        match state {
                            ElementState::Pressed => {
                                self.state = State::Rotating;
                                self.steps = 3;
                            },
                            ElementState::Released => self.state = State::Static,

                        }
                    },
                    1 /* left click */ => {
                        match state {
                            ElementState::Pressed => {
                                self.state = State::Panning;
                                self.steps = 3;
                            },
                            ElementState::Released => self.state = State::Static,
                        }
                    },
                    _ => {}
                }
            },
            DeviceEvent::MouseMotion {delta: (x_delta, y_delta)} => if self.state != State::Static {
                match self.state {
                    State::Panning => {
                        let x_relative = *x_delta / self.width as f64 * 3.0;
                        let y_relative = *y_delta / self.height as f64 * 3.0;

                        let look_at = Vector::new(self.look_at.x() - x_relative, self.look_at.y() + y_relative, self.look_at.z());
                        let look_from = Vector::new(self.look_from.x() - x_relative, self.look_from.y() + y_relative, self.look_from.z());
                        println!("panning");
                        self.update_position_and_look(look_from, look_at);
                        self.full_rendered = false;
                    },
                    State::Rotating => {
                        self.full_rendered = false;
                    },
                    _ => {}
                }
                
                
                

            },
            DeviceEvent::MouseWheel {delta, ..} => {
                let scroll = match delta {
                    MouseScrollDelta::LineDelta(_, scroll) => *scroll,
                    MouseScrollDelta::PixelDelta(PhysicalPosition {
                        y: scroll,
                        ..
                    }) => *scroll as f32,
                };
                let from = Vector::new(
                    self.look_from.x(),// + (scroll / 120.0) as f64, 
                    self.look_from.y(), 
                    self.look_from.z() + (scroll / 120.0) as f64
                );
                let at = Vector::new(
                    self.look_at.x(),// + (scroll / 120.0) as f64, 
                    self.look_at.y(), 
                    self.look_at.z() + (scroll / 120.0) as f64
                );
                self.update_position_and_look(from, at);
                // self.full_rendered = false;
            },
            _ => {}
        }
    }

    pub fn update_position_and_look(&mut self, look_from: Vector, look_at: Vector) {
        self.look_at = look_at;
        self.look_from = look_from;
        self.camera = Camera::from_ratio(
            self.width as f64 / self.height as f64, 
            self.fov, 
            self.look_from, 
            self.look_at, 
            self.up
        );
    }

    pub fn update_position(&mut self, look_from: Vector) {
        self.look_from = look_from;
        self.camera = Camera::from_ratio(
            self.width as f64 / self.height as f64, 
            self.fov, 
            self.look_from, 
            self.look_at, 
            self.up
        );
    }
    
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn max_samples(&self) -> u32 {
        self.max_samples
    }
    pub fn max_depth(&self) -> u32 {
        self.max_depth
    }

    pub fn clear(&mut self, frame: &mut [u8]) {
        if !self.full_rendered {
            let _ = frame.into_par_iter().for_each(|pixel| {
                *pixel = 0;
            });
        }
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        if !self.full_rendered {
            let frame = Mutex::new(frame);
            let samples;

            if self.state == State::Static {
                self.steps = 1;
                samples = self.max_samples;
            } else {
                samples = 1;
                self.steps = 3;
            }
    
            let _ = (0..self.width).into_par_iter().step_by(self.steps).for_each(|i| {
                let _ = (0..self.height).into_par_iter().step_by(self.steps).for_each(|j| {
    
                    let index = (i + self.width * (self.height - j - 1)) as usize * 4;
                    let color = self.get_pixel_color(i, j, samples).pixels(samples);
    
                    let mut frame = frame.lock().unwrap();
                    frame[index] = color[0];
                    frame[index + 1] = color[1];
                    frame[index + 2] = color[2];
                    frame[index + 3] = 255;
                });
            });
            if self.state == State::Static {
                self.full_rendered = true;
            }
        }

        println!("finished");
    }

    fn get_pixel_color(&self, i: u32, j: u32, samples: u32) -> Color {
        let depth = if self.state == State::Static {
            self.max_depth
        } else {
            1
        };
        let mut color = Color::new_black();
        for _ in 0..samples {
            let u = (i as f64 + rand::thread_rng().gen::<f64>()) / (self.width - 1) as f64;
            let v = (j as f64 + rand::thread_rng().gen::<f64>()) / (self.height - 1) as f64;
            let ray = self.camera.get_ray(u, v);
            color = color + self.ray_color(&ray, depth);
        }
        
        color
    }

    fn ray_color(&self, ray: &Ray, depth: u32) -> Color {
        let mut rec = HitRecord::new();
        if depth <= 0 {
            return Color::new_white();
        }
    
        if self.hit(ray, 0.001, INFINITY, &mut rec) {
            let mut scattered = Ray::new(Vector::new_empty(), Vector::new_empty());
            let mut attenuation = Color::new_black();
            if rec.material.unwrap().scatter(ray, &rec, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_color(&scattered, depth - 1);
            }
            return Color::new_black();
        }
    
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Color::new_white() + t * Color::new(0.5, 0.7, 1.0);
    }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest = t_max;
    
        for object in &self.world {
            if object.hit(ray, t_min, closest, &mut temp_record) {
                hit_anything = true;
                closest = temp_record.t.unwrap();
                *hit_record = temp_record;
            }
        }
    
        hit_anything
    }
}

fn random_world() -> Vec<Sphere> {
    let mut world: Vec<Sphere> = Vec::new();

    let ground = Material::new_metal(Color::new(0.8, 0.8, 0.5), 0.2);
    let center_sphere = Material::new_dielectric(Color::new(0.8, 1.0, 0.8), 1.5);
    let material_left = Material::new_metal(Color::new(0.5, 0.5, 0.7), 0.2);
    let material_right = Material::new_metal(Color::new(0.8, 0.6, 0.2), 0.8);

    world.push(Sphere::new(Vector::new(0.0, -1000.0, 0.0), 1000.0, ground));
    world.push(Sphere::new(Vector::new(0.0, 1.0, 0.0), 1.0, center_sphere));
    world.push(Sphere::new(Vector::new(-4.0, 1.0, 0.0), 1.0, material_left));
    world.push(Sphere::new(Vector::new(4.0, 1.0, 0.0), 1.0, material_right));

    for i in -11..11 {
        for j in -12..5 {
            let mut rng = rand::thread_rng();
            let mat_rng = rng.gen::<f64>();
            let center = Vector::new(i as f64 + 0.9 * rng.gen::<f64>(), 0.2, j as f64 + 0.9 * rng.gen::<f64>());
            if (center - Vector::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = match mat_rng {
                    r if r < 0.2 => {
                        let albedo = Color::random() * Color::random();
                        Material::new_lambertian(albedo)
                    }
                    r if r < 0.8 => {
                        let albedo = Color::random_range(0.5,1.0);
                        let fuzz = rng.gen_range(0.0..0.5);
                        Material::new_metal(albedo, fuzz)
                    }
                    _ => {
                        let albedo = Color::random_range(0.8,1.0);
                        Material::new_dielectric(albedo, 1.5)
                    }
                };
                world.push(Sphere::new(center, 0.2, material));

            }
        }
    }

    world
}