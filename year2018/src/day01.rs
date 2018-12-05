use base::{Part, Solver};

use std::collections::HashSet;
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day01)
}

struct Day01;

impl Solver for Day01 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let changes = parse_input(input);
        match part {
            Part::One => Ok(final_frequency(&changes).to_string()),
            Part::Two => Ok(first_duplicate_frequency(&changes).to_string()),
        }
    }
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| i64::from_str(line).unwrap())
        .collect()
}

fn final_frequency(changes: &[i64]) -> i64 {
    changes.iter().sum()
}

fn first_duplicate_frequency(changes: &[i64]) -> i64 {
    let looped_frequencies = changes.iter().cycle().scan(0, |acc, &x| {
        *acc = *acc + x;
        Some(*acc)
    });
    let mut seen = HashSet::new();
    seen.insert(0);
    for freq in looped_frequencies {
        if seen.contains(&freq) {
            return freq;
        }
        seen.insert(freq);
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/01");
            let expected = "416";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "\
+1
-2
+3
+1\
            ";
            let expected = "3";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "\
+1
+1
+1\
            ";
            let expected = "3";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "\
+1
+1
-2\
            ";
            let expected = "0";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_4() {
            let solver = get_solver();
            let input = "\
-1
-2
-3\
            ";
            let expected = "-6";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/01");
            let expected = "56752";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "\
+1
-2
+3
+1\
            ";
            let expected = "2";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "\
+1
-1\
            ";
            let expected = "0";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "\
+3
+3
+4
-2
-4\
            ";
            let expected = "10";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_4() {
            let solver = get_solver();
            let input = "\
-6
+3
+8
+5
-6\
            ";
            let expected = "5";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_5() {
            let solver = get_solver();
            let input = "\
+7
+7
-2
-7
-4\
            ";
            let expected = "14";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
