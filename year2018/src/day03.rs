use regex::Regex;

use base::{Part, Solver};

use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day03)
}

struct Day03;

impl Solver for Day03 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        Err("day 03 not yet implemented".to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Claim {
    id: isize,
    start_coords: (isize, isize),
    size: (isize, isize),
}

impl FromStr for Claim {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref CLAIM_RE: Regex =
                Regex::new(r"#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<dx>\d+)x(?P<dy>\d+)")
                    .unwrap();
        }
        let captures = CLAIM_RE.captures(s).unwrap();
        let id = isize::from_str(&captures["id"]).unwrap();
        let x = isize::from_str(&captures["x"]).unwrap();
        let y = isize::from_str(&captures["y"]).unwrap();
        let dx = isize::from_str(&captures["dx"]).unwrap();
        let dy = isize::from_str(&captures["dy"]).unwrap();
        Ok(Self {
            id: id,
            start_coords: (x, y),
            size: (dx, dy),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parsing {
        use super::*;

        #[test]
        fn single_digits() {
            let input = "#1 @ 1,3: 4x4";
            let expected = Claim {
                id: 1,
                start_coords: (1, 3),
                size: (4, 4),
            };
            assert_eq!(expected, Claim::from_str(input).unwrap());
        }

        #[test]
        fn multiple_digits() {
            let input = "#123 @ 19,443: 40x32";
            let expected = Claim {
                id: 123,
                start_coords: (19, 443),
                size: (40, 32),
            };
            assert_eq!(expected, Claim::from_str(input).unwrap());
        }
    }

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2\
            ";
            let expected = "4";
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
