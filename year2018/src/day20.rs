use std::collections::{HashMap, HashSet};

use base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day20)
}

struct Day20;

impl Solver for Day20 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let regex = parse(input);
        println!("{:?}", regex);
        let graph = construct(&regex);
        println!("{:?}", graph);
        match part {
            Part::One => Err("day 20 part 1 not yet implemented".to_string()),
            Part::Two => Err("day 20 part 2 not yet implemented".to_string()),
        }
    }
}

type Graph = HashMap<Position, HashSet<Position>>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn north(&self) -> Self {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn south(&self) -> Self {
        Position {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn east(&self) -> Self {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn west(&self) -> Self {
        Position {
            x: self.x - 1,
            y: self.y,
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

fn construct(regex: &Regex) -> Graph {
    let mut graph = Graph::new();
    let origin = Position { x: 0, y: 0 };
    let mut positions = HashSet::new();
    positions.insert(origin);
    construct_regex(regex, &positions, &mut graph);
    graph
}

fn construct_regex(
    regex: &Regex,
    positions: &HashSet<Position>,
    graph: &mut Graph,
) -> HashSet<Position> {
    let mut new_positions = positions.clone();
    for token in &regex.tokens {
        match token {
            Token::Terminals(ref terminals) => {
                new_positions = construct_terminals(terminals, &new_positions, graph);
            }
            Token::Branch(ref regexes) => {
                new_positions = construct_branch(regexes, &new_positions, graph);
            }
        }
    }
    new_positions
}

fn construct_terminals(
    terminals: &[char],
    positions: &HashSet<Position>,
    graph: &mut Graph,
) -> HashSet<Position> {
    let mut new_positions = HashSet::new();
    for &position in positions {
        let mut current_position = position;
        for t in terminals {
            let next_position = match t {
                'N' => current_position.north(),
                'E' => current_position.east(),
                'S' => current_position.south(),
                'W' => current_position.west(),
                _ => unreachable!(),
            };
            graph
                .entry(current_position)
                .or_insert_with(HashSet::new)
                .insert(next_position);
            graph
                .entry(next_position)
                .or_insert_with(HashSet::new)
                .insert(current_position);
            current_position = next_position;
        }
        new_positions.insert(current_position);
    }
    new_positions
}

fn construct_branch(
    regexes: &[Regex],
    positions: &HashSet<Position>,
    graph: &mut Graph,
) -> HashSet<Position> {
    let mut new_positions = HashSet::new();
    for regex in regexes {
        new_positions.extend(construct_regex(regex, positions, graph));
    }
    new_positions
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
