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
        let values_to_insert = 2017;
        let mut vec: Vec<u32> = Vec::with_capacity(values_to_insert + 1);
        vec.push(0);

        let mut current_position = 0;
        for i in 1..values_to_insert + 1 {
            let index_to_insert = ((current_position + length) % i) + 1;
            vec.insert(index_to_insert, i as u32);
            current_position = index_to_insert;
        }
        match part {
            Part::One => Ok(vec[current_position + 1].to_string()),
            Part::Two => Err("part 2 not implemented yet".to_string()),
        }
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
}
