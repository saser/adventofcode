use nalgebra::MatrixMN;
use typenum::U300;

use base::{Part, Solver};

type PowerGrid = MatrixMN<i64, U300, U300>;

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day11)
}

struct Day11;

impl Solver for Day11 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let serial = input.parse::<i64>().unwrap();
        let power_grid = PowerGrid::from_fn(power_level(serial));
        match part {
            Part::One => Err("day 11 part 1 not yet implemented".to_string()),
            Part::Two => Err("day 11 part 2 not yet implemented".to_string()),
        }
    }
}

fn xy_to_ij((x, y): (usize, usize)) -> (usize, usize) {
    // `i` denotes row, and thus depends on `y`. Likewise, `j` denotes column, and thus depends on `x`.
    let i = y - 1;
    let j = x - 1;
    (i, j)
}

fn ij_to_xy((i, j): (usize, usize)) -> (usize, usize) {
    // `x` denotes column, and thus depends on `j`. Likewise, `y` denotes row, and thus depends on `i`.
    let x = 1 + j;
    let y = 1 + i;
    (x, y)
}

fn power_level(serial: i64) -> impl Fn(usize, usize) -> i64 {
    move |i, j| {
        // `x` denotes column, and thus depends on `j`. Likewise, `y` denotes row, and thus depends on `i`.
        let (x, y) = ij_to_xy((i, j));
        let x = x as i64;
        let y = y as i64;
        let rack_id = x + 10;
        let mut power = y * rack_id;
        power += serial;
        power *= rack_id;
        power /= 100;
        power %= 10;
        power -= 5;
        power
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
            let input = include_str!("../../inputs/2018/11").trim();
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "18";
            let expected = "33,45";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "42";
            let expected = "21,61";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/11").trim();
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
