extern crate base;

use base::{Part, Solver};
use std::collections::HashMap;
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day12)
}

struct Day12;

impl Solver for Day12 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        Err("day 12 not yet implemented".to_string())
    }
}

fn parse_input(input: &str) -> HashMap<u64, Vec<u64>> {
    input.lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let parts: Vec<&str> = line.split(" <-> ").collect();
    let program = u64::from_str(parts[0]).unwrap();
    let connected: Vec<u64> = parts[1]
        .split(", ")
        .map(u64::from_str)
        .map(Result::unwrap)
        .collect();
    (program, connected)
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
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5\
            ";
            let expected = "6";
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
