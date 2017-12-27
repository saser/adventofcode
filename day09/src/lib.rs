extern crate base;

use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day09)
}

struct Day09;

impl Solver for Day09 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let tokens = parse_tokens(input)?;
        let score = process_tokens(&tokens);
        match part {
            Part::One => Ok(score.to_string()),
            Part::Two => Err("part 2 not implemented yet".to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Token {
    StartGroup,
    EndGroup,
    StartGarbage,
    EndGarbage,
    Garbage(char),
    Ignore(char),
    Separator,
}

fn parse_tokens(s: &str) -> Result<Vec<Token>, String> {
    let mut chars = s.chars();
    let mut tokens = Vec::with_capacity(s.len());
    let mut is_garbage = false;

    while let Some(c) = chars.next() {
        let token = if is_garbage {
            match c {
                '>' => Token::EndGarbage,
                '!' => Token::Ignore(chars.next().unwrap()),
                garbage => Token::Garbage(garbage),
            }
        } else {
            match c {
                '{' => Token::StartGroup,
                '}' => Token::EndGroup,
                '<' => Token::StartGarbage,
                ',' => Token::Separator,
                invalid => {
                    return Err(format!("could not parse to token: invalid character '{}'",
                                       invalid));
                }
            }
        };

        if token == Token::StartGarbage {
            is_garbage = true;
        } else if token == Token::EndGarbage {
            is_garbage = false;
        }

        tokens.push(token);
    }

    Ok(tokens)
}

fn process_tokens(tokens: &[Token]) -> (u64, u64) {
    let mut score = 0;
    let mut current_group = 0;
    for &token in tokens {
        match token {
            Token::StartGroup => {
                current_group += 1;
                score += current_group;
            }
            Token::EndGroup => current_group -= 1,
            _ => {}
        };
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "{}";
            let expected = "1";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "{{{}}}";
            let expected = "6";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "{{},{}}";
            let expected = "5";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_4() {
            let solver = get_solver();
            let input = "{{{},{},{{}}}}";
            let expected = "16";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_5() {
            let solver = get_solver();
            let input = "{<a>,<a>,<a>,<a>}";
            let expected = "1";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_6() {
            let solver = get_solver();
            let input = "{{<ab>},{<ab>},{<ab>},{<ab>}}";
            let expected = "9";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_7() {
            let solver = get_solver();
            let input = "{{<!!>},{<!!>},{<!!>},{<!!>}}";
            let expected = "9";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_8() {
            let solver = get_solver();
            let input = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
            let expected = "3";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "put some input here";
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
