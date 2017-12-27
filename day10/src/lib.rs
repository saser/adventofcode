extern crate base;

use base::{Part, Solver};
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day10)
}

struct Day10;

impl Solver for Day10 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let mut vector = initialize_vector();
        let lengths = parse_input_as_lengths(input);
        match part {
            Part::One => Ok(hash_and_multiply(&mut vector, &lengths).to_string()),
            Part::Two => Err("not yet implemented".to_string()),
        }
    }
}

fn parse_input_as_lengths(input: &str) -> Vec<u8> {
    input.split(',')
        .map(u8::from_str)
        .map(Result::unwrap)
        .collect()
}

fn initialize_vector() -> Vec<u8> {
    let mut i = 0;
    let highest_value = std::u8::MAX;
    let mut v = Vec::with_capacity(highest_value as usize);
    while i <= highest_value {
        v.push(i);
        i += 1;
    }
    v
}

fn indices_wrapping(slice_length: usize, start: usize, length: usize) -> Vec<usize> {
    (start..start + length).map(|i| i % slice_length).collect()
}

fn reverse_by_indices<T: Copy>(slice: &mut [T], indices: &[usize]) {
    if indices.len() < 1 {
        return;
    }

    let mut i = 0;
    let mut j = indices.len() - 1;
    while i < j {
        let early = slice[indices[i]];
        let late = slice[indices[j]];
        slice[indices[i]] = late;
        slice[indices[j]] = early;

        i += 1;
        j -= 1;
    }
}

fn perform_knot<T: Copy>(slice: &mut [T], start: usize, length: usize) {
    let indices = indices_wrapping(slice.len(), start, length);
    reverse_by_indices(slice, &indices);
}

fn knot_hash<T: Copy>(slice: &mut [T],
                      lengths: &[u8],
                      mut current: usize,
                      mut skip_size: usize)
                      -> (usize, usize) {
    let len = slice.len();
    for &length in lengths {
        perform_knot(slice, current, length as usize);
        current = (current + length as usize + skip_size) % len;
        skip_size += 1;
    }
    (current, skip_size)
}

fn hash_and_multiply(slice: &mut [u8], lengths: &[u8]) -> u64 {
    knot_hash(slice, lengths, 0, 0);
    slice[0] as u64 * slice[1] as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let input = "3,4,1,5";

            let lengths = parse_input_as_lengths(input);
            let mut vector = vec![0, 1, 2, 3, 4];
            knot_hash(&mut vector, &lengths, 0, 0);
            let product = vector[0] * vector[1];

            assert_eq!(12, product);
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
