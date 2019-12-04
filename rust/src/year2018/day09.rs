use std::collections::VecDeque;
use std::io;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::base::Part;

pub fn part1(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::One)
}

pub fn part2(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::Two)
}

fn solve(r: &mut dyn io::Read, part: Part) -> Result<String, String> {
    let mut input = String::new();
    r.read_to_string(&mut input).map_err(|e| e.to_string())?;
    let (players, last_marble) = parse_input(input.trim());
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
    use crate::test;

    fn make_input(players: i32, points: i32) -> String {
        format!(
            "{} players; last marble is worth {} points",
            players, points
        )
    }

    mod part1 {
        use super::*;

        test!(example1, &make_input(9, 25), "32", part1);
        test!(example2, &make_input(10, 1618), "8317", part1);
        test!(example3, &make_input(13, 7999), "146373", part1);
        test!(example4, &make_input(17, 1104), "2764", part1);
        test!(example5, &make_input(21, 6111), "54718", part1);
        test!(example6, &make_input(30, 5807), "37305", part1);
        test!(actual, file "../../../inputs/2018/09", "436720", part1);
    }

    mod part2 {
        use super::*;

        test!(actual, file "../../../inputs/2018/09", "3527845091", part2);
    }
}
