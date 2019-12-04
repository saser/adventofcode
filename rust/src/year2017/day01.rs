use crate::base::Part;

use std::io;

pub fn part1(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::One)
}

pub fn part2(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::Two)
}

fn solve(r: &mut dyn io::Read, part: Part) -> Result<String, String> {
    let mut input = String::new();
    r.read_to_string(&mut input).map_err(|e| e.to_string())?;
    let digits = parse_input(input.trim());
    let offset = match part {
        Part::One => 1,
        Part::Two => digits.len() / 2,
    };
    Ok(sum_matching(&digits, offset).to_string())
}

fn parse_input(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn sum_matching(digits: &[u32], offset: usize) -> u32 {
    let n = digits.len();
    let mut sum = 0;
    for (idx, &d) in digits.iter().enumerate() {
        let u = d;
        let v_idx = (idx + offset) % n;
        let v = digits[v_idx];
        if u == v {
            sum += u;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(example1, "1122", "3", part1);
        test!(example2, "1111", "4", part1);
        test!(example3, "1234", "0", part1);
        test!(example4, "91212129", "9", part1);
        test!(actual, file "../../../inputs/2017/01", "1044", part1);
    }

    mod part2 {
        use super::*;

        test!(example1, "1212", "6", part2);
        test!(example2, "1221", "0", part2);
        test!(example3, "123425", "4", part2);
        test!(example4, "123123", "12", part2);
        test!(example5, "12131415", "4", part2);
        test!(actual, file "../../../inputs/2017/01", "1054", part2);
    }
}
