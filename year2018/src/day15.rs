use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day15)
}

struct Day15;

type Path = Vec<Position>;
type Cavern = BTreeMap<Position, Tile>;
type Units = BTreeMap<Position, Unit>;

impl Solver for Day15 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let (cavern, units) = parse_input(input);
        print_cavern(&cavern);
        match part {
            Part::One => Err("day 15 part 1 not yet implemented".to_string()),
            Part::Two => Err("day 15 part 2 not yet implemented".to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Position {
    row: isize,
    col: isize,
}

impl Ord for Position {
    fn cmp(&self, other: &Position) -> Ordering {
        match self.row.cmp(&other.row) {
            Ordering::Equal => self.col.cmp(&other.col),
            ordering => ordering,
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Tile {
    Wall,
    Open,
    Unit(Unit),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Unit {
    unit_type: UnitType,
    hitpoints: i64,
    attack_power: i64,
}

impl Unit {
    fn new(unit_type: UnitType) -> Self {
        Unit {
            unit_type,
            hitpoints: 200,
            attack_power: 3,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum UnitType {
    Goblin,
    Elf,
}

fn parse_input(input: &str) -> (Cavern, Units) {
    let mut cavern = Cavern::new();
    let mut units = Units::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let position = Position {
                row: row as isize,
                col: col as isize,
            };
            let opt_unit = match c {
                'G' => Some(Unit::new(UnitType::Goblin)),
                'E' => Some(Unit::new(UnitType::Elf)),
                _ => None,
            };
            let tile = match c {
                '#' => Tile::Wall,
                '.' => Tile::Open,
                'G' | 'E' => Tile::Unit(opt_unit.unwrap()),
                _ => unreachable!(),
            };
            cavern.insert(position, tile);
            if let Some(unit) = opt_unit {
                units.insert(position, unit);
            }
        }
    }
    (cavern, units)
}

fn print_cavern(cavern: &Cavern) {
    let mut last_row = 0;
    for (&position, &tile) in cavern.iter() {
        if position.row > last_row {
            println!();
        }
        last_row = position.row;
        let c = match tile {
            Tile::Wall => '#',
            Tile::Open => '.',
            Tile::Unit(unit) => match unit.unit_type {
                UnitType::Goblin => 'G',
                UnitType::Elf => 'E',
            },
        };
        print!("{}", c);
    }
    println!();
}

fn adjacent_positions(position: Position) -> BTreeSet<Position> {
    [(-1, 0), (0, -1), (1, 0), (0, 1)]
        .into_iter()
        .map(|(drow, dcol)| Position {
            row: position.row + drow,
            col: position.col + dcol,
        })
        .collect()
}

fn in_range(position: Position, cavern: &Cavern) -> BTreeSet<Position> {
    adjacent_positions(position)
        .into_iter()
        .filter(|adjacent| {
            cavern.contains_key(&adjacent) && *cavern.get(&adjacent).unwrap() == Tile::Open
        })
        .collect()
}


#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct SPEntry {
    position: Position,
    path: Path,
}

impl Ord for SPEntry {
    fn cmp(&self, other: &SPEntry) -> Ordering {
        let mut ordering = other.path.len().cmp(&self.path.len());
        if ordering != Ordering::Equal {
            return ordering;
        }
        for (o, s) in other.path.iter().zip(self.path.iter()) {
            ordering = o.cmp(s);
            if ordering != Ordering::Equal {
                break;
            }
        }
        ordering
    }
}

impl PartialOrd for SPEntry {
    fn partial_cmp(&self, other: &SPEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(from: Position, to: Position, cavern: &Cavern) -> Option<Path> {
    let mut queue = BinaryHeap::new();
    let mut visited: BTreeMap<Position, Path> = BTreeMap::new();
    queue.extend(in_range_entries(from, &Vec::new(), cavern));
    while let Some(next) = queue.pop() {
        if let Some(path) = visited.get(&next.position) {
            if next.path.len() == path.len() && next.path[0] < path[0] {
                visited.insert(next.position, next.path.clone());
            } else {
                continue;
            }
        }
        visited.insert(next.position, next.path.clone());
        queue.extend(in_range_entries(next.position, &next.path, cavern))
    }
    visited.remove(&to)
}

fn in_range_entries(position: Position, base_path: &Path, cavern: &Cavern) -> Vec<SPEntry> {
    in_range(position, cavern)
        .iter()
        .map(|&in_range| {
            let position = in_range;
            let mut path = base_path.clone();
            path.push(position);
            SPEntry { position, path }
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
            let input = include_str!("../../inputs/2018/15").trim();
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "\
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######\
            ";
            let expected = "27730";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "\
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######\
            ";
            let expected = "36334";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "\
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######\
            ";
            let expected = "39514";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_4() {
            let solver = get_solver();
            let input = "\
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######\
            ";
            let expected = "27755";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_5() {
            let solver = get_solver();
            let input = "\
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######\
            ";
            let expected = "28944";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_6() {
            let solver = get_solver();
            let input = "\
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########\
            ";
            let expected = "18740";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/15").trim();
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
