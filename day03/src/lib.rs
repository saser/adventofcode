extern crate base;

use base::{Part, Solver};
use base::grid::*;

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
            Part::Two => Err("part 2 not done yet".to_string()),
        }
    }
}

fn parse_input(input: &str) -> u64 {
    str::parse(input).unwrap()
}

fn distance_to_center(target_number: u64) -> u64 {
    let mut traveler = Traveler::default();
    // The traveler starts out facing north. Turn clockwise once, to make it face east.
    traveler.turn(Turn::Clockwise);
    // The spiral starts at 1, not 0.
    let mut number = 1;
    // The innermost layer, containing only 1, is layer 1.
    let mut current_layer = 1;
    let mut steps_to_corners = Vec::with_capacity(4);
    let mut to_corner = 1;
    let mut to_next_layer = 1;

    while number < target_number {
        traveler.step();
        number += 1;

        to_next_layer -= 1;
        if to_next_layer == 0 {
            current_layer += 1;
            let layer_side = 2 * current_layer - 1;
            let steps = layer_side - 1;
            steps_to_corners.append(&mut vec![steps + 1, steps, steps, steps - 1]);
            to_next_layer = 4 * steps;
        }

        to_corner -= 1;
        if to_corner == 0 {
            traveler.turn(Turn::CounterClockwise);
            to_corner = steps_to_corners.pop().unwrap();
        }
    }

    traveler.pos().manhattan_distance()
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
