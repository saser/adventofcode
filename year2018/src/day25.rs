use std::str::FromStr;

use base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day25)
}

struct Day25;

impl Solver for Day25 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let points = parse_input(input);
        match part {
            Part::One => Err("day 25 part 1 not yet implemented".to_string()),
            Part::Two => Err("day 25 part 2 not yet implemented".to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point4D {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Point4D {
    fn manhattan_distance_to(&self, other: Point4D) -> u64 {
        ((self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.w - other.w).abs()) as u64
    }
}

impl FromStr for Point4D {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<&str>>();
        let x = i64::from_str(parts[0]).unwrap();
        let y = i64::from_str(parts[1]).unwrap();
        let z = i64::from_str(parts[2]).unwrap();
        let w = i64::from_str(parts[3]).unwrap();
        Ok(Point4D { x, y, z, w })
    }
}

fn parse_input(input: &str) -> Vec<Point4D> {
    input
        .lines()
        .map(Point4D::from_str)
        .map(Result::unwrap)
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
            let input = include_str!("../../inputs/2018/25").trim();
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "\
0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0\
            ";
            let expected = "2";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "\
-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0\
            ";
            let expected = "4";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "\
1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2\
            ";
            let expected = "3";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_4() {
            let solver = get_solver();
            let input = "\
1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2\
            ";
            let expected = "8";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/25").trim();
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "put some input here";
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
