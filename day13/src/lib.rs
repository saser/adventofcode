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
        let layers = parse_input(input);
        let total_severity: u64 = layers.iter()
            .map(|(&layer, &depth)| severity(layer, depth))
            .sum();
        Ok(total_severity.to_string())
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

fn detected_when_entering(picosecond: u64, depth: u64) -> bool {
    picosecond % ((depth - 1) * 2) == 0
}

fn severity(layer: u64, depth: u64) -> u64 {
    if detected_when_entering(layer, depth) {
        layer * depth
    } else {
        0
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
