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
        let mut programs = generate_programs(16);
        let iterations = match part {
            Part::One => 1,
            Part::Two => 1_000_000_000,
        };
        let moves = parse_input(input);
        let final_configuration = perform_moves_n(&mut programs, &moves, iterations);
        Ok(final_configuration)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
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
            let spin = usize::from_str(&captures["size"]).unwrap();
            Ok(Move::Spin(spin))
        } else if let Some(captures) = EXCHANGE_RE.captures(s) {
            let pos1 = usize::from_str(&captures["pos1"]).unwrap();
            let pos2 = usize::from_str(&captures["pos2"]).unwrap();
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
    input
        .split(',')
        .map(Move::from_str)
        .map(Result::unwrap)
        .collect()
}

fn generate_programs(count: usize) -> VecDeque<char> {
    if count > 16 {
        panic!("too high count: {}", count);
    }

    "abcdefghijklmnop".chars().take(count).collect()
}

fn programs_to_string(programs: &VecDeque<char>) -> String {
    programs
        .iter()
        .map(char::to_string)
        .collect::<Vec<String>>()
        .join("")
}

fn perform_move(programs: &mut VecDeque<char>, m: Move) {
    match m {
        Move::Spin(spin) => for _ in 0..spin {
            let program = programs.pop_back().unwrap();
            programs.push_front(program);
        },
        Move::Exchange(i, j) => programs.swap(i, j),
        Move::Partner(p1, p2) => {
            let (i, _) = programs
                .iter()
                .enumerate()
                .find(|&(_, &c)| c == p1)
                .unwrap();
            let (j, _) = programs
                .iter()
                .enumerate()
                .find(|&(_, &c)| c == p2)
                .unwrap();
            programs.swap(i, j);
        }
    }
}

fn perform_moves_n(programs: &mut VecDeque<char>, moves: &[Move], iterations: usize) -> String {
    let mut seen: Vec<String> = Vec::with_capacity(1_000_000);
    let mut final_configuration = programs_to_string(programs);
    seen.push(final_configuration.clone());
    for i in 1..iterations + 1 {
        for &m in moves {
            perform_move(programs, m);
        }

        let configuration = programs_to_string(programs);
        final_configuration = configuration.to_string();
        // Credits for the idea below goes to Magnus Hagmar.
        if &seen[0] == &final_configuration {
            final_configuration = seen[iterations % i].clone();
            break;
        }
        seen.push(configuration);
    }
    final_configuration
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
            perform_moves_n(&mut programs, &moves, 1);
            let expected = "baedc";
            assert_eq!(expected, programs_to_string(&programs));
        }
    }
}
