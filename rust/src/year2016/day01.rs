use lazy_static::lazy_static;
use regex::Regex;

use std::io;
use std::str::FromStr;

use crate::base::grid::*;

pub fn part1(r: &mut dyn io::Read) -> Result<String, String> {
    let mut input = String::new();
    r.read_to_string(&mut input).map_err(|e| e.to_string())?;
    let instructions = parse_input(&input);
    Ok(final_position(&instructions)
        .manhattan_distance()
        .to_string())
}

fn perform_instructions(instrs: &[(Turn, u64)]) -> Vec<Traveler> {
    instrs
        .iter()
        .scan(Traveler::default(), |state, &(turn, steps)| {
            state.turn(turn);
            state.step_n(steps);
            Some(*state)
        })
        .collect()
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
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(example1, "R2, L3", "5", part1);
        test!(example2, "R2, R2, R2", "2", part1);
        test!(example3, "R5, L5, R5, R3", "12", part1);
        test!(actual, file "../../../inputs/2016/01", "243", part1);
    }
}
