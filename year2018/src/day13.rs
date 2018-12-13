use base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day13)
}

struct Day13;

impl Solver for Day13 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        match part {
            Part::One => Err("day 13 part 1 not yet implemented".to_string()),
            Part::Two => Err("day 13 part 2 not yet implemented".to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Tile {
    None,
    Vertical,
    Horizontal,
    Intersection,
    ForwardSlash,
    BackwardSlash,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::None
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            '/' => Tile::ForwardSlash,
            '+' => Tile::Intersection,
            '\\' => Tile::BackwardSlash,
            _ => Tile::None,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!("invalid direction char: {}", c),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct Cart {
    row: usize,
    col: usize,
    dir: Direction,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/13").trim();
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/13_example").trim();
            let expected = "7,3";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/13").trim();
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
