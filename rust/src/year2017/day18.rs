use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use crate::base::Part;

pub fn part1(r: &mut dyn Read) -> Result<String, String> {
    solve(r, Part::One)
}

pub fn part2(r: &mut dyn Read) -> Result<String, String> {
    solve(r, Part::Two)
}

fn solve(r: &mut dyn Read, part: Part) -> Result<String, String> {
    let instructions = BufReader::new(r)
        .lines()
        .map(|line| line.expect("input line read failed"))
        .map(|ref line| Instruction::from_str(line))
        .map(|i| i.expect("instruction parsing failed"))
        .collect::<Vec<Instruction>>();
    Err("not implemented yet".to_string())
}

enum Instruction {
    Snd(Operand),
    Set(Operand, Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Mod(Operand, Operand),
    Rcv(Operand),
    Jgz(Operand, Operand),
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<&str>>();
        if parts.len() == 0 {
            return Err("empty instruction".to_string());
        }
        if parts.len() == 1 {
            return Err(format!("missing operands: {}", s));
        }
        let op = parts[0];
        let operand1 = Operand::from_str(parts[1])?;
        match op {
            "snd" => Ok(Instruction::Snd(operand1)),
            "rcv" => Ok(Instruction::Rcv(operand1)),
            "set" | "add" | "mul" | "mod" | "jgz" if parts.len() == 3 => {
                let operand2 = Operand::from_str(parts[2])?;
                Ok(match op {
                    "set" => Instruction::Set(operand1, operand2),
                    "add" => Instruction::Add(operand1, operand2),
                    "mul" => Instruction::Mul(operand1, operand2),
                    "mod" => Instruction::Mod(operand1, operand2),
                    "jgz" => Instruction::Jgz(operand1, operand2),
                    _ => unreachable!(),
                })
            }
            _ => Err(format!("invalid op: {}", op)),
        }
    }
}

enum Operand {
    Integer(i64),
    Register(char),
}

impl FromStr for Operand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(i) = i64::from_str(s) {
            return Ok(Operand::Integer(i));
        }
        if let Some(c) = s.chars().next() {
            return Ok(Operand::Register(c));
        }
        Err(format!("invalid operand: {}", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(example, file "testdata/day18/ex", "4", part1);
        test!(actual, file "../../../inputs/2017/18", "3188", part1);
    }

    // mod part2 {
    //     use super::*;

    //     test!(example, "", "", part2);
    //     test!(actual, file "../../../inputs/2017/18", "", part2);
    // }
}
