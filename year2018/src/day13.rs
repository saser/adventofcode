use std::collections::BTreeSet;

use base::grid::Grid;
use base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day13)
}

struct Day13;

type Tiles = Grid<Tile>;
type Carts = BTreeSet<Cart>;

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
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next(&self) -> Self {
        match *self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct Cart {
    row: usize,
    col: usize,
    dir: Direction,
    turn: Turn,
}

fn parse_input(input: &str) -> (Tiles, Carts) {
    let char_grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let nrows = char_grid.len();
    let ncols = char_grid.iter().map(|row| row.len()).max().unwrap();
    let mut tiles = Grid::new(nrows, ncols);
    let mut carts = BTreeSet::new();
    for row in 0..nrows {
        for col in 0..ncols {
            let c = char_grid[row][col];
            if ['^', '>', 'v', '<'].contains(&c) {
                let dir = Direction::from(c);
                carts.insert(Cart {
                    row,
                    col,
                    dir,
                    turn: Turn::Left,
                });
            } else {
                tiles[(row, col)] = Tile::from(char_grid[row][col]);
            }
        }
    }
    (tiles, carts)
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
