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

fn parse_input(input: &str) -> Vec<i64> {
    input.lines()
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
