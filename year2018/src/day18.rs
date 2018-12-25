use nalgebra::DMatrix;

use std::cmp::{max, min};
use std::fmt;

use base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day18)
}

struct Day18;

type Tiles = DMatrix<Tile>;

impl Solver for Day18 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let tiles = parse_input(input);
        println!("{:?}", surrounding(0, 0, &tiles));
        match part {
            Part::One => Err("day 18 part 1 not yet implemented".to_string()),
            Part::Two => Err("day 18 part 2 not yet implemented".to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Tile {
    Open,
    Tree,
    Lumberyard,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match *self {
            Tile::Open => '.',
            Tile::Tree => '|',
            Tile::Lumberyard => '#',
        };
        write!(f, "{}", c)
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => Tile::Tree,
            '#' => Tile::Lumberyard,
            _ => Tile::Open,
        }
    }
}

fn parse_input(input: &str) -> Tiles {
    let chars = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let nrows = chars.len();
    let ncols = chars[0].len();
    Tiles::from_fn(nrows, ncols, |row, col| Tile::from(chars[row][col]))
}

fn surrounding(row: usize, col: usize, tiles: &Tiles) -> Vec<Tile> {
    let start_row = max(0, row as isize - 1) as usize;
    let start_col = max(0, col as isize - 1) as usize;
    let end_row = min((tiles.nrows() - 1) as isize, (row + 1) as isize) as usize;
    let end_col = min((tiles.ncols() - 1) as isize, (col + 1) as isize) as usize;
    let nrows = end_row - start_row + 1;
    let ncols = end_col - start_col + 1;
    let start = (start_row, start_col);
    let shape = (nrows, ncols);

    let mut surrounding = tiles
        .slice(start, shape)
        .iter()
        .cloned()
        .collect::<Vec<Tile>>();
    let middle_tile = tiles[(row, col)];
    let middle_tile_index = surrounding
        .iter()
        .position(|&tile| tile == middle_tile)
        .unwrap();
    surrounding.remove(middle_tile_index);
    surrounding
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/18").trim();
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.\
            ";
            let expected = "1147";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/18").trim();
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
