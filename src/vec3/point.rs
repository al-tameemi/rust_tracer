use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new(&self, x: f64, y: f64, z: f64) -> Point {
        Point {
            x,
            y,
            z
        }
    }

    pub fn new_empty(&self) -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl ops::Add<PointOffset> for Point {
    type Output = Point;

    fn add(self, rhs: PointOffset) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<Point> for PointOffset {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Point {
        Point {
            x: self.x * rhs as f64,
            y: self.y * rhs as f64,
            z: self.z * rhs as f64,
        }
    }
}

impl ops::Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Point {
        self * (1.0 / rhs)
    }
}

impl ops::Div<i32> for Point {
    type Output = Point;

    fn div(self, rhs: i32) -> Point {
        self * (1.0 / rhs as f64)
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

impl ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        self * -1.0
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointOffset {
    x: f64,
    y: f64,
    z: f64,
}

impl PointOffset {
    pub fn new(&self, x: f64, y: f64, z: f64) -> PointOffset {
        PointOffset {
            x,
            y,
            z
        }
    }

    pub fn new_empty(&self) -> PointOffset {
        PointOffset {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
}