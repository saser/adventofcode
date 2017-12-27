extern crate base;

use base::{Part, Solver};
use std::ops::{Add, Neg};
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day11)
}

struct Day11;

impl Solver for Day11 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let directions = parse_input(input);
        let distance = directions.as_slice()
            .iter()
            .map(HexDirection::as_point)
            .fold(Point3D::origin(), |point, dir| point + dir)
            .manhattan_distance() / 2;
        Ok(distance.to_string())
    }
}

fn parse_input(input: &str) -> Vec<HexDirection> {
    input.split(',')
        .map(HexDirection::from_str)
        .map(Result::unwrap)
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    fn from(x: i64, y: i64, z: i64) -> Point3D {
        Point3D { x: x, y: y, z: z }
    }

    fn origin() -> Point3D {
        Point3D::from(0, 0, 0)
    }

    fn manhattan_distance(&self) -> u64 {
        (self.x.abs() + self.y.abs() + self.z.abs()) as u64
    }
}

impl Add for Point3D {
    type Output = Point3D;

    fn add(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum HexDirection {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl HexDirection {
    fn as_point(&self) -> Point3D {
        // A hexgrid can be represented as a "stack of boxes" in a kind of staircase pattern.
        match *self {
            HexDirection::NorthEast => Point3D::from(1, 0, 1),
            HexDirection::SouthWest => Point3D::from(-1, 0, -1),
            HexDirection::North => Point3D::from(0, 1, 1),
            HexDirection::South => Point3D::from(0, -1, -1),
            HexDirection::NorthWest => Point3D::from(-1, 1, 0),
            HexDirection::SouthEast => Point3D::from(1, -1, 0),
        }
    }
}

impl FromStr for HexDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<HexDirection, String> {
        match s {
            "n" => Ok(HexDirection::North),
            "ne" => Ok(HexDirection::NorthEast),
            "se" => Ok(HexDirection::SouthEast),
            "s" => Ok(HexDirection::South),
            "sw" => Ok(HexDirection::SouthWest),
            "nw" => Ok(HexDirection::NorthWest),
            _ => Err(format!("invalid hex-direction: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "ne,ne,ne";
            let expected = "3";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "ne,ne,sw,sw";
            let expected = "0";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "ne,ne,s,s";
            let expected = "2";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_4() {
            let solver = get_solver();
            let input = "se,sw,se,sw,sw";
            let expected = "3";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }
}
