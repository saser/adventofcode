use std::str::FromStr;

use crate::base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day14)
}

struct Day14;

impl Solver for Day14 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let made_recipes = parse_input(input);
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

    mod part1 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../../inputs/2018/14").trim();
            let expected = "5371393113";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "9";
            let expected = "5158916779";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "5";
            let expected = "0124515891";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "18";
            let expected = "9251071085";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_4() {
            let solver = get_solver();
            let input = "2018";
            let expected = "5941429882";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../../inputs/2018/14").trim();
            let expected = "20286858";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "51589";
            let expected = "9";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "01245";
            let expected = "5";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "92510";
            let expected = "18";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_4() {
            let solver = get_solver();
            let input = "59414";
            let expected = "2018";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
