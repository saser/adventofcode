extern crate base;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use base::{Part, Solver};
use regex::Regex;
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day08)
}

struct Day08;

impl Solver for Day08 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        Err("day 08 not yet implemented".to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Operation {
    Inc(i64),
    Dec(i64),
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Operation, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<op>inc|dec) (?P<value>-?\d+)").unwrap();
        }
        let captures = RE.captures(s).unwrap();
        let value = i64::from_str(&captures["value"]).unwrap();
        match &captures["op"] {
            "inc" => Ok(Operation::Inc(value)),
            "dec" => Ok(Operation::Dec(value)),
            _ => Err(format!("invalid operation: {}", s)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Comparison {
    Lt,
    Gt,
    EqLt,
    EqGt,
    Eq,
    Neq,
}

impl FromStr for Comparison {
    type Err = String;

    fn from_str(s: &str) -> Result<Comparison, String> {
        match s {
            "<" => Ok(Comparison::Lt),
            ">" => Ok(Comparison::Gt),
            "<=" => Ok(Comparison::EqLt),
            ">=" => Ok(Comparison::EqGt),
            "==" => Ok(Comparison::Eq),
            "!=" => Ok(Comparison::Neq),
            _ => Err(format!("invalid comparison: {}", s)),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Condition {
    register: String,
    cmp: Comparison,
    value: i64,
}

impl FromStr for Condition {
    type Err = String;

    fn from_str(s: &str) -> Result<Condition, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<register>) (?P<cmp><|>|<=|>=|==|!=) (?P<value>-?\d+)").unwrap();
        }
        let captures = RE.captures(s).unwrap();
        let register = captures["register"].to_string();
        let cmp = Comparison::from_str(&captures["cmp"]).unwrap();
        let value = i64::from_str(&captures["value"]).unwrap();
        Ok(Condition {
            register: register,
            cmp: cmp,
            value: value,
        })
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Instruction {
    register: String,
    op: Operation,
    cond: Condition,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Instruction, String> {
        let parts: Vec<&str> = s.split(" if ").collect();
        let before_condition: Vec<&str> = parts[0].splitn(2, ' ').collect();
        let register = before_condition[0].to_string();
        let op = Operation::from_str(before_condition[1])?;
        let cond = Condition::from_str(parts[1])?;
        Ok(Instruction {
            register: register,
            op: op,
            cond: cond,
        })
    }
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
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10\
            ";
            let expected = "1";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
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
