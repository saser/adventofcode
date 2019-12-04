use std::io;
use std::str::FromStr;

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
    let length = parse_input(input.trim());
    match part {
        Part::One => {
            let (vec, final_position) = build_ring_buffer(2017, length);
            Ok(vec[final_position + 1].to_string())
        }
        Part::Two => Ok(value_after_zero(50_000_000, length).to_string()),
    }
}

fn parse_input(input: &str) -> usize {
    usize::from_str(input).unwrap()
}

fn build_ring_buffer(final_value: usize, length: usize) -> (Vec<usize>, usize) {
    let mut vec = Vec::with_capacity(final_value + 1);
    vec.push(0);
    let mut current_position = 0;
    for i in 1..=final_value {
        let index_to_insert = ((current_position + length) % i) + 1;
        vec.insert(index_to_insert, i);
        current_position = index_to_insert;
    }
    (vec, current_position)
}

fn value_after_zero(final_value: usize, length: usize) -> usize {
    let mut index_for_zero = 0;
    let mut value_after_zero = 0;
    let mut current_position = index_for_zero;
    for i in 1..final_value {
        let index_to_insert = ((current_position + length) % i) + 1;
        if index_to_insert <= index_for_zero {
            index_for_zero += 1;
        } else if index_to_insert == index_for_zero + 1 {
            value_after_zero = i;
        }
        current_position = index_to_insert;
    }
    value_after_zero
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(example, "3", "638", part1);
        test!(actual, file "../../../inputs/2017/17", "1311", part1);
    }

    mod part2 {
        use super::*;

        test!(actual, file "../../../inputs/2017/17", "39170601", part2);
    }
}
