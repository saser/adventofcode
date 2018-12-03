use base::{Part, Solver};

use std::collections::HashMap;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day02)
}

struct Day02;

impl Solver for Day02 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let box_id_character_counts = input.lines().map(character_counts);
        let contains_tuples = box_id_character_counts.map(|counts| contains_any_two_three(&counts));
        let (total_twos, total_threes) = contains_tuples
            .fold((0, 0), |(acc_x, acc_y), (t_x, t_y)| {
                (acc_x + t_x, acc_y + t_y)
            });
        match part {
            Part::One => Ok((total_twos * total_threes).to_string()),
            Part::Two => Err("herp derp 2".to_string()),
        }
    }
}

fn character_counts(box_id: &str) -> HashMap<char, u64> {
    let mut counts = HashMap::new();
    for c in box_id.chars() {
        let counter = counts.entry(c).or_insert(0);
        *counter += 1;
    }
    counts
}

fn contains_any_two_three(counts: &HashMap<char, u64>) -> (i64, i64) {
    let mut contains = (0, 0);
    for (_c, &count) in counts {
        if count == 2 {
            contains.0 = 1;
        } else if count == 3 {
            contains.1 = 1;
        }
        if contains == (1, 1) {
            break;
        }
    }
    contains
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/02");
            let expected = "5880";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab\
            ";
            let expected = "12";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz\
            ";
            let expected = "fgij";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
