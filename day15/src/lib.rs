extern crate base;

use base::{Part, Solver};
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day15)
}

struct Day15;

impl Solver for Day15 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        Err("day 15 not yet implemented".to_string())
    }
}

fn parse_input(input: &str) -> (u64, u64) {
    let lines: Vec<&str> = input.lines().collect();
    (parse_line(&lines[0]), parse_line(&lines[1]))
}

fn parse_line(line: &str) -> u64 {
    let parts: Vec<&str> = line.split(' ').collect();
    u64::from_str(parts[parts.len() - 1]).unwrap()
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
Generator A starts with 65
Generator B starts with 8921\
            ";
            let expected = "588";
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
