use regex::Regex;

use std::str::FromStr;

use base::grid::*;
use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day01)
}

struct Day01;

impl Solver for Day01 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let instrs = parse_input(input);
        match part {
            Part::One => Ok(final_position(&instrs).manhattan_distance().to_string()),
            Part::Two => Err("part two not implemented yet".to_string()),
        }
    }
}

fn perform_instructions(instrs: &[(Turn, u64)]) -> Vec<Traveler> {
    instrs
        .iter()
        .scan(Traveler::default(), |state, &(turn, steps)| {
            state.turn(turn);
            state.step_n(steps);
            Some(*state)
        }).collect()
}

fn final_position(instrs: &[(Turn, u64)]) -> Point {
    perform_instructions(instrs).last().unwrap().pos()
}

fn parse_input(input: &str) -> Vec<(Turn, u64)> {
    lazy_static! {
        static ref INSTR_RE: Regex = Regex::new(r"(?P<dir>[RL])(?P<steps>\d+)").unwrap();
    }
    input
        .split(", ")
        .map(|instr| {
            let captures = INSTR_RE.captures(instr).unwrap();
            let turn = Turn::from_str(&captures["dir"]).unwrap();
            let steps = u64::from_str(&captures["steps"]).unwrap();
            (turn, steps)
        }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "R2, L3";
            let parsed = parse_input(input);
            assert_eq!(
                vec![(Turn::Clockwise, 2), (Turn::CounterClockwise, 3)],
                parsed
            );
            let expected = "5";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "R2, R2, R2";
            let parsed = parse_input(input);
            assert_eq!(
                vec![
                    (Turn::Clockwise, 2),
                    (Turn::Clockwise, 2),
                    (Turn::Clockwise, 2)
                ],
                parsed
            );
            let expected = "2";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "R5, L5, R5, R3";
            let parsed = parse_input(input);
            assert_eq!(
                vec![
                    (Turn::Clockwise, 5),
                    (Turn::CounterClockwise, 5),
                    (Turn::Clockwise, 5),
                    (Turn::Clockwise, 3)
                ],
                parsed
            );
            let expected = "12";
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
