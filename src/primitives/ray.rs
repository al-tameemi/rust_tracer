use crate::primitives::vec::Vec;

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

