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
    let instructions = parse_input(&input);
    let next_value = match part {
        Part::One => increase_by_one,
        Part::Two => decrement_if_three_or_more,
    };
    Ok(steps_until_escape(&mut instructions.clone(), next_value).to_string())
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(str::parse).map(Result::unwrap).collect()
}

fn increase_by_one(i: i64) -> i64 {
    i + 1
}

fn decrement_if_three_or_more(i: i64) -> i64 {
    if i >= 3 {
        i - 1
    } else {
        i + 1
    }
}

fn steps_until_escape(instructions: &mut [i64], next_value: fn(i64) -> i64) -> u64 {
    let mut idx = 0;
    let mut counter = 0;
    while idx < instructions.len() {
        let offset = instructions[idx];
        instructions[idx] = next_value(offset);
        if offset < 0 {
            idx -= offset.abs() as usize;
        } else {
            idx += offset as usize;
        }
        counter += 1;
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(example, file "testdata/day05/ex", "5", part1);
        test!(actual, file "../../../inputs/2017/05", "358131", part1);
    }

    mod part2 {
        use super::*;

        test!(example, file "testdata/day05/ex", "10", part2);
        test!(actual, file "../../../inputs/2017/05", "25558839", part2);
    }
}
