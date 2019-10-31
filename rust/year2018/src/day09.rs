use lazy_static::lazy_static;
use regex::Regex;

use std::collections::VecDeque;
use std::str::FromStr;

use base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day09)
}

struct Day09;

impl Solver for Day09 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let (players, last_marble) = parse_input(input);
        match part {
            Part::One => {
                let scores = play_game(players, last_marble);
                let winner = scores.iter().max().unwrap();
                Ok(winner.to_string())
            }
            Part::Two => {
                let scores = play_game(players, last_marble * 100);
                let winner = scores.iter().max().unwrap();
                Ok(winner.to_string())
            }
        }
    }
}

fn parse_input(input: &str) -> (usize, usize) {
    lazy_static! {
        static ref INPUT_RE: Regex = Regex::new(
            r"(?P<players>\d+) players; last marble is worth (?P<last_marble>\d+) points"
        )
        .unwrap();
    }
    let captures = INPUT_RE.captures(input).unwrap();
    let players = usize::from_str(&captures["players"]).unwrap();
    let last_marble = usize::from_str(&captures["last_marble"]).unwrap();
    (players, last_marble)
}

fn play_game(players: usize, last_marble: usize) -> Vec<usize> {
    let mut scores = vec![0; players];
    let mut ring = VecDeque::new();
    ring.push_front(0);
    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            for _ in 0..7 {
                let popped = ring.pop_back().unwrap();
                ring.push_front(popped);
            }
            scores[marble % players] += marble + ring.pop_front().unwrap();
        } else {
            for _ in 0..2 {
                let popped = ring.pop_front().unwrap();
                ring.push_back(popped);
            }
            ring.push_front(marble);
        }
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/09").trim();
            let expected = "436720";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "9 players; last marble is worth 25 points";
            let expected = "32";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "10 players; last marble is worth 1618 points";
            let expected = "8317";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "13 players; last marble is worth 7999 points";
            let expected = "146373";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_4() {
            let solver = get_solver();
            let input = "17 players; last marble is worth 1104 points";
            let expected = "2764";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_5() {
            let solver = get_solver();
            let input = "21 players; last marble is worth 6111 points";
            let expected = "54718";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_6() {
            let solver = get_solver();
            let input = "30 players; last marble is worth 5807 points";
            let expected = "37305";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/09").trim();
            let expected = "3527845091";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
