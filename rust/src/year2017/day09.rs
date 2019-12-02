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
    let tokens = parse_tokens(input.trim())?;
    let (score, removed_garbage) = process_tokens(&tokens);
    match part {
        Part::One => Ok(score.to_string()),
        Part::Two => Ok(removed_garbage.to_string()),
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
                    return Err(format!(
                        "could not parse to token: invalid character '{}'",
                        invalid
                    ));
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
    let mut removed_garbage = 0;
    for &token in tokens {
        match token {
            Token::StartGroup => {
                current_group += 1;
                score += current_group;
            }
            Token::EndGroup => current_group -= 1,
            Token::Garbage(_) => removed_garbage += 1,
            _ => {}
        };
    }
    (score, removed_garbage)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(example1, "{}", "1", part1);
        test!(example2, "{{{}}}", "6", part1);
        test!(example3, "{{},{}}", "5", part1);
        test!(example4, "{{{},{},{{}}}}", "16", part1);
        test!(example5, "{<a>,<a>,<a>,<a>}", "1", part1);
        test!(example6, "{{<ab>},{<ab>},{<ab>},{<ab>}}", "9", part1);
        test!(example7, "{{<!!>},{<!!>},{<!!>},{<!!>}}", "9", part1);
        test!(example8, "{{<a!>},{<a!>},{<a!>},{<ab>}}", "3", part1);
        test!(
            actual,
            include_str!("../../../inputs/2017/09"),
            "21037",
            part1
        );
    }

    mod part2 {
        use super::*;

        test!(example1, "<>", "0", part2);
        test!(example2, "<random characters>", "17", part2);
        test!(example3, "<<<<>", "3", part2);
        test!(example4, "<{!>}>", "2", part2);
        test!(example5, "<!!>", "0", part2);
        test!(example6, "<!!!>>", "0", part2);
        test!(example7, "<{o\"i!a,<{i<a>", "10", part2);
        test!(
            actual,
            include_str!("../../../inputs/2017/09"),
            "9495",
            part2
        );
    }
}
