extern crate base;

use base::{Part, Solver};
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day17)
}

struct Day17;

impl Solver for Day17 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let length = parse_input(input);
        let mut vec: Vec<u16> = Vec::with_capacity(2018);
        vec.push(0);

        let mut current_position = 0;
        for i in 1..2018 {
            let n = vec.len();
            let index_to_insert = ((current_position + length) % n) + 1;
            vec.insert(index_to_insert, i);
            current_position = index_to_insert;
        }
        Ok(vec[current_position + 1].to_string())
    }
}

fn parse_input(input: &str) -> usize {
    usize::from_str(input).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "3";
            let expected = "638";
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
