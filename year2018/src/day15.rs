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
        match part {
            Part::One => {
                let (full_rounds, _cavern_after_combat, units_after_combat) =
                    combat(&cavern, &units);
                let hitpoints_sum = units_after_combat
                    .values()
                    .map(|unit| unit.hitpoints as usize)
                    .sum::<usize>();
                let outcome = full_rounds * hitpoints_sum;
                Ok(outcome.to_string())
            }
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

#[allow(dead_code)]
fn print_cavern(cavern: &Cavern, units: &Units) {
    let mut last_row = 0;
    for (&position, &tile) in cavern.iter() {
        if position.row > last_row {
            print!("   ");
            for (unit_position, unit) in units.iter() {
                if unit_position.row != last_row {
                    continue;
                }
                let c = match unit.unit_type {
                    UnitType::Goblin => 'G',
                    UnitType::Elf => 'E',
                };
                print!("{}({}), ", c, unit.hitpoints);
            }
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

fn find_target_positions(target_unit_type: UnitType, units: &Units) -> BTreeSet<Position> {
    units
        .iter()
        .filter_map(|(&unit_position, unit)| {
            if unit.unit_type == target_unit_type {
                Some(unit_position)
            } else {
                None
            }
        })
        .collect()
}

fn combat(cavern: &Cavern, units: &Units) -> (usize, Cavern, Units) {
    let mut current_cavern = cavern.clone();
    let mut current_units = units.clone();
    let mut full_rounds = 0;
    let mut combat_ended = false;
    while !combat_ended {
        let (cavern_after_round, units_after_round, combat_ended_during_round) =
            round(&current_cavern, &current_units);
        current_cavern = cavern_after_round;
        current_units = units_after_round;
        combat_ended = combat_ended_during_round;
        if !combat_ended {
            full_rounds += 1;
        }
    }
    (full_rounds, current_cavern, current_units)
}

fn round(cavern: &Cavern, units: &Units) -> (Cavern, Units, bool) {
    let mut current_cavern = cavern.clone();
    let mut current_units = units.clone();
    for &acting_position in units.keys() {
        if !current_units.contains_key(&acting_position) {
            // The unit that would have acted has died.
            continue;
        }
        let (cavern_after_turn, units_after_turn, combat_ended) =
            turn(acting_position, &current_cavern, &current_units);
        if combat_ended {
            return (cavern_after_turn, units_after_turn, true);
        }
        current_cavern = cavern_after_turn;
        current_units = units_after_turn;
    }
    (current_cavern, current_units, false)
}

fn turn(acting_position: Position, cavern: &Cavern, units: &Units) -> (Cavern, Units, bool) {
    let acting_unit = *units.get(&acting_position).unwrap();
    let target_unit_type = match acting_unit.unit_type {
        UnitType::Goblin => UnitType::Elf,
        UnitType::Elf => UnitType::Goblin,
    };
    let target_positions = find_target_positions(target_unit_type, units);
    if target_positions.is_empty() {
        // There are no targets at all, so combat ends without anything being changed.
        return (cavern.clone(), units.clone(), true);
    }

    // There are still targets left.
    if let Some((cavern_after_attacking, units_after_attacking)) =
        perform_attack_if_possible(acting_position, &target_positions, cavern, units)
    {
        // The acting unit could perform an attack, so end the turn and return the results of
        // attacking.
        return (cavern_after_attacking, units_after_attacking, false);
    }

    // The acting unit is not currently in range of attacking anyone, and will therefore try to
    // move.
    let in_range_positions = target_positions
        .iter()
        .flat_map(|&target_position| in_range(target_position, cavern))
        .collect::<BTreeSet<Position>>();
    if in_range_positions.is_empty() {
        // There are no open squares adjacent to any of the targets, so the acting unit cannot
        // move, ending its turn without anything being changed.
        return (cavern.clone(), units.clone(), false);
    }

    // The acting tries to move (might not be able to move due to being locked in).
    let (position_after_moving, cavern_after_moving, units_after_moving) =
        perform_move(acting_position, &in_range_positions, cavern, units);

    let mut new_cavern = cavern_after_moving.clone();
    let mut new_units = units_after_moving.clone();
    // After moving, the acting unit might be able to attack a target.
    if let Some((cavern_after_attacking, units_after_attacking)) = perform_attack_if_possible(
        position_after_moving,
        &target_positions,
        &cavern_after_moving,
        &units_after_moving,
    ) {
        new_cavern = cavern_after_attacking;
        new_units = units_after_attacking;
    }

    // The unit has moved and possibly attacked, ending its turn.
    (new_cavern, new_units, false)
}

fn perform_attack_if_possible(
    acting_position: Position,
    target_positions: &BTreeSet<Position>,
    cavern: &Cavern,
    units: &Units,
) -> Option<(Cavern, Units)> {
    let adjacent_positions_to_acting = adjacent_positions(acting_position);
    let attackable_positions = target_positions & &adjacent_positions_to_acting;
    if attackable_positions.is_empty() {
        None
    } else {
        let acting_unit = *units.get(&acting_position).unwrap();
        Some(perform_attack(
            acting_unit,
            &attackable_positions,
            cavern,
            units,
        ))
    }
}

fn perform_attack(
    acting_unit: Unit,
    attackable_positions: &BTreeSet<Position>,
    cavern: &Cavern,
    units: &Units,
) -> (Cavern, Units) {
    let mut new_cavern = cavern.clone();
    let mut new_units = units.clone();
    let target_position = *attackable_positions
        .iter()
        .min_by_key(|attackable_position| units.get(attackable_position).unwrap().hitpoints)
        .unwrap();
    let mut attacked_unit = new_units.remove(&target_position).unwrap();
    attacked_unit.hitpoints -= acting_unit.attack_power;
    if attacked_unit.hitpoints > 0 {
        new_units.insert(target_position, attacked_unit);
    } else {
        // The attacked unit died, so remove it both from the cavern and from the units.
        new_cavern.insert(target_position, Tile::Open);
    }
    (new_cavern, new_units)
}

fn perform_move(
    start_position: Position,
    in_range_positions: &BTreeSet<Position>,
    cavern: &Cavern,
    units: &Units,
) -> (Position, Cavern, Units) {
    let mut new_position = start_position;
    let mut new_cavern = cavern.clone();
    let mut new_units = units.clone();
    let chosen = in_range_positions
        .iter()
        .filter_map(|&in_range_position| {
            shortest_path(start_position, in_range_position, cavern)
                .map(|path| (in_range_position, path))
        })
        .min_by(
            |(position1, path1), (position2, path2)| match path1.len().cmp(&path2.len()) {
                Ordering::Equal => position1.cmp(position2),
                ordering => ordering,
            },
        );
    if let Some((_chosen_position, chosen_path)) = chosen {
        let first_step = chosen_path[0];
        let removed_tile = new_cavern.insert(start_position, Tile::Open).unwrap();
        let removed_unit = new_units.remove(&start_position).unwrap();
        new_position = first_step;
        new_cavern.insert(first_step, removed_tile);
        new_units.insert(first_step, removed_unit);
    }
    (new_position, new_cavern, new_units)
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
    while let Some(current) = queue.pop() {
        if current.position == to {
            return Some(current.path);
        }
        if visited.contains_key(&current.position) {
            continue;
        }
        visited.insert(current.position, current.path.clone());
        queue.extend(in_range_entries(current.position, &current.path, cavern))
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
            let expected = "201638";
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
            let expected = "4988";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_2() {
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
            let expected = "31284";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_3() {
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
            let expected = "3478";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_4() {
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
            let expected = "6474";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_5() {
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
            let expected = "1140";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
