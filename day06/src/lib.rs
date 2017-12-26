extern crate base;

use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day06)
}

struct Day06;

impl Solver for Day06 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let banks = parse_input(input);
        Err("day 6 not yet implemented".to_string())
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input.trim()
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "0 2 7 0";
            let expected = "5";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }
}
