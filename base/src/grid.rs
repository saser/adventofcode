use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        *self = Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, rhs: isize) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<isize> for Point {
    fn mul_assign(&mut self, rhs: isize) {
        *self = Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}
