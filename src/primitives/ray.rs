use crate::primitives::vector::{Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    pub fn at(&self, t: f64) -> Vector {
        self.origin + t * self.direction
    }
}

