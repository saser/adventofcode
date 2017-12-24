extern crate base;

use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day04)
}

struct Day04;

impl Solver for Day04 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let passphrases = parse_input(input);
        match part {
            Part::One => Ok(count_valid(&passphrases).to_string()),
            Part::Two => Err("part 2 not done yet".to_string()),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<String>> {
    input.lines()
        .map(|line| line.split_whitespace())
        .map(|iter| iter.map(String::from))
        .map(|iter| iter.collect())
        .collect()
}

fn count_valid(passphrases: &[Vec<String>]) -> u32 {
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
            let input = "aa bb cc dd ee";
            let expected = "1";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "aa bb cc dd aa";
            let expected = "0";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "aa bb cc dd aaa";
            let expected = "1";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }
}
