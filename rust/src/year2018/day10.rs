use std::fmt::Write;
use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::base::grid::{Grid, Point};
use crate::base::Part;

pub fn part1(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::One)
}

pub fn part2(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::Two)
}

fn solve(r: &mut dyn io::Read, part: Part) -> Result<String, String> {
    let mut input = String::new();
    r.read_to_string(&mut input).map_err(|e| e.to_string())?;
    let mut stars = parse_input(&input);
    let (seconds, output) = run_until_aligned(&mut stars);
    match part {
        Part::One => Ok(output),
        Part::Two => Ok(seconds.to_string()),
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Star {
    position: Point,
    velocity: Point,
}

impl FromStr for Star {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref STAR_RE: Regex = Regex::new(
                r"position=< *(?P<x>-?\d+), *(?P<y>-?\d+)> velocity=< *(?P<dx>-?\d+), *(?P<dy>-?\d+)>"
            )
            .unwrap();
        }
        let captures = STAR_RE.captures(s).unwrap();
        let x = i64::from_str(&captures["x"])?;
        let y = i64::from_str(&captures["y"])?;
        let dx = i64::from_str(&captures["dx"])?;
        let dy = i64::from_str(&captures["dy"])?;
        let position = Point { x, y };
        let velocity = Point { x: dx, y: dy };
        Ok(Star { position, velocity })
    }
}

fn parse_input(input: &str) -> Vec<Star> {
    input
        .lines()
        .map(Star::from_str)
        .map(Result::unwrap)
        .collect()
}

fn print_stars(stars: &[Star]) -> String {
    let x_min = stars.iter().map(|&star| star.position.x).min().unwrap();
    let y_min = stars.iter().map(|&star| star.position.y).min().unwrap();
    let min = Point { x: x_min, y: y_min };
    let adjusted_positions = stars
        .iter()
        .map(|&star| star.position - min)
        .collect::<Vec<Point>>();
    let cols = 1 + adjusted_positions
        .iter()
        .map(|&position| position.x as usize)
        .max()
        .unwrap();
    let rows = 1 + adjusted_positions
        .iter()
        .map(|&position| position.y as usize)
        .max()
        .unwrap();
    let mut output = String::with_capacity(rows * cols);
    let mut grid = Grid::with(rows as usize, cols as usize, &'.');
    for &position in &adjusted_positions {
        let transposed = Point {
            x: position.y,
            y: position.x,
        };
        grid[transposed] = '#';
    }
    for i in 0..grid.nrows() {
        let s = grid.row(i).iter().collect::<String>();
        writeln!(&mut output, "{}", s).unwrap();
    }
    output
}

fn run_until_aligned(stars: &mut [Star]) -> (u64, String) {
    let mut seconds = 0;
    while !stars_aligned(stars) {
        step_stars(stars);
        seconds += 1;
    }
    (seconds, print_stars(stars))
}

fn step_stars(stars: &mut [Star]) {
    for star in stars {
        star.position += star.velocity;
    }
}

fn stars_aligned(stars: &[Star]) -> bool {
    let limit = 2;
    let positions = stars
        .iter()
        .map(|&star| star.position)
        .collect::<Vec<Point>>();
    stars
        .iter()
        .map(|star| distances(&star.position, &positions))
        .map(|distances| distances.into_iter().min().unwrap())
        .all(|min_distance| min_distance <= limit)
}

fn distances(p: &Point, ps: &[Point]) -> Vec<u64> {
    ps.iter()
        .filter(|&p_| p_ != p)
        .map(|&p_| p.manhattan_distance_to(p_))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(
            example,
            file "testdata/day10/ex.in",
            file "testdata/day10/ex.out",
            part1
        );
        test!(
            actual,
            file "../../../inputs/2018/10",
            file "testdata/day10/actual.out",
            part1
        );
    }

    mod part2 {
        use super::*;

        test!(example, file "testdata/day10/ex.in", "3", part2);
        test!(
            actual,
            file "../../../inputs/2018/10",
            "10355",
            part2
        );
    }
}
