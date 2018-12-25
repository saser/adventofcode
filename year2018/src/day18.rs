use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

use base::grid::Grid;
use base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day18)
}

struct Day18;

type Tiles = Grid<Tile>;

impl Solver for Day18 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let tiles = parse_input(input);
        match part {
            Part::One => {
                let iterations = 10;
                let final_tiles = run_iterations(iterations, &tiles);
                let counts = count(final_tiles.iter());
                let resource_value = counts[&Tile::Tree] * counts[&Tile::Lumberyard];
                Ok(resource_value.to_string())
            }
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

impl Tile {
    fn next(&self, surrounding: &[Tile]) -> Self {
        let mut counts = count(surrounding.iter());
        let trees = *counts.entry(&Tile::Tree).or_insert(0);
        let lumberyards = *counts.entry(&Tile::Lumberyard).or_insert(0);
        match *self {
            Tile::Open => {
                if trees >= 3 {
                    Tile::Tree
                } else {
                    *self
                }
            }
            Tile::Tree => {
                if lumberyards >= 3 {
                    Tile::Lumberyard
                } else {
                    *self
                }
            }
            Tile::Lumberyard => {
                if lumberyards >= 1 && trees >= 1 {
                    *self
                } else {
                    Tile::Open
                }
            }
        }
    }
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
    let mut grid = Tiles::with(nrows, ncols, &Tile::Open);
    for row in 0..nrows {
        for col in 0..ncols {
            grid[(row, col)] = Tile::from(chars[row][col]);
        }
    }
    grid
}

fn surrounding(row: usize, col: usize, tiles: &Tiles) -> Vec<Tile> {
    let start_row = max(0, row as isize - 1) as usize;
    let start_col = max(0, col as isize - 1) as usize;
    let end_row = min((tiles.nrows() - 1) as isize, (row + 1) as isize) as usize;
    let end_col = min((tiles.ncols() - 1) as isize, (col + 1) as isize) as usize;

    let mut surrounding = Vec::new();
    for s_row in start_row..=end_row {
        for s_col in start_col..=end_col {
            if s_row == row && s_col == col {
                continue;
            }
            surrounding.push(tiles[(s_row, s_col)]);
        }
    }
    surrounding
}

fn count<T, I>(iter: I) -> HashMap<T, usize>
where
    T: Eq + Hash + Copy,
    I: Iterator<Item = T>,
{
    let mut map = HashMap::new();
    for item in iter {
        *map.entry(item).or_insert(0) += 1;
    }
    map
}

fn iteration(tiles: &Tiles) -> Tiles {
    let mut new_tiles = tiles.clone();
    for row in 0..tiles.nrows() {
        for col in 0..tiles.ncols() {
            let tile = tiles[(row, col)];
            let surrounding = surrounding(row, col, tiles);
            new_tiles[(row, col)] = tile.next(&surrounding);
        }
    }
    new_tiles
}

fn run_iterations(iterations: usize, tiles: &Tiles) -> Tiles {
    let mut current_tiles = tiles.clone();
    let mut seen = HashMap::new();
    seen.insert(current_tiles.clone(), 0);
    for i in 1..=iterations {
        let new_tiles = iteration(&current_tiles);
        if let Some(seen_i) = seen.get(&new_tiles) {
            let loop_length = i - seen_i;
            let iterations_left = (iterations - seen_i) % loop_length;
            return run_iterations(iterations_left, &new_tiles);
        }
        seen.insert(new_tiles.clone(), i);
        current_tiles = new_tiles;
    }
    current_tiles
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
            let expected = "545600";
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
