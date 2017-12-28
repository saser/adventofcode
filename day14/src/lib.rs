extern crate base;
extern crate day10;

use base::{Part, Solver};
use base::grid::Grid;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day14)
}

struct Day14;

impl Solver for Day14 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let strings = strings_to_hash(input);
        match part {
            Part::One => Ok(total_bits(&hash_all(&strings)).to_string()),
            Part::Two => Err("part 2 not implemented yet".to_string()),
        }
    }
}

fn strings_to_hash(input: &str) -> Vec<String> {
    (0..128)
        .map(|i| format!("{}-{}", input, i))
        .collect()
}

fn hex_digit_to_binary(digit: char) -> String {
    format!("{:04b}", digit.to_digit(16).unwrap())
}

fn hash_to_binary(hash: &str) -> String {
    hash.chars()
        .map(hex_digit_to_binary)
        .collect::<Vec<String>>()
        .join("")
}

fn binary_to_vec(binary: &str) -> Vec<bool> {
    binary.chars()
        .map(|c| match c {
            '1' => true,
            '0' => false,
            _ => panic!("invalid digit in binary string: {}", c),
        })
        .collect()
}

fn bits_in_hash(hash: &str) -> usize {
    hash_to_binary(hash)
        .chars()
        .filter(|&c| c == '1')
        .count()
}

fn hash_all(strings: &[String]) -> Vec<String> {
    strings.iter()
        .map(|string| day10::full_hash_str(&string))
        .collect()
}

fn total_bits(hashes: &[String]) -> usize {
    hashes.iter()
        .map(|hash| bits_in_hash(&hash))
        .sum()
}

fn hashes_to_binary_grid(hashes: &[String]) -> Grid<bool> {
    let mut grid: Grid<bool> = Grid::new(hashes.len(), hashes.len());
    let binary_vectors: Vec<Vec<bool>> = hashes.iter()
        .map(|hash| hash_to_binary(&hash))
        .map(|binstring| binary_to_vec(&binstring))
        .collect();
    for (i, binvec) in binary_vectors.as_slice()
        .iter()
        .enumerate() {
        for (j, &binval) in binvec.as_slice()
            .iter()
            .enumerate() {
            grid[(i, j)] = binval;
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "flqrgnkx";
            let expected = "8108";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "flqrgnkx";
            let expected = "1242";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
