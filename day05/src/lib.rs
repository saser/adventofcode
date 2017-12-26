extern crate base;

use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day05)
}

struct Day05;

impl Solver for Day05 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let instructions = parse_input(input);
        let next_value = match part {
            Part::One => increase_by_one,
            Part::Two => decrement_if_three_or_more,
        };
        Ok(steps_until_escape(&mut instructions.clone(), next_value).to_string())
    }
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn increase_by_one(i: i64) -> i64 {
    i + 1
}

fn decrement_if_three_or_more(i: i64) -> i64 {
    if i >= 3 { i - 1 } else { i + 1 }
}

fn steps_until_escape(instructions: &mut [i64], next_value: fn(i64) -> i64) -> u64 {
    let mut idx = 0;
    let mut counter = 0;
    while idx >= 0 && idx < instructions.len() {
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

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
0
3
0
1
-3\
            ";
            let expected = "5";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
0
3
0
1
-3\
            ";
            let expected = "10";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
