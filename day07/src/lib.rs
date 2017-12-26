extern crate base;

use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day07)
}

struct Day07;

impl Solver for Day07 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        Err("day XX not yet implemented".to_string())
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
