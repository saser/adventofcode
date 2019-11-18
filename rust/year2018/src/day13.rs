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
        let (tiles, carts) = parse_input(input);
        match part {
            Part::One => {
                let (mut remaining_carts, mut collision_positions) = tick(&tiles, &carts);
                while collision_positions.is_empty() {
                    let (new_remaining_carts, new_collision_positions) =
                        tick(&tiles, &remaining_carts);
                    remaining_carts = new_remaining_carts;
                    collision_positions = new_collision_positions;
                }
                let (x, y) = rowcol_to_xy(collision_positions[0]);
                Ok(format!("{},{}", x, y))
            }
            Part::Two => {
                let mut remaining_carts = carts.clone();
                while remaining_carts.len() > 1 {
                    remaining_carts = tick(&tiles, &remaining_carts).0;
                }
                println!("remaining carts: {:?}", remaining_carts);
                let remaining_cart = remaining_carts.iter().next().unwrap();
                let (x, y) = rowcol_to_xy((remaining_cart.row, remaining_cart.col));
                Ok(format!("{},{}", x, y))
            }
        }
    }
}

fn rowcol_to_xy((row, col): (usize, usize)) -> (usize, usize) {
    let x = col;
    let y = row;
    (x, y)
}

#[allow(dead_code)]
fn print_tracks(tiles: &Tiles, carts: &Carts) {
    let nrows = tiles.nrows();
    let ncols = tiles.ncols();
    let mut chars = vec![vec![' '; ncols]; nrows];
    for row in 0..nrows {
        for col in 0..ncols {
            chars[row][col] = tiles[(row, col)].into();
        }
    }
    for &cart in carts.iter() {
        chars[cart.row][cart.col] = cart.into();
    }
    for line in &chars {
        println!("{}", line.iter().collect::<String>());
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
            '<' | '>' => Tile::Horizontal,
            '^' | 'v' => Tile::Vertical,
            _ => Tile::None,
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Tile::None => ' ',
            Tile::Vertical => '|',
            Tile::Horizontal => '-',
            Tile::Intersection => '+',
            Tile::ForwardSlash => '/',
            Tile::BackwardSlash => '\\',
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

impl Direction {
    fn turn(&self, t: &Turn) -> Self {
        match *t {
            Turn::Left => match *self {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
            },
            Turn::Right => match *self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
            Turn::Straight => *self,
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

impl Into<char> for Cart {
    fn into(self) -> char {
        match self.dir {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }
}

fn parse_input(input: &str) -> (Tiles, Carts) {
    let char_grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let nrows = char_grid.len();
    let ncols = char_grid.iter().map(|row| row.len()).max().unwrap();
    let mut tiles = Grid::new(nrows, ncols);
    let mut carts = BTreeSet::new();
    for (row, row_chars) in char_grid.iter().enumerate() {
        for (col, &c) in row_chars.iter().enumerate() {
            tiles[(row, col)] = Tile::from(char_grid[row][col]);
            if ['^', '>', 'v', '<'].contains(&c) {
                let dir = Direction::from(c);
                carts.insert(Cart {
                    row,
                    col,
                    dir,
                    turn: Turn::Left,
                });
            }
        }
    }
    (tiles, carts)
}

fn tick(tiles: &Tiles, carts: &Carts) -> (Carts, Vec<(usize, usize)>) {
    let mut remaining_carts = carts.clone();
    let mut collision_positions = Vec::new();
    for cart in carts.iter() {
        if !remaining_carts.contains(cart) {
            continue;
        }
        let stepped_cart = step_cart(tiles, cart);
        remaining_carts.remove(cart);
        if let Some(other_cart) = did_collide(&stepped_cart, &remaining_carts) {
            remaining_carts.remove(&other_cart);
            remaining_carts.remove(&stepped_cart);
            collision_positions.push((stepped_cart.row, stepped_cart.col));
        } else {
            remaining_carts.insert(stepped_cart);
        }
    }
    (remaining_carts, collision_positions)
}

fn did_collide(cart: &Cart, carts: &Carts) -> Option<Cart> {
    let destroyed_cart = carts
        .iter()
        .cloned()
        .find(|&other_cart| cart.row == other_cart.row && cart.col == other_cart.col);
    destroyed_cart
}

fn step_cart(tiles: &Tiles, cart: &Cart) -> Cart {
    let (new_row, new_col) = match cart.dir {
        Direction::Up => (cart.row - 1, cart.col),
        Direction::Right => (cart.row, cart.col + 1),
        Direction::Down => (cart.row + 1, cart.col),
        Direction::Left => (cart.row, cart.col - 1),
    };
    let (new_dir, new_turn) = match tiles[(new_row, new_col)] {
        Tile::ForwardSlash => (
            match cart.dir {
                Direction::Up | Direction::Down => cart.dir.turn(&Turn::Right),
                Direction::Right | Direction::Left => cart.dir.turn(&Turn::Left),
            },
            cart.turn,
        ),
        Tile::BackwardSlash => (
            match cart.dir {
                Direction::Up | Direction::Down => cart.dir.turn(&Turn::Left),
                Direction::Right | Direction::Left => cart.dir.turn(&Turn::Right),
            },
            cart.turn,
        ),
        Tile::Intersection => (cart.dir.turn(&cart.turn), cart.turn.next()),
        _ => (cart.dir, cart.turn),
    };
    Cart {
        row: new_row,
        col: new_col,
        dir: new_dir,
        turn: new_turn,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../../inputs/2018/13").trim();
            let expected = "16,45";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = include_str!("../../../inputs/2018/13_example1").trim();
            let expected = "7,3";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../../inputs/2018/13").trim_end();
            let expected = "21,91";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = include_str!("../../../inputs/2018/13_example2").trim();
            let expected = "6,4";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
