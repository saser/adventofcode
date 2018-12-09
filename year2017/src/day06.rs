use base::{Part, Solver};
use std::collections::HashMap;

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day06)
}

struct Day06;

impl Solver for Day06 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let banks = parse_input(input);
        let (redistributions, loop_size) = count_redistributions(&banks);
        let answer = match part {
            Part::One => redistributions,
            Part::Two => loop_size,
        };
        Ok(answer.to_string())
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn count_redistributions(banks: &[u64]) -> (u64, u64) {
    let mut distributions: HashMap<Vec<u64>, u64> = HashMap::new();

    let mut counter = 0;
    let mut distribution = Vec::from(banks);
    distributions.insert(distribution.clone(), counter as u64);

    while counter < distributions.len() {
        distribution = redistribute(&distribution);
        counter += 1;
        distributions
            .entry(distribution.clone())
            .or_insert(counter as u64);
    }
    let first_distribution_in_loop = distributions.get(&distribution).unwrap();

    (counter as u64, counter as u64 - first_distribution_in_loop)
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
    banks
        .iter()
        .enumerate()
        .fold(0, |max_index, (index, bank)| {
            if bank > &banks[max_index] {
                index
            } else {
                max_index
            }
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

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "0 2 7 0";
            let expected = "4";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
