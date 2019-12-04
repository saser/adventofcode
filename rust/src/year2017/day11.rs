use crate::base::Part;
use std::io;
use std::ops::Add;
use std::str::FromStr;

pub fn part1(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::One)
}

pub fn part2(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::Two)
}

fn solve(r: &mut dyn io::Read, part: Part) -> Result<String, String> {
    let mut input = String::new();
    r.read_to_string(&mut input).map_err(|e| e.to_string())?;
    let directions = parse_input(input.trim());
    let (final_position, furthest) = directions
        .as_slice()
        .iter()
        .map(|hex_dir| hex_dir.as_point())
        .fold((Point3D::origin(), 0), |(point, furthest), dir| {
            let new_point = point + dir;
            let new_furthest = std::cmp::max(furthest, new_point.manhattan_distance() / 2);
            (new_point, new_furthest)
        });
    match part {
        Part::One => {
            let distance = final_position.manhattan_distance() / 2;
            Ok(distance.to_string())
        }
        Part::Two => Ok(furthest.to_string()),
    }
}

fn parse_input(input: &str) -> Vec<HexDirection> {
    input
        .split(',')
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
        Point3D { x, y, z }
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
    fn as_point(self) -> Point3D {
        // A hexgrid can be represented as a "stack of boxes" in a kind of staircase pattern.
        match self {
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
    use crate::test;

    mod part1 {
        use super::*;

        test!(example1, "ne,ne,ne", "3", part1);
        test!(example2, "ne,ne,sw,sw", "0", part1);
        test!(example3, "ne,ne,s,s", "2", part1);
        test!(example4, "se,sw,se,sw,sw", "3", part1);
        test!(actual, file "../../../inputs/2017/11", "761", part1);
    }

    mod part2 {
        use super::*;

        test!(actual, file "../../../inputs/2017/11", "1542", part2);
    }
}
