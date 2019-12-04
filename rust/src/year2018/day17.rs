use std::collections::BTreeSet;
use std::io;
use std::iter;

use lazy_static::lazy_static;
use regex::Regex;

use crate::base::grid::Grid as BaseGrid;
use crate::base::Part;

type Grid = BaseGrid<char>;

pub fn part1(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::One)
}

pub fn part2(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::Two)
}

fn solve(r: &mut dyn io::Read, part: Part) -> Result<String, String> {
    let mut input = String::new();
    r.read_to_string(&mut input).map_err(|e| e.to_string())?;
    let (mut grid, adjusted_spring, spring_offset) = parse_input(&input);
    flow(adjusted_spring.down(), &mut grid);
    match part {
        Part::One => {
            let count = grid.iter().filter(|&&c| water(c)).count() - spring_offset;
            Ok(count.to_string())
        }
        Part::Two => {
            let count = grid.iter().filter(|&&c| c == '~').count();
            Ok(count.to_string())
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn up(&self) -> Self {
        Position {
            row: self.row - 1,
            col: self.col,
        }
    }

    fn down(&self) -> Self {
        Position {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn left(&self) -> Self {
        Position {
            row: self.row,
            col: self.col - 1,
        }
    }

    fn right(&self) -> Self {
        Position {
            row: self.row,
            col: self.col + 1,
        }
    }
}

impl Into<(usize, usize)> for Position {
    fn into(self) -> (usize, usize) {
        (self.row, self.col)
    }
}

fn parse_input(input: &str) -> (Grid, Position, usize) {
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
    // The +2 is for the extra open tiles at the left and right edges of the bounding boxes.
    let ncols = max_col - min_col + 1 + 2;
    let adjust = |position: Position| Position {
        row: position.row - min_row,
        col: position.col - min_col + 1,
    };
    let mut grid = Grid::with(nrows, ncols, &'.');
    for &position in &clay {
        grid[adjust(position)] = '#';
    }
    let adjusted_spring = adjust(spring);
    grid[adjusted_spring] = '+';
    let clay_min_row = clay.iter().map(|position| position.row).min().unwrap();
    let spring_offset = clay_min_row - 1;
    (grid, adjusted_spring, spring_offset)
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

fn flow(start: Position, grid: &mut Grid) {
    let touchdown = match flow_down(start, grid) {
        Some(position) => position,
        None => return,
    };

    let mut floor = touchdown;
    let (mut opt_left_start, mut opt_right_start) = flow_sideways(floor, grid);
    while opt_left_start.is_none() && opt_right_start.is_none() {
        floor = floor.up();
        let (new_opt_left_start, new_opt_right_start) = flow_sideways(floor, grid);
        opt_left_start = new_opt_left_start;
        opt_right_start = new_opt_right_start;
    }
    if let Some(left_start) = opt_left_start {
        flow(left_start, grid);
    }
    if let Some(right_start) = opt_right_start {
        flow(right_start, grid);
    }
}

fn flow_down(start: Position, grid: &mut Grid) -> Option<Position> {
    let positions = (start.row..grid.nrows()).map(|row| Position {
        row,
        col: start.col,
    });
    for position in positions {
        if grid[position] == '|' {
            return None;
        }
        grid[position] = '|';
        let down = position.down();
        if down.row < grid.nrows() {
            if blocking(grid[down]) {
                return Some(position);
            }
        }
    }
    None
}

fn flow_sideways(start: Position, grid: &mut Grid) -> (Option<Position>, Option<Position>) {
    let mut floor_positions = Vec::new();
    let mut left_start = None;
    let mut has_left_wall = false;
    let left_positions = (1..=start.col).rev().map(|col| Position {
        row: start.row,
        col,
    });
    for position in left_positions {
        floor_positions.push(position);
        let next_left = position.left();
        if blocking(grid[next_left]) {
            has_left_wall = true;
            break;
        }
        if free(grid[next_left.down()]) {
            left_start = Some(next_left);
            break;
        }
    }

    let mut right_start = None;
    let mut has_right_wall = false;
    let right_positions = (start.col..grid.ncols()).map(|col| Position {
        row: start.row,
        col,
    });
    for position in right_positions {
        floor_positions.push(position);
        let next_right = position.right();
        if blocking(grid[next_right]) {
            has_right_wall = true;
            break;
        }
        if free(grid[next_right.down()]) {
            right_start = Some(next_right);
            break;
        }
    }
    let square = if has_right_wall && has_left_wall {
        '~'
    } else {
        '|'
    };
    for position in floor_positions {
        grid[position] = square;
    }
    (left_start, right_start)
}

fn water(c: char) -> bool {
    ['|', '~'].contains(&c)
}

fn blocking(c: char) -> bool {
    ['#', '~'].contains(&c)
}

fn free(c: char) -> bool {
    ['.', '|'].contains(&c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(example, include_str!("testdata/day17/ex"), "57", part1);
        test!(
            actual,
            include_str!("../../../inputs/2018/17"),
            "31471",
            part1
        );
    }

    mod part2 {
        use super::*;

        test!(example, include_str!("testdata/day17/ex"), "29", part2);
        test!(
            actual,
            include_str!("../../../inputs/2018/17"),
            "24169",
            part2
        );
    }
}
