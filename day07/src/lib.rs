extern crate base;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use base::{Part, Solver};
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day07)
}

struct Day07;

impl Solver for Day07 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let programs = parse_input(input);
        let tower = construct_tower(&programs);
        let bottom_program = find_bottom_program(&tower);
        if part == Part::One {
            Ok(bottom_program.name.clone())
        } else {
            Err("part 2 not yet implemented".to_string())
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Program {
    name: String,
    weight: u64,
    holding_up: Option<Vec<String>>,
    held_up_by: Option<String>,
}

impl FromStr for Program {
    type Err = String;
    fn from_str(s: &str) -> Result<Program, String> {
        lazy_static! {
            static ref NAME_AND_WEIGHT: Regex = Regex::new(r"(?P<name>\w+) \((?P<weight>\d+)\)").unwrap();
        }
        let parts: Vec<&str> = s.split(" -> ").collect();
        let (name_and_weight, programs) = (parts[0],
                                           if parts.len() == 2 {
                                               Some(parts[1])
                                           } else {
                                               None
                                           });
        let name_and_weight_caps = NAME_AND_WEIGHT.captures(name_and_weight).unwrap();
        let name = name_and_weight_caps["name"].to_string();
        let weight: u64 = name_and_weight_caps["weight"].parse().unwrap();
        let holding_up =
            programs.map(|program_str| program_str.split(", ").map(String::from).collect());

        Ok(Program {
            name: name,
            weight: weight,
            holding_up: holding_up,
            held_up_by: None,
        })
    }
}

fn parse_input(input: &str) -> HashMap<String, Program> {
    input.lines()
        .map(Program::from_str)
        .map(Result::unwrap)
        .map(|prog| (prog.name.clone(), prog))
        .collect()
}

fn construct_tower(programs: &HashMap<String, Program>) -> HashMap<String, Program> {
    let mut tower = programs.clone();
    let progs_holding_up: Vec<Program> = tower.values()
        .cloned()
        .filter(|prog| prog.holding_up.is_some())
        .collect();
    for holding_prog in progs_holding_up {
        for prog in holding_prog.holding_up.unwrap() {
            tower.get_mut(&prog).unwrap().held_up_by = Some(holding_prog.name.clone());
        }
    }
    tower
}

fn find_bottom_program(tower: &HashMap<String, Program>) -> Program {
    tower.values().find(|prog| prog.held_up_by.is_none()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_tests {
        use super::*;

        #[test]
        fn program_not_holding_up() {
            let input = "pbga (66)";
            let program = Program::from_str(input).unwrap();
            assert_eq!("pbga", &program.name);
            assert_eq!(66, program.weight);
        }

        #[test]
        fn program_holding_up() {
            let input = "fwft (72) -> ktlj, cntj, xhth";
            let program = Program::from_str(input).unwrap();
            assert_eq!("fwft", &program.name);
            assert_eq!(72, program.weight);
            assert_eq!(&["ktlj", "cntj", "xhth"], &program.holding_up.unwrap()[..]);
        }
    }

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)\
            ";
            let expected = "tknk";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)\
            ";
            let expected = "60";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
