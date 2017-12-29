extern crate base;

use base::{Part, Solver};
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day18)
}

struct Day18;

impl Solver for Day18 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        Err("day 18 not yet implemented".to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Operand {
    Register(char),
    Literal(i64),
}

impl FromStr for Operand {
    type Err = String;

    fn from_str(s: &str) -> Result<Operand, String> {
        let first_char = s.chars().next().unwrap();
        if first_char.is_digit(10) || first_char == '-' {
            Ok(Operand::Literal(i64::from_str(s).unwrap()))
        } else {
            Ok(Operand::Register(char::from_str(s).unwrap()))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Instruction {
    Sound(Operand),
    Set(char, Operand),
    Add(char, Operand),
    Mul(char, Operand),
    Mod(char, Operand),
    Recover(Operand),
    Jgz(Operand, Operand),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Instruction, String> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() == 2 {
            let operand = Operand::from_str(parts[1])?;
            match parts[0] {
                "snd" => Ok(Instruction::Sound(operand)),
                "rcv" => Ok(Instruction::Recover(operand)),
                _ => Err(format!("invalid instruction: {}", s)),
            }
        } else if parts.len() == 3 {
            let op2 = Operand::from_str(parts[2])?;
            if parts[0] == "jgz" {
                let op1 = Operand::from_str(parts[1])?;
                Ok(Instruction::Jgz(op1, op2))
            } else {
                let op1 = char::from_str(parts[1]).map_err(|e| e.to_string())?;
                match parts[0] {
                    "set" => Ok(Instruction::Set(op1, op2)),
                    "add" => Ok(Instruction::Add(op1, op2)),
                    "mul" => Ok(Instruction::Mul(op1, op2)),
                    "mod" => Ok(Instruction::Mod(op1, op2)),
                    _ => Err(format!("invalid instruction: {}", s)),
                }
            }
        } else {
            Err(format!("invalid instruction: {}", s))
        }
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
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2\
            ";
            let expected = "4";
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
