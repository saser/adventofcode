use base::grid::Point;
use base::{Part, Solver};

use std::str::FromStr;

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day06)
}

struct Day06;

impl Solver for Day06 {
    fn solve(&self, part: Part, _input: &str) -> Result<String, String> {
        match part {
            Part::One => Err("day 06 part 1 not yet implemented".to_string()),
            Part::Two => Err("day 06 part 2 not yet implemented".to_string()),
        }
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(Point::from_str)
        .map(Result::unwrap)
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct BoundingBox {
    x: u64,
    y: u64,
    width: u64,
    height: u64,
}

impl BoundingBox {
    fn from_points(points: &[Point]) -> Self {
        let top_left = points.iter().fold(Point::origin(), |acc, point| Point {
            x: acc.x.min(point.x),
            y: acc.y.min(point.y),
        });
        let bottom_right = points.iter().fold(Point::origin(), |acc, point| Point {
            x: acc.x.max(point.x),
            y: acc.y.max(point.y),
        });
        BoundingBox {
            x: top_left.x as u64,
            y: top_left.y as u64,
            width: (bottom_right.x - top_left.x) as u64,
            height: (bottom_right.y - top_left.y) as u64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9\
            ";
            let expected = "17";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "put some input here";
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
