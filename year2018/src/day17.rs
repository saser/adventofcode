use lazy_static::lazy_static;
use regex::Regex;

use std::collections::BTreeSet;
use std::iter;

use base::grid::Grid as BaseGrid;
use base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day17)
}

struct Day17;

type Grid = BaseGrid<char>;

impl Solver for Day17 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let (grid, adjusted_spring) = parse_input(input);
        grid.print();
        match part {
            Part::One => Err("day 17 part 1 not yet implemented".to_string()),
            Part::Two => Err("day 17 part 2 not yet implemented".to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    row: usize,
    col: usize,
}

impl Into<(usize, usize)> for Position {
    fn into(self) -> (usize, usize) {
        (self.row, self.col)
    }
}

fn parse_input(input: &str) -> (Grid, Position) {
    let clay = input
        .lines()
        .fold(BTreeSet::new(), |acc, line| &acc | &parse_line(line));
    let spring = Position { row: 0, col: 500 };
    let with_spring = clay.iter().chain(iter::once(&spring));
    let clay_rows = with_spring.clone().map(|position| position.row);
    let min_row = clay_rows.clone().min().unwrap();
    let max_row = clay_rows.clone().max().unwrap();
    let clay_cols = with_spring.clone().map(|position| position.col);
    let min_col = clay_cols.clone().min().unwrap();
    let max_col = clay_cols.clone().max().unwrap();
    let nrows = max_row - min_row + 1;
    let ncols = max_col - min_col + 1;
    let adjust = |position: Position| Position {
        row: position.row - min_row,
        col: position.col - min_col,
    };
    let mut grid = Grid::with(nrows, ncols, &'.');
    for &position in &clay {
        grid[adjust(position)] = '#';
    }
    let adjusted_spring = adjust(spring);
    grid[adjusted_spring] = '+';
    (grid, adjusted_spring)
}

fn parse_line(line: &str) -> BTreeSet<Position> {
    lazy_static! {
        static ref VEIN_RE: Regex = Regex::new(
            r"(?P<n1>x|y)=(?P<c1>\d+), (?P<n2>x|y)=(?P<c2_start>\d+)\.\.(?P<c2_end>\d+)"
        )
        .unwrap();
    }
    let captures = VEIN_RE.captures(line).unwrap();
    let c1 = captures["c1"].parse::<usize>().unwrap();
    let c2_start = captures["c2_start"].parse::<usize>().unwrap();
    let c2_end = captures["c2_end"].parse::<usize>().unwrap();
    let x_first = &captures["n1"] == "x";
    (c2_start..=c2_end)
        .map(|c2| {
            let (x, y) = if x_first { (c1, c2) } else { (c2, c1) };
            Position { col: x, row: y }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/17").trim();
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504\
                ";
            let expected = "57";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/17").trim();
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
