extern crate base;

use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day03)
}

struct Day03;

impl Solver for Day03 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        // We need to trim the input, in case it contains a '\n' at the end.
        let number = parse_input(input.trim());
        match part {
            Part::One => Ok(distance_to_center(number).to_string()),
            Part::Two => Err("part 2 not done yet".to_string()),
        }
    }
}

fn distance_to_center(number: usize) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "1";
            let expected = "0";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "12";
            let expected = "3";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "23";
            let expected = "2";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_4() {
            let solver = get_solver();
            let input = "1024";
            let expected = "31";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }
}
