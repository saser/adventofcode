use std::collections::HashMap;
use std::io;

use crate::base::Part;

pub fn part1(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::One)
}

pub fn part2(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::Two)
}

fn solve(r: &mut dyn io::Read, part: Part) -> Result<String, String> {
    let mut input = String::new();
    r.read_to_string(&mut input).map_err(|e| e.to_string())?;
    let (pots, map) = parse_input(&input);
    match part {
        Part::One => {
            let n = 20;
            let sum = sum_after_n_generations(n, &pots, &map);
            Ok(sum.to_string())
        }
        Part::Two => {
            let n = 50_000_000_000;
            let sum = sum_after_n_generations(n, &pots, &map);
            Ok(sum.to_string())
        }
    }
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut lines = input.lines();
    let initial_state_line = lines.next().unwrap();
    let parts = initial_state_line.split(": ").collect::<Vec<&str>>();
    let initial_bits = bitstring_to_bits(parts[1]);
    lines.next();
    let rest = lines.collect::<Vec<&str>>();
    let mut map = vec![0; 1 << 5];
    for &line in &rest {
        let (idx, bit) = parse_pattern(line);
        map[idx] = bit;
    }
    (initial_bits, map)
}

fn parse_pattern(line: &str) -> (usize, usize) {
    let parts = line.split(" => ").collect::<Vec<&str>>();
    let pattern = bitstring_to_bits(parts[0]);
    let output = bitstring_to_bits(parts[1]);
    (bits_to_usize(&pattern), bits_to_usize(&output))
}

fn bits_to_usize(bits: &[usize]) -> usize {
    bits.iter().fold(0, |x, &bit| (x << 1) | bit)
}

fn bitstring_to_bits(bitstring: &str) -> Vec<usize> {
    bitstring
        .chars()
        .map(|c| match c {
            '.' => 0,
            '#' => 1,
            _ => panic!("invalid char: {}", c),
        })
        .collect()
}

fn pad_with_zeroes(pots: &Vec<usize>, pad: usize) -> Vec<usize> {
    let mut padded = pots.clone();
    let pattern = vec![0; pad];
    while &padded[..pad] != &pattern[..] {
        padded.insert(0, 0);
    }
    while &padded[padded.len() - pad..] != &pattern[..] {
        padded.insert(padded.len(), 0);
    }
    padded
}

fn trim_zeroes(pots: &[usize]) -> Vec<usize> {
    let (first_one_idx, _first_one) = pots
        .iter()
        .enumerate()
        .find(|&(_i, &pot)| pot == 1)
        .unwrap();
    let (last_one_idx, _last_one) = pots
        .iter()
        .enumerate()
        .rfind(|&(_i, &pot)| pot == 1)
        .unwrap();
    pots[first_one_idx..=last_one_idx].to_vec()
}

fn pots_to_indices(pots: &[usize]) -> Vec<usize> {
    pots.windows(5).map(bits_to_usize).collect()
}

fn generation(pots: &Vec<usize>, map: &[usize], first_one: isize) -> (Vec<usize>, isize) {
    let pad = 5;
    let padded = pad_with_zeroes(pots, pad);
    let indices = pots_to_indices(&padded);
    let new_pots = indices.iter().map(|&idx| map[idx]).collect::<Vec<usize>>();
    let compare_pad = pad - 2;
    let (i, _new_bit) = new_pots
        .iter()
        .enumerate()
        .find(|&(_i, &new_bit)| new_bit == 1)
        .unwrap();
    let diff = i as isize - compare_pad as isize;
    let new_first_one = first_one + diff;
    let trimmed = trim_zeroes(&new_pots);
    (trimmed, new_first_one)
}

fn n_generations_from(
    n: usize,
    start_gen: usize,
    pots: &Vec<usize>,
    map: &[usize],
    first_one: isize,
) -> (Vec<usize>, isize) {
    let mut seen = HashMap::new();
    let mut current_pots = pots.clone();
    let mut current_first_one = first_one;
    seen.insert(current_pots.clone(), (start_gen, current_first_one));
    for gen in 1..=n {
        let (new_pots, new_first_one) = generation(&current_pots, map, current_first_one);
        if let Some((seen_gen, seen_first_one)) = seen.get(&new_pots) {
            let loop_length = gen - seen_gen;
            let number_of_loops = (n - seen_gen) as isize / loop_length as isize;
            let generations_left = (n - seen_gen) % loop_length;
            let diff = new_first_one - seen_first_one;
            return n_generations_from(
                generations_left,
                *seen_gen,
                &new_pots,
                map,
                seen_first_one + diff * number_of_loops,
            );
        }
        seen.insert(new_pots.clone(), (gen, new_first_one));
        current_pots = new_pots;
        current_first_one = new_first_one;
    }
    (current_pots, current_first_one)
}

fn sum_pot_indices(pots: &[usize], first_one: isize) -> isize {
    (first_one..)
        .zip(pots.iter())
        .map(|(idx, &pot)| idx * pot as isize)
        .sum()
}

fn sum_after_n_generations(n: usize, pots: &Vec<usize>, map: &[usize]) -> isize {
    let (new_pots, first_one) = n_generations_from(n, 0, pots, map, 0);
    sum_pot_indices(&new_pots, first_one)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(
            actual,
            include_str!("../../../inputs/2018/12"),
            "3221",
            part1
        );
        test!(example, include_str!("testdata/day12/ex"), "325", part1);
    }

    mod part2 {
        use super::*;

        test!(
            actual,
            include_str!("../../../inputs/2018/12"),
            "2600000001872",
            part2
        );
    }
}
