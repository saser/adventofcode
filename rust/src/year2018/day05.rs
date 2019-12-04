use std::io;

use rayon::prelude::*;

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
    input = input.trim().to_string();
    match part {
        Part::One => {
            let after_reactions = fully_react(input.chars());
            Ok(after_reactions.len().to_string())
        }
        Part::Two => {
            let chars = (b'a'..=b'z').map(char::from).collect::<Vec<char>>();
            let best = chars
                .par_iter()
                .map(|&c| fully_react_without(&input, c))
                .map(|s| s.len())
                .min()
                .unwrap();
            Ok(best.to_string())
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
    use crate::test;

    mod part1 {
        use super::*;

        test!(example1, "aA", "0", part1);
        test!(example2, "abBA", "0", part1);
        test!(example3, "abAB", "4", part1);
        test!(example4, "aabAAB", "6", part1);
        test!(example5, "dabAcCaCBAcCcaDA", "10", part1);
        test!(
            actual,
            include_str!("../../../inputs/2018/05"),
            "9686",
            part1
        );
    }

    mod part2 {
        use super::*;

        test!(example, "dabAcCaCBAcCcaDA", "4", part2);
        test!(
            actual,
            include_str!("../../../inputs/2018/05"),
            "5524",
            part2
        );
    }
}
