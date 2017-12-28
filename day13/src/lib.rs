extern crate base;

use base::{Part, Solver};
use std::collections::HashMap;
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day13)
}

struct Day13;

impl Solver for Day13 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        Err("day 13 not yet implemented".to_string())
    }
}

fn parse_input(input: &str) -> HashMap<u64, u64> {
    input.lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> (u64, u64) {
    let parts: Vec<&str> = line.split(": ").collect();
    let layer = u64::from_str(parts[0]).unwrap();
    let depth = u64::from_str(parts[1]).unwrap();
    (layer, depth)
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
0: 3
1: 2
4: 4
6: 4\
            ";
            let expected = "24";
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
