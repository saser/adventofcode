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
        match part {
            Part::One => {
                let lengths = parse_input_as_lengths(input);
                Ok(hash_and_multiply(&mut vector, &lengths).to_string())
            }
            Part::Two => {
                let lengths = parse_input_as_bytes(input);
                Ok(full_hash(&mut vector, &lengths))
            }
        }
    }
}

fn parse_input_as_lengths(input: &str) -> Vec<u8> {
    input.split(',')
        .map(u8::from_str)
        .map(Result::unwrap)
        .collect()
}

fn parse_input_as_bytes(input: &str) -> Vec<u8> {
    Vec::from(input.as_bytes())
}

fn initialize_vector() -> Vec<u8> {
    let mut i = 0;
    let highest_value = std::u8::MAX;
    let mut v = Vec::with_capacity(highest_value as usize + 1);
    while i < highest_value {
        v.push(i);
        i += 1;
    }
    v.push(highest_value);
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

fn knot_hash_n<T: Copy>(slice: &mut [T], lengths: &[u8], n: u64) {
    let mut current = 0;
    let mut skip_size = 0;
    for _ in 0..n {
        let (new_current, new_skip_size) = knot_hash(slice, lengths, current, skip_size);
        current = new_current;
        skip_size = new_skip_size;
    }
}

fn hash_and_multiply(slice: &mut [u8], lengths: &[u8]) -> u64 {
    knot_hash(slice, lengths, 0, 0);
    slice[0] as u64 * slice[1] as u64
}

fn add_suffix(lengths: &[u8]) -> Vec<u8> {
    let mut vec = Vec::from(lengths);
    vec.append(&mut vec![17, 31, 73, 47, 23]);
    vec
}

fn byte_as_hexadecimal(byte: u8) -> String {
    format!("{:02x}", byte)
}

fn full_hash(slice: &mut [u8], lengths: &[u8]) -> String {
    let lengths_suffixed = add_suffix(lengths);
    knot_hash_n(slice, &lengths_suffixed, 64);
    slice.chunks(16)
        .map(|chunk| chunk.iter())
        .map(|iter| iter.fold(0, |acc, x| acc ^ x))
        .map(byte_as_hexadecimal)
        .collect::<Vec<String>>()
        .join("")
    //hash
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
        fn example_1() {
            let solver = get_solver();
            let input = "";
            let expected = "a2582a3a0e66e6e86e3812dcb672a272";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "AoC 2017";
            let expected = "33efeb34ea91902bb2f59c9920caa6cd";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "1,2,3";
            let expected = "3efbe78a8d82f29979031a4aa0b16a9d";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_4() {
            let solver = get_solver();
            let input = "1,2,4";
            let expected = "63960835bcdc130f0b66d7ff4f6a5a8e";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
