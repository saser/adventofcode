use regex::Regex;

use base::{Part, Solver};

use std::collections::HashMap;
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day03)
}

struct Day03;

impl Solver for Day03 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let claims = input
            .lines()
            .map(Claim::from_str)
            .map(Result::unwrap)
            .collect::<Vec<Claim>>();
        let map = build_map(&claims);
        match part {
            Part::One => {
                let count = map
                    .values()
                    .filter(|point_claims| point_claims.len() > 1)
                    .count();
                Ok(count.to_string())
            }
            Part::Two => Err("part 2 not yet implemented".to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    dx: usize,
    dy: usize,
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
        let id = usize::from_str(&captures["id"]).unwrap();
        let x = usize::from_str(&captures["x"]).unwrap();
        let y = usize::from_str(&captures["y"]).unwrap();
        let dx = usize::from_str(&captures["dx"]).unwrap();
        let dy = usize::from_str(&captures["dy"]).unwrap();
        Ok(Self { id, x, y, dx, dy })
    }
}

fn build_map(claims: &[Claim]) -> HashMap<(usize, usize), Vec<&Claim>> {
    let mut map = HashMap::new();
    for claim in claims {
        for i in claim.x..claim.x + claim.dx {
            for j in claim.y..claim.y + claim.dy {
                let point_claims = map.entry((i, j)).or_insert(Vec::new());
                point_claims.push(claim);
            }
        }
    }
    map
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
                x: 1,
                y: 3,
                dx: 4,
                dy: 4,
            };
            assert_eq!(expected, Claim::from_str(input).unwrap());
        }

        #[test]
        fn multiple_digits() {
            let input = "#123 @ 19,443: 40x32";
            let expected = Claim {
                id: 123,
                x: 19,
                y: 443,
                dx: 40,
                dy: 32,
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
            let input = "\
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2\
            ";
            let expected = "3";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
