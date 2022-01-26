use std::f64::consts::PI;

use crate::{primitives::{vector::Vector, ray::Ray}};

pub struct Camera {
    pub origin: Vector,
    pub horizontal: Vector,
    pub vertical: Vector,
    pub lower_left_corner: Vector    
}

impl Camera {
    pub fn from_ratio(aspect_ratio: f64, fov: f64, look_from: Vector, look_at: Vector, up: Vector) -> Camera {
        let theta = degree_to_radian(fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        // let focal_length = 1.0;

        let w = (look_from - look_at).unit_vector();
        let u = up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
        
        Camera { 
            origin, 
            horizontal, 
            vertical, 
            lower_left_corner 
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray { 
            origin: self.origin, 
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin 
        }
    }
}

fn degree_to_radian(deg: f64) -> f64 {
    deg * PI / 180.0
}