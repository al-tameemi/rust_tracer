use super::image::Image;
use crate::primitives::vector::{Vector, Vec3};

pub struct Camera {
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,
    pub origin: Vector,
    pub horizontal: Vector,
    pub vertical: Vector,
    pub lower_left_corner: Vector    
}

impl Camera {
    pub fn from_image(image: &Image) -> Camera {
        let viewport_height = 2.0;
        let viewport_width = image.aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vector::new(0.0, 0.0, 0.0);
        let horizontal = Vector::new(viewport_width, 0.0, 0.0);
        let vertical = Vector::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 as f64 - vertical / 2.0 as f64 - Vector::new(0.0, 0.0, focal_length);
        
        Camera { 
            viewport_height, 
            viewport_width, 
            focal_length, 
            origin, 
            horizontal, 
            vertical, 
            lower_left_corner 
        }
    }
}