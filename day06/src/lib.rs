extern crate base;

use base::{Part, Solver};
use std::collections::HashSet;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day06)
}

struct Day06;

impl Solver for Day06 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let banks = parse_input(input);
        match part {
            Part::One => Ok(count_redistributions(&banks).to_string()),
            _ => Err("part 2 not yet implemented".to_string()),
        }
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input.trim()
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn count_redistributions(banks: &[u64]) -> u64 {
    let mut distributions: HashSet<Vec<u64>> = HashSet::new();
    let mut distribution = Vec::from(banks);
    distributions.insert(distribution.clone());

    let mut counter = 0;
    while counter < distributions.len() {
        distribution = redistribute(&distribution);
        distributions.insert(distribution.clone());
        counter += 1;
    }
    counter as u64
}

fn redistribute(banks: &[u64]) -> Vec<u64> {
    let mut vec = Vec::from(banks);

    let max_bank_index = find_max_index(&vec);
    let mut blocks_to_redistribute = vec[max_bank_index];
    vec[max_bank_index] = 0;

    let len = vec.len();
    let mut bank_to_increase_index = (max_bank_index + 1) % len;
    while blocks_to_redistribute > 0 {
        vec[bank_to_increase_index] += 1;
        bank_to_increase_index = (bank_to_increase_index + 1) % len;
        blocks_to_redistribute -= 1;
    }
    vec
}

fn find_max_index<T: Ord>(banks: &[T]) -> usize {
    banks.iter()
        .enumerate()
        .fold(0, |max_index, (index, bank)| if bank > &banks[max_index] {
            index
        } else {
            max_index
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "0 2 7 0";
            let expected = "5";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }
}
