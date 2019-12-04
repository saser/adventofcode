use std::collections::HashMap;
use std::io;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

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
        Part::Two => {
            let mut candidate_claims = map
                .values()
                .filter(|point_claims| point_claims.len() == 1)
                .map(|point_claims| point_claims[0]);
            let lonely_claim = candidate_claims
                .find(|&claim| {
                    claim.covered_points().iter().all(|point| {
                        let point_claims = &map[point];
                        point_claims.len() == 1 && point_claims[0] == claim
                    })
                })
                .unwrap();
            Ok(lonely_claim.id.to_string())
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

impl Claim {
    fn covered_points(&self) -> Vec<(usize, usize)> {
        let mut points = Vec::with_capacity(self.dx * self.dy);
        for i in self.x..self.x + self.dx {
            for j in self.y..self.y + self.dy {
                points.push((i, j));
            }
        }
        points
    }
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
                let point_claims = map.entry((i, j)).or_insert_with(Vec::new);
                point_claims.push(claim);
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

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

        test!(example, include_str!("testdata/day03/ex"), "4", part1);
        test!(
            actual,
            include_str!("../../../inputs/2018/03"),
            "113716",
            part1
        );
    }

    mod part2 {
        use super::*;

        test!(example, include_str!("testdata/day03/ex"), "3", part2);
        test!(
            actual,
            include_str!("../../../inputs/2018/03"),
            "742",
            part2
        );
    }
}
