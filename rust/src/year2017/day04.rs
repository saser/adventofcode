use std::collections::HashSet;
use std::io;

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
    let passphrases = parse_input(&input);
    let validator = match part {
        Part::One => contains_unique_passwords,
        Part::Two => contains_no_anagrams,
    };
    Ok(count_valid(validator, &passphrases).to_string())
}

fn parse_input(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|iter| iter.map(String::from))
        .map(|iter| iter.collect())
        .collect()
}

fn count_valid(validator: fn(&[String]) -> bool, passphrases: &[Vec<String>]) -> usize {
    passphrases
        .iter()
        .filter(|&phrase| validator(&phrase))
        .count()
}

fn contains_unique_passwords(passphrase: &[String]) -> bool {
    let words = passphrase.len();
    let set: HashSet<String> = passphrase.iter().cloned().collect();
    set.len() == words
}

fn contains_no_anagrams(passphrase: &[String]) -> bool {
    fn sort_string(s: &str) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        chars.into_iter().collect()
    }
    let words = passphrase.len();
    let set: HashSet<String> = passphrase.into_iter().map(|s| sort_string(&s)).collect();
    set.len() == words
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(example1, "aa bb cc dd ee", "1", part1);
        test!(example2, "aa bb cc dd aa", "0", part1);
        test!(example3, "aa bb cc dd aaa", "1", part1);
        test!(actual, file "../../../inputs/2017/04", "337", part1);
    }

    mod part2 {
        use super::*;

        test!(example1, "abcde fghij", "1", part2);
        test!(example2, "abcde xyz ecdab", "0", part2);
        test!(example3, "a ab abc abd abf abj", "1", part2);
        test!(example4, "iiii oiii ooii oooi oooo", "1", part2);
        test!(example5, "oiii ioii iioi iiio", "0", part2);
        test!(actual, file "../../../inputs/2017/04", "231", part2);
    }
}
