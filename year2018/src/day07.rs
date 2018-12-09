use lazy_static::lazy_static;
use regex::Regex;

use std::collections::{HashMap, HashSet};

use base::{Part, Solver};

type Dependencies = HashMap<char, HashSet<char>>;
type Dependants = HashMap<char, HashSet<char>>;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day07)
}

struct Day07;

impl Solver for Day07 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let (dependencies, dependants) = parse_input(input);
        match part {
            Part::One => Err("day 07 part 1 not yet implemented".to_string()),
            Part::Two => Err("day 07 part 2 not yet implemented".to_string()),
        }
    }
}

fn parse_input(input: &str) -> (Dependencies, Dependants) {
    let mut dependencies = HashMap::new();
    let mut dependants = HashMap::new();
    input
        .lines()
        .map(parse_instruction)
        .for_each(|(dependency, dependant)| {
            dependencies
                .entry(dependant)
                .or_insert_with(HashSet::new)
                .insert(dependency);
            dependants
                .entry(dependency)
                .or_insert_with(HashSet::new)
                .insert(dependant);
        });
    (dependencies, dependants)
}

fn parse_instruction(instruction: &str) -> (char, char) {
    lazy_static! {
        static ref INSTR_RE: Regex = Regex::new(
            r"Step (?P<dependency>\w) must be finished before step (?P<dependant>\w) can begin."
        )
        .unwrap();
    }
    let captures = INSTR_RE.captures(instruction).unwrap();
    let dependency = captures["dependency"].chars().collect::<Vec<char>>()[0];
    let dependant = captures["dependant"].chars().collect::<Vec<char>>()[0];
    (dependency, dependant)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/07");
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.\
                ";
            let expected = "CABDFE";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/07");
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "put some input here";
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
