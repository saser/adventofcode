use std::default::Default;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
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

impl Mul<i64> for Point {
    type Output = Point;

    fn mul(self, rhs: i64) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<i64> for Point {
    fn mul_assign(&mut self, rhs: i64) {
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

impl Point {
    /// Returns a `Point` representing the origin of a grid, i.e., `(0, 0)`.
    pub fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    /// Calculates the Manhattan distance to the origin.
    pub fn manhattan_distance(&self) -> u64 {
        self.manhattan_distance_to(Point::origin())
    }

    /// Calculates the Manhattan distance to another `Point`.
    pub fn manhattan_distance_to(&self, other: Point) -> u64 {
        let d_x = self.x - other.x;
        let d_y = self.y - other.y;
        d_x.abs() as u64 + d_y.abs() as u64
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
    pub fn turn(&self, turn: Turn) -> Direction {
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

    pub fn as_point(&self) -> Point {
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
    /// Returns the travelers current position as a `Point`.
    pub fn pos(&self) -> Point {
        self.pos
    }

    /// Returns the travelers current direction.
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// Turns the traveler, updating its direction, but not its position.
    pub fn turn(&mut self, turn: Turn) {
        self.direction = self.direction.turn(turn);
    }

    /// Takes one step in the travelers direction, updating its position, but not its direction.
    pub fn step(&mut self) {
        self.pos = self.pos + self.direction.as_point();
    }

    /// Takes `n` steps in the travelers direction, updating its position, but not its direction.
    pub fn step_n(&mut self, n: u64) {
        (0..n).for_each(|_| self.step());
    }
}
