extern crate base;

use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day05)
}

struct Day05;

impl Solver for Day05 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let instructions = parse_input(input);
        match part {
            Part::One => Ok(steps_until_escape(&mut instructions.clone()).to_string()),
            _ => Err("part 2 not yet implemented".to_string()),
        }
    }
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn steps_until_escape(instructions: &mut [i64]) -> u64 {
    let mut idx = 0;
    let mut counter = 0;
    while idx >= 0 && idx < instructions.len() {
        let offset = instructions[idx];
        instructions[idx] += 1;
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
