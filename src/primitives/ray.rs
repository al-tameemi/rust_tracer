use crate::primitives::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Vec,
    pub direction: Vec,
}

impl Ray {
    pub fn new(origin: Vec, direction: Vec) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    pub fn at(&self, t: f64) -> Vec {
        self.origin + t * self.direction
    }
}

