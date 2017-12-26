extern crate base;

use base::{Part, Solver};
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day07)
}

struct Day07;

impl Solver for Day07 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        Err("day XX not yet implemented".to_string())
    }
}

struct Program {
    name: String,
    weight: u64,
    holding_up: Vec<String>,
    held_up_by: Option<String>,
}

impl FromStr for Program {
    type Err = String;
    fn from_str(s: &str) -> Result<Program, String> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            // Add example here.
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            // Add example here.
        }
    }
}
