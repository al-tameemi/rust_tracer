use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
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