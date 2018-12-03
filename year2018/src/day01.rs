use base::{Part, Solver};

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
            Part::Two => Err("part 2 not yet implemented".to_string()),
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
    changes.iter().fold(0, |acc, &x| acc + x)
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
        fn example() {
            let solver = get_solver();
            let input = "put some input here";
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
