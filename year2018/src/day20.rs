use base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day20)
}

struct Day20;

impl Solver for Day20 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let regex = parse(input);
        println!("{:?}", regex);
        match part {
            Part::One => Err("day 20 part 1 not yet implemented".to_string()),
            Part::Two => Err("day 20 part 2 not yet implemented".to_string()),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Regex {
    tokens: Vec<Token>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Token {
    Terminals(Vec<char>),
    Branch(Vec<Regex>),
}

fn parse(input: &str) -> Regex {
    let regex_chars = input.chars().collect::<Vec<char>>();
    let (regex, _offset) = parse_regex(&regex_chars);
    regex
}

fn parse_regex(regex_chars: &[char]) -> (Regex, usize) {
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < regex_chars.len() {
        match regex_chars[i] {
            '^' | '$' => i += 1,
            '|' | ')' => break,
            '(' => {
                let (regexes, offset) = parse_branch(&regex_chars[i..]);
                tokens.push(Token::Branch(regexes));
                i += offset;
            }
            t if is_terminal(t) => {
                let (terminals, offset) = parse_terminals(&regex_chars[i..]);
                tokens.push(Token::Terminals(terminals));
                i += offset;
            }
            c => panic!("parse_regex: unexpected char: {}", c),
        };
    }
    (Regex { tokens }, i)
}

fn parse_terminals(regex_chars: &[char]) -> (Vec<char>, usize) {
    let terminals = regex_chars
        .iter()
        .cloned()
        .take_while(|&c| is_terminal(c))
        .collect::<Vec<char>>();
    let offset = terminals.len();
    (terminals, offset)
}

fn parse_branch(regex_chars: &[char]) -> (Vec<Regex>, usize) {
    let mut regexes = Vec::new();
    let mut i = 0;
    while i < regex_chars.len() {
        match regex_chars[i] {
            '(' | '|' => {
                i += 1;
                let (regex, offset) = parse_regex(&regex_chars[i..]);
                regexes.push(regex);
                i += offset;
            }
            ')' => {
                i += 1;
                break;
            }
            c => panic!("parse_branch: unexpected char: {}", c),
        };
    }
    (regexes, i)
}

fn is_terminal(c: char) -> bool {
    ['N', 'E', 'S', 'W'].contains(&c)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/20").trim();
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "^WNE$";
            let expected = "3";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "^ENWWW(NEEE|SSE(EE|N))$";
            let expected = "10";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
            let expected = "18";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_4() {
            let solver = get_solver();
            let input = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
            let expected = "23";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_5() {
            let solver = get_solver();
            let input = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
            let expected = "31";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/20").trim();
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
