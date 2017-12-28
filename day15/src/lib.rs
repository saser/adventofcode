extern crate base;

use base::{Part, Solver};
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day15)
}

struct Day15;

impl Solver for Day15 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let (start_a, start_b) = parse_input(input);
        let matching_pairs = (0..40_000_000)
            .scan((start_a, start_b), |pair, _| {
                *pair = next_values(*pair);
                Some(*pair)
            })
            //.inspect(|pair| println!("pair: {:?}", pair))
            .filter(|&pair| lowest_16_bits_matching(pair))
            .count();
        Ok(matching_pairs.to_string())
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

fn next_values((value_a, value_b): (u64, u64)) -> (u64, u64) {
    let next_value_a = (value_a * 16807) % 2147483647;
    let next_value_b = (value_b * 48271) % 2147483647;
    (next_value_a, next_value_b)
}

fn lowest_16_bits_matching((value_a, value_b): (u64, u64)) -> bool {
    value_a & 0xFFFF == value_b & 0xFFFF
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
            let input = "\
Generator A starts with 65
Generator B starts with 8921\
            ";
            let expected = "309";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
