extern crate base;

use base::{Part, Solver};
use std::collections::HashSet;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day04)
}

struct Day04;

impl Solver for Day04 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let passphrases = parse_input(input);
        let validator = match part {
            Part::One => contains_unique_passwords,
            Part::Two => contains_no_anagrams,
        };
        Ok(count_valid(validator, passphrases.clone()).to_string())
    }
}

fn parse_input(input: &str) -> Vec<Vec<String>> {
    input.lines()
        .map(|line| line.split_whitespace())
        .map(|iter| iter.map(String::from))
        .map(|iter| iter.collect())
        .collect()
}

fn count_valid(validator: fn(Vec<String>) -> bool, passphrases: Vec<Vec<String>>) -> usize {
    passphrases.iter()
        .filter(|&phrase| validator(phrase.clone()))
        .count()
}

fn contains_unique_passwords(passphrase: Vec<String>) -> bool {
    let words = passphrase.len();
    let set: HashSet<String> = passphrase.into_iter().collect();
    set.len() == words
}

fn contains_no_anagrams(passphrase: Vec<String>) -> bool {
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

    mod part2 {
        use super::*;

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "abcde fghij";
            let expected = "1";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "abcde xyz ecdab";
            let expected = "0";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "a ab abc abd abf abj";
            let expected = "1";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_4() {
            let solver = get_solver();
            let input = "iiii oiii ooii oooi oooo";
            let expected = "1";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_5() {
            let solver = get_solver();
            let input = "oiii ioii iioi iiio";
            let expected = "0";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
