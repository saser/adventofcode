use std::collections::HashMap;
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
    let ids = input.lines();
    match part {
        Part::One => {
            let box_id_character_counts = ids.map(character_counts);
            let contains_tuples =
                box_id_character_counts.map(|counts| contains_any_two_three(&counts));
            let (total_twos, total_threes) = contains_tuples
                .fold((0, 0), |(acc_x, acc_y), (t_x, t_y)| {
                    (acc_x + t_x, acc_y + t_y)
                });
            Ok((total_twos * total_threes).to_string())
        }
        Part::Two => {
            let ids = ids.collect::<Vec<&str>>();
            for (i, id1) in ids.iter().enumerate() {
                for id2 in &ids[i..] {
                    let (same_chars, count) = remove_differing_characters(id1, id2);
                    if count == 1 {
                        return Ok(same_chars);
                    }
                }
            }
            unreachable!()
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
    for &count in counts.values() {
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

fn remove_differing_characters(id1: &str, id2: &str) -> (String, u64) {
    id1.chars()
        .zip(id2.chars())
        .fold((String::new(), 0), |(s, count), (c1, c2)| {
            let mut new_s = s.clone();
            let mut new_count = count;
            if c1 == c2 {
                new_s.push(c1);
            } else {
                new_count += 1;
            }
            (new_s, new_count)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(example, include_str!("testdata/day02/p1ex"), "12", part1);
        test!(
            actual,
            include_str!("../../../inputs/2018/02"),
            "5880",
            part1
        );
    }

    mod part2 {
        use super::*;

        test!(example, include_str!("testdata/day02/p2ex"), "fgij", part2);
        test!(
            actual,
            include_str!("../../../inputs/2018/02"),
            "tiwcdpbseqhxryfmgkvjujvza",
            part2
        );
    }
}
