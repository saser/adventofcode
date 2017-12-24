use std::default::Default;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(&self, turn: Turn) -> Direction {
        match turn {
            Turn::Clockwise => {
                match *self {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                }
            }
            Turn::CounterClockwise => {
                match *self {
                    Direction::North => Direction::West,
                    Direction::East => Direction::North,
                    Direction::South => Direction::East,
                    Direction::West => Direction::South,
                }
            }
        }
    }

    fn as_point(&self) -> Point {
        let x = match *self {
            Direction::East => 1,
            Direction::West => -1,
            _ => 0,
        };
        let y = match *self {
            Direction::North => 1,
            Direction::South => -1,
            _ => 0,
        };
        Point { x: x, y: y }
    }
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::North
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Turn {
    Clockwise,
    CounterClockwise,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Traveler {
    pos: Point,
    direction: Direction,
}

impl Traveler {
    /// Turns the traveler, updating its direction, but not its position.
    pub fn turn(&mut self, turn: Turn) {
        self.direction = self.direction.turn(turn);
    }

    /// Takes one step in the travelers direction, updating its position, but not its direction.
    pub fn step(&mut self) {
        self.pos = self.pos + self.direction.as_point();
    }

    /// Takes `n` steps in the travelers direction, updating its position, but not its direction.
    pub fn step_n(&mut self, n: usize) {
        (0..n).for_each(|_| self.step());
    }
}
