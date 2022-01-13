use std::ops;
use num::{ Num, NumCast };

pub trait PointObject {
    fn new(&self, x: f64, y: f64, z: f64) -> Self;
    fn new_empty(&self) -> Self;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl PointObject for Point {
    fn new(&self, x: f64, y: f64, z: f64) -> Point {
        Point {
            x,
            y,
            z
        }
    }

    fn new_empty(&self) -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }
}

impl Point {

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl<T> ops::Add<T> for Point where T: PointObject {
    type Output = Point;

    fn add(self, rhs: T) -> Point {
        Point {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
        }
    }
} 

impl<T> ops::Sub<T> for Point where T: PointObject {
    type Output = Point;

    fn sub(self, rhs: T) -> Point {
        Point {
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
            z: self.z - rhs.z(),
        }
    }
} 

impl<T> ops::Mul<T> for Point where T: Num + NumCast {
    type Output = Point;

    fn mul(self, rhs: T) -> Point {
        let rhs = T::to_f64(&rhs).unwrap();
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> ops::Div<T> for Point where T: Num + NumCast {
    type Output = Point;

    fn div(self, rhs: T) -> Point {
        self * (1.0 / T::to_f64(&rhs).unwrap())
    }
}


impl ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        self * -1.0
    }
}

impl ops::Index<usize> for Point {
    type Output = f64;

    fn index<'a>(&'a self, index: usize) -> &'a f64 {
        match index {
            1 => {&self.x},
            2 => {&self.y},
            3 => {&self.z},
            _ => {panic!("Invalid index: '{}' of a 3D array.", index);}
        }
    }
}

impl ops::IndexMut<usize> for Point {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut f64 {
        match index {
            1 => {&mut self.x},
            2 => {&mut self.y},
            3 => {&mut self.z},
            _ => {panic!("Invalid index: '{}' of a 3D array.", index);}
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointOffset {
    x: f64,
    y: f64,
    z: f64,
}

impl PointObject for PointOffset {
    fn new(&self, x: f64, y: f64, z: f64) -> PointOffset {
        PointOffset {
            x,
            y,
            z
        }
    }

    fn new_empty(&self) -> PointOffset {
        PointOffset {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }
}