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
    let fun = match part {
        Part::One => min_max,
        Part::Two => divisors,
    };
    Ok(input
        .lines()
        .map(parse_line)
        .map(|v| fun(&v))
        .sum::<u32>()
        .to_string())
}

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(u32::from_str)
        .map(Result::unwrap)
        .collect()
}

fn min_max(nums: &[u32]) -> u32 {
    // Can also be solved using the `Iterator::max` and `Iterator::min` methods, but that's no fun.

    let mut min = nums[0];
    let mut max = nums[0];
    for &n in nums.iter() {
        if n < min {
            min = n;
        } else if n > max {
            max = n;
        }
    }
    max - min
}

fn divisors(nums: &[u32]) -> u32 {
    // This is a very bad, brute-force approach to the problem. But I can't think of any other way
    // to do it that is more optimal.

    for (i, &x) in nums.iter().enumerate() {
        for (j, &y) in nums.iter().enumerate() {
            if i == j {
                continue;
            };

            if x % y == 0 {
                return x / y;
            } else if y % x == 0 {
                return y / x;
            }
        }
    }

    // This is unreachable since the input is guaranteed to have exactly one pair of numbers that
    // divide each other, so the function is guaranteed to return in the above loops.
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(example1, "5 1 9 5", "8", part1);
        test!(example2, "7 5 3", "4", part1);
        test!(example3, "2 4 6 8", "6", part1);
        test!(
            example_all,
            include_str!("testdata/day02/p1ex"),
            "18",
            part1
        );
        test!(
            actual,
            include_str!("../../../inputs/2017/02"),
            "36766",
            part1
        );
    }

    mod part2 {
        use super::*;

        test!(example1, "5 9 2 8", "4", part2);
        test!(example2, "9 4 7 3", "3", part2);
        test!(example3, "3 8 6 5", "2", part2);
        test!(example_all, include_str!("testdata/day02/p2ex"), "9", part2);
        test!(
            actual,
            include_str!("../../../inputs/2017/02"),
            "261",
            part2
        );
    }
}
