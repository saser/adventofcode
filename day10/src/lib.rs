extern crate base;

use base::{Part, Solver};
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day10)
}

struct Day10;

impl Solver for Day10 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let mut vector = initialize_vector(256);
        let lengths = parse_input(input);
        knot_hash(&mut vector, &lengths);
        let product = vector[0] * vector[1];
        Ok(product.to_string())
    }
}

fn parse_input(input: &str) -> Vec<u8> {
    input.split(',')
        .map(u8::from_str)
        .map(Result::unwrap)
        .collect()
}

fn initialize_vector(size: u64) -> Vec<u64> {
    (0..size).collect()
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

fn knot_hash<T: Copy>(slice: &mut [T], lengths: &[u8]) {
    let mut current = 0;
    let mut skip_size = 0;
    let len = slice.len();
    for &length in lengths {
        perform_knot(slice, current, length as usize);
        current = (current + length as usize + skip_size) % len;
        skip_size += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let input = "3,4,1,5";

            let lengths = parse_input(input);
            let mut vector = initialize_vector(5);
            knot_hash(&mut vector, &lengths);
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
