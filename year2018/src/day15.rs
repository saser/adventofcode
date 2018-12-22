use std::cmp::Ordering;
use std::collections::BTreeMap;

use base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day15)
}

struct Day15;

type Cavern = BTreeMap<Pos, Tile>;
type Units = BTreeMap<Pos, Unit>;

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
struct Pos(usize, usize);

impl Ord for Pos {
    fn cmp(&self, other: &Pos) -> Ordering {
        match self.0.cmp(&other.0) {
            Ordering::Equal => self.1.cmp(&other.1),
            ordering => ordering,
        }
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Pos) -> Option<Ordering> {
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
            let pos = Pos(row, col);
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
            cavern.insert(pos, tile);
            if let Some(unit) = opt_unit {
                units.insert(pos, unit);
            }
        }
    }
    (cavern, units)
}

fn print_cavern(cavern: &Cavern) {
    let mut last_row = 0;
    for (&Pos(row, _col), &tile) in cavern.iter() {
        if row > last_row {
            println!();
        }
        last_row = row;
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
