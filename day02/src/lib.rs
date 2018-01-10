extern crate base;

use base::{Part, Solver};
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day02)
}

struct Day02;

impl Solver for Day02 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
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

    mod part1 {
        use super::*;

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "5 1 9 5";
            let expected = "8";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "7 5 3";
            let expected = "4";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "2 4 6 8";
            let expected = "6";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_all() {
            let solver = get_solver();
            let input = "\
5 1 9 5
7 5 3
2 4 6 8\
            ";
            let expected = "18";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "5 9 2 8";
            let expected = "4";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "9 4 7 3";
            let expected = "3";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "3 8 6 5";
            let expected = "2";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_all() {
            let solver = get_solver();
            let input = "\
5 9 2 8
9 4 7 3
3 8 6 5\
            ";
            let expected = "9";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }

}
