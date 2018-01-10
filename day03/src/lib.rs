extern crate base;

use base::{Part, Solver};
use base::grid::*;
use std::collections::HashMap;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day03)
}

struct Day03;

impl Solver for Day03 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        // We need to trim the input, in case it contains a '\n' at the end.
        let number = parse_input(input.trim());
        match part {
            Part::One => Ok(distance_to_center(number).to_string()),
            Part::Two => Ok(first_after_number_by_summing(number).to_string()),
        }
    }
}

fn parse_input(input: &str) -> u64 {
    str::parse(input).unwrap()
}

struct SpiralTraveler {
    traveler: Traveler,
    current_layer: u64,
    steps_to_corners: Vec<u64>,
    to_corner: u64,
    to_next_layer: u64,
}

impl SpiralTraveler {
    fn new() -> SpiralTraveler {
        let mut traveler = Traveler::default();
        traveler.direction = Direction::East;
        SpiralTraveler {
            traveler: traveler,
            current_layer: 1,
            steps_to_corners: Vec::with_capacity(4),
            to_corner: 1,
            to_next_layer: 1,
        }
    }
}

impl Iterator for SpiralTraveler {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        self.traveler.step();

        self.to_next_layer -= 1;
        if self.to_next_layer == 0 {
            self.current_layer += 1;
            let layer_side = 2 * self.current_layer - 1;
            let steps = layer_side - 1;
            self.steps_to_corners
                .append(&mut vec![steps + 1, steps, steps, steps - 1]);
            self.to_next_layer = 4 * steps;
        }

        self.to_corner -= 1;
        if self.to_corner == 0 {
            self.traveler.turn(Turn::CounterClockwise);
            self.to_corner = self.steps_to_corners.pop().unwrap();
        }
        Some(self.traveler.pos)
    }
}

struct Spiral {
    spiral_traveler: SpiralTraveler,
    grid: HashMap<Point, u64>,
    pos: Point,
    value: u64,
    next_value_fun: Box<Fn(&Spiral) -> u64>,
}

impl Spiral {
    fn new(next_value_fun: Box<Fn(&Spiral) -> u64>) -> Spiral {
        let initial_pos = Point { x: 0, y: 0 };
        let mut initial_grid = HashMap::new();
        initial_grid.insert(initial_pos, 1);
        Spiral {
            spiral_traveler: SpiralTraveler::new(),
            grid: initial_grid,
            pos: initial_pos,
            value: 1,
            next_value_fun: next_value_fun,
        }
    }
}

impl Iterator for Spiral {
    type Item = (Point, u64);

    fn next(&mut self) -> Option<(Point, u64)> {
        self.pos = self.spiral_traveler.next().unwrap();
        self.value = (self.next_value_fun)(self);
        self.grid.insert(self.pos, self.value);
        Some((self.pos, self.value))
    }
}

fn first_after_number_by_summing(target_number: u64) -> u64 {
    let mut spiral = Spiral::new(Box::new(|spiral| {
        let current_pos = spiral.pos;
        let deltas = [
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 0, y: 1 },
            Point { x: -1, y: 1 },
            Point { x: -1, y: 0 },
            Point { x: -1, y: -1 },
            Point { x: 0, y: -1 },
            Point { x: 1, y: -1 },
        ];
        deltas
            .iter()
            .map(|&delta| current_pos + delta)
            .map(|pos| spiral.grid.get(&pos))
            .map(|op_val| op_val.unwrap_or(&0))
            .sum()
    }));
    spiral.find(|&(_, value)| value > target_number).unwrap().1
}

fn distance_to_center(target_number: u64) -> u64 {
    let mut spiral = Spiral::new(Box::new(|spiral| spiral.value + 1));
    let (pos, _) = spiral.find(|&(_, value)| value == target_number).unwrap();
    pos.manhattan_distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "1";
            let expected = "0";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "12";
            let expected = "3";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "23";
            let expected = "2";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_4() {
            let solver = get_solver();
            let input = "1024";
            let expected = "31";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }
}
