use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::base::Part;

pub fn part1(r: &mut dyn Read) -> Result<String, String> {
    solve(r, Part::One)
}

pub fn part2(r: &mut dyn Read) -> Result<String, String> {
    solve(r, Part::Two)
}

fn solve(r: &mut dyn Read, part: Part) -> Result<String, String> {
    let particles = BufReader::new(r)
        .lines()
        .map(Result::unwrap)
        .map(|ref line| Particle::from_str(line))
        .map(Result::unwrap)
        .collect::<Vec<Particle>>();
    if part == Part::One {
        let (idx, _particle) = particles
            .iter()
            .enumerate()
            .min_by_key(|(_idx, particle)| particle.acc.manhattan_distance())
            .unwrap();
        return Ok(idx.to_string());
    }
    Err("not implemented yet".to_string())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Vector3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector3D {
    fn origin() -> Self {
        Vector3D { x: 0, y: 0, z: 0 }
    }

    fn manhattan_distance_to(&self, other: Vector3D) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn manhattan_distance(&self) -> i64 {
        self.manhattan_distance_to(Vector3D::origin())
    }
}

impl FromStr for Vector3D {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<&str>>();
        if parts.len() != 3 {
            return Err(format!("expected 3 parts, found {}", parts.len()));
        }
        let numbers = parts
            .into_iter()
            .map(i64::from_str)
            .map(Result::unwrap)
            .collect::<Vec<i64>>();
        Ok(Self {
            x: numbers[0],
            y: numbers[1],
            z: numbers[2],
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Particle {
    pos: Vector3D,
    vel: Vector3D,
    acc: Vector3D,
}

impl FromStr for Particle {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"p=<(?P<pos>[\-0-9,]+)>, v=<(?P<vel>[\-0-9,]+)>, a=<(?P<acc>[\-0-9,]+)>"
            )
            .unwrap();
        }
        match RE.captures(s) {
            Some(captures) => {
                let pos = Vector3D::from_str(&captures["pos"])?;
                let vel = Vector3D::from_str(&captures["vel"])?;
                let acc = Vector3D::from_str(&captures["acc"])?;
                Ok(Self {
                    pos: pos,
                    vel: vel,
                    acc: acc,
                })
            }
            None => Err(format!("invalid particle: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(example, file "testdata/day20/p1ex", "0", part1);
        test!(actual, file "../../../inputs/2017/20", "258", part1);
    }

    // mod part2 {
    //     use super::*;

    //     test!(example, "", "", part2);
    //     test!(actual, file "../../../inputs/2017/20", "", part2);
    // }
}
