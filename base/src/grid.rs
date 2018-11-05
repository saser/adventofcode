use std::default::Default;
use std::fmt;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    /// Creates a new grid, with a size of `rows` rows and `cols` cols.
    ///
    /// # Panics
    ///
    /// Panics if either `rows`, `cols`, or both are less than 1.
    pub fn new(rows: usize, cols: usize) -> Grid<T> {
        if rows < 1 || cols < 1 {
            panic!(
                "a grid must at least have 1 row and 1 col; got {rows} rows and {cols} cols",
                rows = rows,
                cols = cols
            )
        }
        let mut grid: Vec<Vec<T>> = Vec::with_capacity(rows);
        (0..cols).for_each(|_| grid.push(Vec::with_capacity(cols)));
        Grid { grid: grid }
    }

    /// Returns the number of rows in the grid.
    pub fn rows(&self) -> usize {
        self.grid.len()
    }

    /// Returns the number of columns in the grid.
    pub fn cols(&self) -> usize {
        self.grid[0].len()
    }

    /// Returns a reference to the element at `(row, col)`.
    pub fn at<Pos: Into<(usize, usize)>>(&self, pos: Pos) -> &T {
        let (row, col) = pos.into();
        &self.grid[row][col]
    }

    /// Returns a mutable reference to the element at `(row, col)`.
    pub fn at_mut<Pos: Into<(usize, usize)>>(&mut self, pos: Pos) -> &mut T {
        let (row, col) = pos.into();
        &mut self.grid[row][col]
    }
}

impl<T: Clone> Grid<T> {
    /// Returns a vector with the elements on row `row` in the grid.
    pub fn row(&self, row: usize) -> Vec<T> {
        self.grid[row].clone()
    }

    /// Returns a vector with the elements on col `col` in the grid.
    pub fn col(&self, col: usize) -> Vec<T> {
        self.grid
            .clone()
            .into_iter()
            .map(|row| row[col].clone())
            .collect()
    }
}

impl<T, Idx: Into<(usize, usize)>> Index<Idx> for Grid<T> {
    type Output = T;

    fn index(&self, index: Idx) -> &T {
        self.at(index)
    }
}

impl<T, Idx: Into<(usize, usize)>> IndexMut<Idx> for Grid<T> {
    fn index_mut(&mut self, index: Idx) -> &mut T {
        self.at_mut(index)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
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
            Turn::Clockwise => match *self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
            Turn::CounterClockwise => match *self {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },
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

impl FromStr for Turn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Turn::CounterClockwise),
            "R" => Ok(Turn::Clockwise),
            _ => Err(format!("invalid turn: `{}`", s)),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Traveler {
    pub pos: Point,
    pub direction: Direction,
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
        self.direction = self.peek_turn(turn);
    }

    /// Returns the direction the traveler would be facing after turning in the given direction.
    pub fn peek_turn(&self, turn: Turn) -> Direction {
        self.direction.turn(turn)
    }

    /// Takes one step in the travelers direction, updating its position, but not its direction.
    pub fn step(&mut self) {
        self.pos = self.peek_step();
    }

    /// Returns the position the traveler would be at after taking one step in its current
    /// direction.
    pub fn peek_step(&self) -> Point {
        self.pos + self.direction.as_point()
    }

    /// Takes `n` steps in the travelers direction, updating its position, but not its direction.
    pub fn step_n(&mut self, n: u64) {
        (0..n).for_each(|_| self.step());
    }
}
