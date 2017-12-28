extern crate base;

use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day15)
}

struct Day15;

impl Solver for Day15 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        Err("day 15 not yet implemented".to_string())
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
            let input = "put some input here";
            let expected = "expected output";
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
