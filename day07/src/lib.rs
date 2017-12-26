extern crate base;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use base::{Part, Solver};
use regex::Regex;
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day07)
}

struct Day07;

impl Solver for Day07 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        Err("day XX not yet implemented".to_string())
    }
}

struct Program {
    name: String,
    weight: u64,
    holding_up: Vec<String>,
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
        let holding_up: Vec<String> = match programs {
            None => Vec::new(),
            Some(program_str) => program_str.split(", ").map(String::from).collect(),
        };

        Ok(Program {
            name: name,
            weight: weight,
            holding_up: holding_up,
            held_up_by: None,
        })
    }
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
            assert_eq!(&["ktlj", "cntj", "xhth"], &program.holding_up[..]);
        }
    }

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            // Add example here.
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            // Add example here.
        }
    }
}
