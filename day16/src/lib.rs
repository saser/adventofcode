extern crate base;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use base::{Part, Solver};
use regex::Regex;
use std::collections::VecDeque;
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day16)
}

struct Day16;

impl Solver for Day16 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        Err("day 16 not yet implemented".to_string())
    }
}

enum Move {
    Spin(u64),
    Exchange(u64, u64),
    Partner(char, char),
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Move, String> {
        lazy_static! {
                static ref SPIN_RE: Regex = Regex::new(r"s(?P<size>\d+)").unwrap();
                static ref EXCHANGE_RE: Regex = Regex::new(r"x(?P<pos1>\d+)/(?P<pos2>\d+)").unwrap();
                static ref PARTNER_RE: Regex = Regex::new(r"p(?P<name1>\w)/(?P<name2>\w)").unwrap();
        }
        if let Some(captures) = SPIN_RE.captures(s) {
            let spin = u64::from_str(&captures["size"]).unwrap();
            Ok(Move::Spin(spin))
        } else if let Some(captures) = EXCHANGE_RE.captures(s) {
            let pos1 = u64::from_str(&captures["pos1"]).unwrap();
            let pos2 = u64::from_str(&captures["pos2"]).unwrap();
            Ok(Move::Exchange(pos1, pos2))
        } else if let Some(captures) = PARTNER_RE.captures(s) {
            let name1 = char::from_str(&captures["name1"]).unwrap();
            let name2 = char::from_str(&captures["name2"]).unwrap();
            Ok(Move::Partner(name1, name2))
        } else {
            Err(format!("invalid move: {}", s))
        }
    }
}

fn parse_input(input: &str) -> Vec<Move> {
    input.split(',')
        .map(Move::from_str)
        .map(Result::unwrap)
        .collect()
}

fn generate_programs(count: usize) -> VecDeque<char> {
    "abcefghijklmnop"
        .chars()
        .take(count)
        .collect()
}

fn programs_to_string(programs: &VecDeque<char>) -> String {
    programs.iter()
        .map(char::to_string)
        .collect::<Vec<String>>()
        .join("")
}

fn perform_moves(programs: &mut VecDeque<char>, moves: &[Move]) {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let input = "s1,x3/4,pe/b";
            let mut programs = generate_programs(5);
            let moves = parse_input(input);
            perform_moves(&mut programs, &moves);
            let expected = "baedc";
            assert_eq!(expected, programs_to_string(&programs));
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
