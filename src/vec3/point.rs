use std::ops;
use num::{ Num, NumCast };

pub trait PointObject {
    fn new(x: f64, y: f64, z: f64) -> Self;
    fn new_empty() -> Self;
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
    fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            x,
            y,
            z
        }
    }

    fn new_empty() -> Point {
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

    pub fn dot(&self, point: Point) -> f64 {
        self.x * point.x +
        self.y * point.y +
        self.z * point.z
    }

    pub fn cross(&self, point: Point) -> Point {
        Point { 
            x: self.y * point.z - self.z * point.x,
            y: self.z * point.x - self.x * point.z,
            z: self.x * point.y - self.y * point.x 
        }
    }

    pub fn unit_vector(&self) -> Point {
        self.clone() / self.length()
    }
}

/// Adds a point or a point offset to another point.
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

/// Subtracts a point or a point offset to another point.
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

/* Start Multiplication */
/// Multiply a point by a generic number type
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

/// Multiply x, y, and z of one point with the x, y, and z of another point.
impl ops::Mul<Point> for Point {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

/// Multiply x, y, and z of a point with the x, y, and z of an offset.
impl ops::Mul<PointOffset> for Point {
    type Output = Point;

    fn mul(self, rhs: PointOffset) -> Point {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}
/* End Multiplication */

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
            0 => {&self.x},
            1 => {&self.y},
            2 => {&self.z},
            _ => {panic!("Invalid index: '{}' of a 3D array.", index);}
        }
    }
}

impl ops::IndexMut<usize> for Point {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut f64 {
        match index {
            0 => {&mut self.x},
            1 => {&mut self.y},
            2 => {&mut self.z},
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
    fn new(x: f64, y: f64, z: f64) -> PointOffset {
        PointOffset {
            x,
            y,
            z
        }
    }

    fn new_empty() -> PointOffset {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_addition_and_subtraction() {
        let point = Point::new(1.1, 1.2, 1.3);
        let point2 = Point::new(1.0, 2.0, 3.0);
        let point = point + point2;

        assert!(point.x - 2.1 < 0.0000001);
        assert!(point.y - 3.2 < 0.0000001);
        assert!(point.z - 4.3 < 0.0000001);

        let point = Point::new(1.1, 1.2, 1.3);
        let offset = PointOffset::new(0.1, 0.2, 0.3);
        let point = point + offset;
        
        assert!(point.x - 1.2 < 0.0000001);
        assert!(point.y - 1.4 < 0.0000001);
        assert!(point.z - 1.6 < 0.0000001);

        let point = Point::new(1.1, 1.2, 1.3);
        let point2 = Point::new(1.0, 2.0, 3.0);
        let point = point - point2;

        assert!(point.x - 0.1 < 0.0000001);
        assert!(point.y + 0.8 < 0.0000001);
        assert!(point.z + 1.7 < 0.0000001);

        let point = Point::new(1.1, 1.2, 1.3);
        let offset = PointOffset::new(0.1, 0.2, 0.3);
        let point = point - offset;
        
        assert!(point.x - 1.0 < 0.0000001);
        assert!(point.y - 1.0 < 0.0000001);
        assert!(point.z - 1.0 < 0.0000001);
    }

    #[test]
    fn test_point_scalar_multiplication_division() {
        let point = Point::new(2.0, 3.0, 4.0);
        let factor = 2;

        let point_new = point * factor;

        assert!(point_new.x - 4.0 < 0.0000001);
        assert!(point_new.y - 6.0 < 0.0000001);
        assert!(point_new.z - 8.0 < 0.0000001);

        let point_new = point * factor;

        assert!(point_new.x - 4.0 < 0.0000001);
        assert!(point_new.y - 6.0 < 0.0000001);
        assert!(point_new.z - 8.0 < 0.0000001);

        // Division
        let point_new = point / factor;
        assert!(point_new.x - 1.0 < 0.0000001);
        assert!(point_new.y - 1.5 < 0.0000001);
        assert!(point_new.z - 2.0 < 0.0000001);
    }

    #[test]
    #[allow(unused)]
    fn test_point_index() {
        let point = Point::new(2.0, 3.0, 4.0);
        let x = point[0];
        let y = point[1];
        let z = point[2];

        assert!(point.x - x < 0.0000001);
        assert!(point.y - y < 0.0000001);
        assert!(point.z - z < 0.0000001);


        let point = Point::new(2.0, 3.0, 4.0);
        let mut x = point[0];
        let mut y = point[1];
        let mut z = point[2];

        assert!(point.x - x < 0.0000001);
        assert!(point.y - y < 0.0000001);
        assert!(point.z - z < 0.0000001);
    }


}