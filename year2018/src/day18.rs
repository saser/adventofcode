use nalgebra::DMatrix;

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
        println!("{}", tiles);
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
