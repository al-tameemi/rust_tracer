use super::image::Image;
use crate::{primitives::{vector::{Vector, Vec3}, ray::Ray}, degree_to_radian};

pub struct Camera {
    pub origin: Vector,
    pub horizontal: Vector,
    pub vertical: Vector,
    pub lower_left_corner: Vector    
}

impl Camera {
    pub fn from_image(image: &Image, fov: f64) -> Camera {

        let theta = degree_to_radian(fov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = image.aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vector::new(0.0, 0.0, 0.0);
        let horizontal = Vector::new(viewport_width, 0.0, 0.0);
        let vertical = Vector::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 as f64 - vertical / 2.0 as f64 - Vector::new(0.0, 0.0, focal_length);
        
        Camera { 
            origin, 
            horizontal, 
            vertical, 
            lower_left_corner 
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray { 
            origin: self.origin, 
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin 
        }
    }
}