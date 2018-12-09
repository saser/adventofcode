use base::{Part, Solver};
use crate::day10;

use std::collections::{HashSet, VecDeque};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day14)
}

struct Day14;

impl Solver for Day14 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let strings = strings_to_hash(input);
        match part {
            Part::One => Ok(total_bits(&hash_all(&strings)).to_string()),
            Part::Two => {
                //let hashes = hash_all(&strings);
                Ok(count_groups(&hashes_to_binary(&hash_all(&strings))).to_string())
            }
        }
    }
}

fn strings_to_hash(input: &str) -> Vec<String> {
    (0..128).map(|i| format!("{}-{}", input, i)).collect()
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
    binary
        .chars()
        .map(|c| match c {
            '1' => true,
            '0' => false,
            _ => panic!("invalid digit in binary string: {}", c),
        }).collect()
}

fn bits_in_hash(hash: &str) -> usize {
    hash_to_binary(hash).chars().filter(|&c| c == '1').count()
}

fn hash_all(strings: &[String]) -> Vec<String> {
    strings
        .iter()
        .map(|string| day10::full_hash_str(&string))
        .collect()
}

fn total_bits(hashes: &[String]) -> usize {
    hashes.iter().map(|hash| bits_in_hash(&hash)).sum()
}

fn hashes_to_binary(hashes: &[String]) -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();
    let binary_vectors: Vec<Vec<bool>> = hashes
        .iter()
        .map(|hash| hash_to_binary(&hash))
        .map(|binstring| binary_to_vec(&binstring))
        .collect();
    for (i, binvec) in binary_vectors.as_slice().iter().enumerate() {
        for (j, &binval) in binvec.as_slice().iter().enumerate() {
            if binval {
                set.insert((i, j));
            }
        }
    }
    set
}

fn mark_group(
    visited: &mut HashSet<(usize, usize)>,
    set: &HashSet<(usize, usize)>,
    (start_x, start_y): (usize, usize),
) {
    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y));

    while let Some((x, y)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        let mut adjacent = vec![(x + 1, y), (x, y + 1)];
        if x > 0 {
            adjacent.push((x - 1, y));
        }

        if y > 0 {
            adjacent.push((x, y - 1));
        }

        for &pos in &adjacent {
            if set.contains(&pos) {
                queue.push_back(pos);
            }
        }
    }
}

fn count_groups(set: &HashSet<(usize, usize)>) -> u64 {
    let mut visited = HashSet::with_capacity(set.len());
    let mut counter = 0;
    for &pos in set.iter() {
        if visited.contains(&pos) {
            continue;
        }
        mark_group(&mut visited, set, pos);
        counter += 1;
    }
    counter
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
