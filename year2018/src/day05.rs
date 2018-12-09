use rayon::prelude::*;

use base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day05)
}

struct Day05;

impl Solver for Day05 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        match part {
            Part::One => {
                let after_reactions = fully_react(input.chars());
                Ok(after_reactions.len().to_string())
            }
            Part::Two => {
                let chars = (b'a'..=b'z').map(char::from).collect::<Vec<char>>();
                let best = chars
                    .par_iter()
                    .map(|&c| fully_react_without(input, c))
                    .map(|s| s.len())
                    .min()
                    .unwrap();
                Ok(best.to_string())
            }
        }
    }
}

fn fully_react(char_iter: impl Iterator<Item = char>) -> String {
    let mut chars = char_iter.map(Option::Some).collect::<Vec<Option<char>>>();

    remove_reactions(&mut chars);

    chars.iter().filter_map(|&opt_c| opt_c).collect()
}

fn fully_react_without(polymer: &str, unit: char) -> String {
    let filtered = polymer
        .chars()
        .filter(|c| c.to_ascii_uppercase() != unit.to_ascii_uppercase());
    fully_react(filtered)
}

fn remove_reactions(chars: &mut [Option<char>]) {
    let mut c1_index = 0;
    while c1_index < chars.len() {
        let c2_index = match find_next_forward(chars, c1_index + 1) {
            Some(i) => i,
            None => break,
        };
        let c1 = chars[c1_index].unwrap();
        let c2 = chars[c2_index].unwrap();
        if reacts(c1, c2) {
            chars[c1_index] = None;
            chars[c2_index] = None;
            c1_index = match find_next_backward(chars, c1_index) {
                Some(i) => i,
                None => match find_next_forward(chars, c2_index) {
                    Some(i) => i,
                    None => break,
                },
            };
        } else {
            c1_index = c2_index;
        }
    }
}

fn find_next_forward(chars: &[Option<char>], start: usize) -> Option<usize> {
    let mut index = None;
    for (i, opt_c) in chars.iter().enumerate().skip(start) {
        if opt_c.is_some() {
            index = Some(i);
            break;
        }
    }
    index
}

fn find_next_backward(chars: &[Option<char>], start: usize) -> Option<usize> {
    let mut index = None;
    let mut i = start;
    loop {
        if chars[i].is_some() {
            index = Some(i);
            break;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    index
}

fn reacts(c1: char, c2: char) -> bool {
    c1 != c2 && c1.to_ascii_uppercase() == c2.to_ascii_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/05").trim();
            let expected = "9686";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "aA";
            let expected = "0";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "abBA";
            let expected = "0";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "abAB";
            let expected = "4";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_4() {
            let solver = get_solver();
            let input = "aabAAB";
            let expected = "6";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_5() {
            let solver = get_solver();
            let input = "dabAcCaCBAcCcaDA";
            let expected = "10";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/05").trim();
            let expected = "5524";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "dabAcCaCBAcCcaDA";
            let expected = "4";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
