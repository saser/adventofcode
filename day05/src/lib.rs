extern crate base;

use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day05)
}

struct Day05;

impl Solver for Day05 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        Err("no solutions for day 5 implemented yet".to_string())
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
0
3
0
1
-3\
            ";
            let expected = "5";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }
}
