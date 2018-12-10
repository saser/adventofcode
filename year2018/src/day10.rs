use lazy_static::lazy_static;
use regex::Regex;

use std::fmt::Write;
use std::num::ParseIntError;
use std::str::FromStr;

use base::grid::{Grid, Point};
use base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day10)
}

struct Day10;

impl Solver for Day10 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let mut stars = parse_input(input);
        match part {
            Part::One => {
                let (_seconds, output) = run_until_message(&mut stars);
                Ok(output)
            }
            Part::Two => {
                let (seconds, _output) = run_until_message(&mut stars);
                Ok(seconds.to_string())
            }
        }
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
    output.push('\n');
    let mut grid = Grid::with(rows as usize, cols as usize, &' ');
    for &position in &adjusted_positions {
        let transposed = Point {
            x: position.y,
            y: position.x,
        };
        grid[transposed] = '#';
    }
    for i in 0..grid.rows() {
        let s = grid.row(i).iter().collect::<String>();
        writeln!(&mut output, "{}", s).unwrap();
    }
    output
}

fn run_until_message(stars: &mut [Star]) -> (u64, String) {
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
        .map(|star| mdiffs(&star.position, &positions))
        .map(|mdiffs| mdiffs.into_iter().min().unwrap())
        .all(|min_mdiff| min_mdiff <= limit)
}

fn mdiff(p1: &Point, p2: &Point) -> u64 {
    p1.manhattan_distance_to(*p2)
}

fn mdiffs(p: &Point, ps: &[Point]) -> Vec<u64> {
    ps.iter()
        .filter(|&p_| p_ != p)
        .map(|p_| mdiff(p, p_))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/10").trim();
            let expected = "\
#    #  #####   #####   #    #  #####   #####   #    #   #### 
#    #  #    #  #    #  #    #  #    #  #    #  #   #   #    #
#    #  #    #  #    #  #    #  #    #  #    #  #  #    #     
#    #  #    #  #    #  #    #  #    #  #    #  # #     #     
######  #####   #####   ######  #####   #####   ##      #     
#    #  #  #    #       #    #  #    #  #  #    ##      #  ###
#    #  #   #   #       #    #  #    #  #   #   # #     #    #
#    #  #   #   #       #    #  #    #  #   #   #  #    #    #
#    #  #    #  #       #    #  #    #  #    #  #   #   #   ##
#    #  #    #  #       #    #  #####   #    #  #    #   ### #\
            ";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap().trim());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>\
            ";
            let expected = "\
#   #  ###
#   #   # 
#   #   # 
#####   # 
#   #   # 
#   #   # 
#   #   # 
#   #  ###\
            ";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap().trim());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/10").trim();
            let expected = "10355";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>\
            ";
            let expected = "3";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
