use std::ops::{self, Range};

use super::vector::{Vector, Vec3};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {return min;}
    if x > max {return max;}
    x
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            r,
            g,
            b
        }
    }

    pub fn new_white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0
        }
    }

    pub fn new_black() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0
        }
    }

    pub fn random() -> Self {
        Color {
            r: rand::thread_rng().gen::<f64>(),
            g: rand::thread_rng().gen::<f64>(),
            b: rand::thread_rng().gen::<f64>()
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Color {
            r: rand::thread_rng().gen_range(min..max),
            g: rand::thread_rng().gen_range(min..max),
            b: rand::thread_rng().gen_range(min..max)
        }
    }

    pub fn write_color(&self) -> String {
        format!("{} {} {}\n", (256.0 * clamp(self.r, 0.0, 0.999)) as u8, (256.0 * clamp(self.g, 0.0, 0.999)) as u8, (256.0 * clamp(self.b, 0.0, 0.999)) as u8)
    }

    pub fn pixels(&self, samples_per_pixel: i32) -> [u8; 3] {
        let r = (self.r / samples_per_pixel as f64).sqrt();
        let g = (self.g / samples_per_pixel as f64).sqrt();
        let b = (self.b / samples_per_pixel as f64).sqrt();

        [(256.0 * clamp(r, 0.0, 0.999)) as u8, (256.0 * clamp(g, 0.0, 0.999)) as u8, (256.0 * clamp(b, 0.0, 0.999)) as u8]
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::Add<Vector> for Color {
    type Output = Color;

    fn add(self, rhs: Vector) -> Color {
        Color {
            r: self.r + rhs.x(),
            g: self.g + rhs.y(),
            b: self.b + rhs.z(),
        }
    }
}

impl ops::Index<usize> for Color {
    type Output = f64;

    fn index<'a>(&'a self, index: usize) -> &'a f64 {
        match index {
            1 => {&self.r},
            2 => {&self.g},
            3 => {&self.b},
            _ => {panic!("Invalid index: '{}' of a 3D array.", index);}
        }
    }
}

impl ops::IndexMut<usize> for Color {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut f64 {
        match index {
            1 => {&mut self.r},
            2 => {&mut self.g},
            3 => {&mut self.b},
            _ => {panic!("Invalid index: '{}' of a 3D array.", index);}
        }
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;
    
    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b
        }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: rhs.r * self,
            g: rhs.g * self,
            b: rhs.b * self,
        }
    }
}

impl ops::Mul<i32> for Color {
    type Output = Color;

    fn mul(self, rhs: i32) -> Color {
        Color {
            r: self.r * rhs as f64,
            g: self.g * rhs as f64,
            b: self.b * rhs as f64,
        }
    }
}

impl ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Color {
        self * (1.0 / rhs)
    }
}

impl ops::Div<i32> for Color {
    type Output = Color;

    fn div(self, rhs: i32) -> Color {
        self * (1.0 / rhs as f64)
    }
}

impl ops::Neg for Color {
    type Output = Color;

    fn neg(self) -> Color {
        self * -1.0
    }
}