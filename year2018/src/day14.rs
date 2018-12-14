use std::str::FromStr;

use base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day14)
}

struct Day14;

impl Solver for Day14 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let made_recipes = parse_input(input);
        let scores = generate_scores(made_recipes + 10);
        match part {
            Part::One => {
                let following_ten = scores.iter().skip(made_recipes).take(10);
                let s = following_ten
                    .map(|score| score.to_string())
                    .collect::<String>();
                Ok(s)
            }
            Part::Two => Err("day 14 part 2 not yet implemented".to_string()),
        }
    }
}

fn parse_input(input: &str) -> usize {
    usize::from_str(input).unwrap()
}

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

fn generate_scores(nr_recipes: usize) -> Vec<usize> {
    let indices = &mut [0, 1];
    let mut scores = Vec::with_capacity(nr_recipes);
    scores.push(3);
    scores.push(7);
    while scores.len() < nr_recipes {
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
            let input = include_str!("../../inputs/2018/14").trim();
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
            let input = include_str!("../../inputs/2018/14").trim();
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "put some input here";
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
