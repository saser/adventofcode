use std::io;
use std::str::FromStr;

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
    let made_recipes = parse_input(&input);
    let nr_recipes = made_recipes + 10;
    let mut scores = Vec::with_capacity(nr_recipes);
    scores.extend(&[3, 7]);
    let mut indices = [0, 1];
    match part {
        Part::One => {
            generate_scores(&mut scores, &mut indices, nr_recipes);
            let following_ten = scores.iter().skip(made_recipes).take(10);
            let s = following_ten
                .map(|score| score.to_string())
                .collect::<String>();
            Ok(s)
        }
        Part::Two => {
            let pattern = input
                .chars()
                .map(|c| c.to_string())
                .map(|s| usize::from_str(&s).unwrap())
                .collect::<Vec<usize>>();
            let recipes_before = generate_until_pattern(&mut scores, &mut indices, &pattern);
            Ok(recipes_before.to_string())
        }
    }
}

fn parse_input(input: &str) -> usize {
    usize::from_str(input).unwrap()
}

#[allow(dead_code)]
fn print_scores(scores: &[usize], indices: &[usize]) {
    for (i, score) in scores.iter().enumerate() {
        let surround = if i == indices[0] {
            ('(', ')')
        } else if i == indices[1] {
            ('[', ']')
        } else {
            (' ', ' ')
        };
        print!("{}{}{}", surround.0, score, surround.1);
    }
    println!();
}

fn add_scores_to(scores: &mut Vec<usize>, indices: &mut [usize]) {
    let sum = indices.iter().map(|&idx| scores[idx]).sum::<usize>();
    if sum >= 10 {
        scores.push(1);
    }
    scores.push(sum % 10);
    for idx in indices.iter_mut() {
        *idx += 1 + scores[*idx];
        *idx %= scores.len();
    }
}

fn generate_scores(scores: &mut Vec<usize>, indices: &mut [usize], nr_recipes: usize) {
    while scores.len() < nr_recipes {
        add_scores_to(scores, indices);
    }
}

fn generate_until_pattern(
    scores: &mut Vec<usize>,
    indices: &mut [usize],
    pattern: &[usize],
) -> usize {
    let n = pattern.len();
    while scores.len() < n {
        add_scores_to(scores, indices);
    }
    let mut found = false;
    let mut starting_from = 0;
    while !found {
        match contains_subslice_starting_from(&scores, pattern, starting_from) {
            (true, idx) => {
                found = true;
                starting_from = idx;
            }
            (false, idx) => {
                starting_from = idx + 1;
                add_scores_to(scores, indices);
            }
        };
    }
    starting_from
}

fn contains_subslice_starting_from<T>(
    slice: &[T],
    pattern: &[T],
    starting_from: usize,
) -> (bool, usize)
where
    T: PartialEq,
{
    let n = pattern.len();
    if let Some((idx, _subslice)) = slice[starting_from..]
        .windows(n)
        .enumerate()
        .find(|&(_i, subslice)| subslice == pattern)
    {
        (true, starting_from + idx)
    } else {
        (false, slice.len() - n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(example1, "9", "5158916779", part1);
        test!(example2, "5", "0124515891", part1);
        test!(example3, "18", "9251071085", part1);
        test!(example4, "2018", "5941429882", part1);
        test!(actual, file "../../../inputs/2018/14", "5371393113", part1);
    }

    mod part2 {
        use super::*;

        test!(example1, "51589", "9", part2);
        test!(example2, "01245", "5", part2);
        test!(example3, "92510", "18", part2);
        test!(example4, "59414", "2018", part2);
        test!(actual, file "../../../inputs/2018/14", "20286858", part2);
    }
}
