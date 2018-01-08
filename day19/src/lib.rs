extern crate base;

use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day19)
}

struct Day19;

impl Solver for Day19 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        Err("day 19 not yet implemented".to_string())
    }
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<Tile> {
    line.chars().map(|c| Tile::from(c)).collect()
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Tile {
    Empty,
    Horizontal,
    Vertical,
    Corner,
    Letter(char),
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '-' => Tile::Horizontal,
            '|' => Tile::Vertical,
            '+' => Tile::Corner,
            'A'...'Z' => Tile::Letter(c),
            _ => Tile::Empty,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ \
            ";
            let expected = "ABCDEF";
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
