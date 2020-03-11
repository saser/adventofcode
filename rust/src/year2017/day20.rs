use std::cmp::Ordering;
use std::collections::BTreeSet;
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
    let p0 = Particle {
        pos: Vector3D { x: -6, y: 0, z: 0 },
        vel: Vector3D { x: 3, y: 0, z: 0 },
        acc: Vector3D { x: 0, y: 0, z: 0 },
    };
    let p1 = Particle {
        pos: Vector3D { x: -4, y: 0, z: 0 },
        vel: Vector3D { x: 2, y: 0, z: 0 },
        acc: Vector3D { x: 0, y: 0, z: 0 },
    };
    if let Some(t) = p0.collides_with(p1) {
        println!("YES: {}", t);
    }
    Err("not implemented yet".to_string())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Vector3D {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Solution {
    None,
    One(i64),
    Two(i64, i64),
    Infinite,
}

impl Solution {
    fn intersection(&self, other: Solution) -> Solution {
        match (*self, other) {
            (Solution::Infinite, _) => other,
            (_, Solution::Infinite) => *self,
            (Solution::One(t1), Solution::One(t2)) if t1 == t2 => *self,
            (Solution::One(t1), Solution::Two(t2, t3)) if t1 == t2 || t1 == t3 => *self,
            (Solution::Two(t1, t2), Solution::One(t3)) if t1 == t3 || t2 == t3 => other,
            (Solution::Two(t1, t2), Solution::Two(t3, t4)) => {
                let min1 = t1.min(t2);
                let max1 = t1.max(t2);
                let min2 = t3.min(t4);
                let max2 = t3.max(t4);
                if (min1, max1) == (min2, max2) {
                    Solution::Two(min1, max1)
                } else if min1 == min2 || min1 == max2 {
                    Solution::One(min1)
                } else if max1 == min2 || max1 == max2 {
                    Solution::One(max1)
                } else {
                    Solution::None
                }
            }
            _ => Solution::None,
        }
    }
}

fn solve_quadratic(a: i64, b: i64, c: i64) -> Solution {
    if a == 0 {
        // bx + c = 0
        if b == 0 {
            if c == 0 {
                return Solution::Infinite;
            }
            return Solution::None;
        }
        return match -c % b {
            0 => Solution::One(-c / b),
            _ => Solution::None,
        };
    }
    let discriminant = b.pow(2) - 4 * a * c;
    match discriminant.cmp(&0) {
        Ordering::Less => Solution::None,
        Ordering::Equal => match -b % (2 * a) {
            0 => Solution::One(-b / (2 * a)),
            _ => Solution::None,
        },
        Ordering::Greater => {
            let root = ((discriminant as f64).sqrt()) as i64;
            if root.pow(2) != discriminant {
                return Solution::None;
            }
            let numer1 = -b + root;
            let numer2 = -b - root;
            let mut solutions = vec![];
            if numer1 % (2 * a) == 0 {
                solutions.push(numer1 / (2 * a));
            }
            if numer2 % (2 * a) == 0 {
                solutions.push(numer2 / (2 * a));
            }
            match solutions.as_slice() {
                [x] => Solution::One(*x),
                [x1, x2] => Solution::Two(*x1, *x2),
                _ => unreachable!(),
            }
        }
    }
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

impl Particle {
    fn collides_with(&self, other: Particle) -> Option<i64> {
        let a_x = self.acc.x - other.acc.x;
        let b_x = 2 * (self.vel.x - other.vel.x);
        let c_x = 2 * (self.pos.x - other.pos.x);
        let solution_x = solve_quadratic(a_x, b_x, c_x);
        println!("solution_x={:?}", solution_x);
        let mut t_x = BTreeSet::new();
        if solution_x == Solution::None {
            return None;
        } else if let Solution::One(t) = solution_x {
            if t >= 0 {
                t_x.insert(t);
            }
        } else if let Solution::Two(t1, t2) = solution_x {
            if t1 >= 0 {
                t_x.insert(t1);
            }
            if t2 >= 0 {
                t_x.insert(t2);
            }
        }
        let a_y = self.acc.y - other.acc.y;
        let b_y = 2 * (self.vel.y - other.vel.y);
        let c_y = 2 * (self.pos.y - other.pos.y);
        let solution_y = solve_quadratic(a_y, b_y, c_y);
        println!("solution_y={:?}", solution_y);
        let mut t_y = BTreeSet::new();
        if solution_y == Solution::None {
            return None;
        } else if let Solution::One(t) = solution_y {
            if t >= 0 {
                t_y.insert(t);
            }
        } else if let Solution::Two(t1, t2) = solution_y {
            if t1 >= 0 {
                t_y.insert(t1);
            }
            if t2 >= 0 {
                t_y.insert(t2);
            }
        }
        let a_z = self.acc.z - other.acc.z;
        let b_z = 2 * (self.vel.z - other.vel.z);
        let c_z = 2 * (self.pos.z - other.pos.z);
        let solution_z = solve_quadratic(a_z, b_z, c_z);
        println!("solution_z={:?}", solution_z);
        let mut t_z = BTreeSet::new();
        if solution_z == Solution::None {
            return None;
        } else if let Solution::One(t) = solution_z {
            if t >= 0 {
                t_z.insert(t);
            }
        } else if let Solution::Two(t1, t2) = solution_z {
            if t1 >= 0 {
                t_z.insert(t1);
            }
            if t2 >= 0 {
                t_z.insert(t2);
            }
        }
        if (solution_x, solution_y, solution_z)
            == (Solution::Infinite, Solution::Infinite, Solution::Infinite)
        {
            return Some(0);
        }
        let mut t = t_x.clone();
        if !t_y.is_empty() {
            t = t.intersection(&t_y).cloned().collect();
        }
        if !t_z.is_empty() {
            t = t.intersection(&t_z).cloned().collect();
        }
        match t.iter().next() {
            Some(i) => Some(*i),
            None => None,
        }
    }
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
