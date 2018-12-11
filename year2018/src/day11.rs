use nalgebra::{DMatrix, MatrixMN};
// use rayon::prelude::*;
use typenum::U300;

use base::{Part, Solver};

type PowerGrid = MatrixMN<i64, U300, U300>;
type StencilGrid = DMatrix<(usize, i64)>;

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day11)
}

struct Day11;

impl Solver for Day11 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let serial = input.parse::<i64>().unwrap();
        let power_grid = PowerGrid::from_fn(power_level(serial));
        match part {
            Part::One => {
                let stencil_grid = stencil_grid(&power_grid, 3);
                let (x, y, (_size, _value)) = max_stencil_xy(&stencil_grid);
                Ok(format!("{},{}", x, y))
            }
            Part::Two => {
                let stencil_grid = max_stencil_grid(&power_grid);
                let (x, y, (size, _value)) = max_stencil_xy(&stencil_grid);
                Ok(format!("{},{},{}", x, y, size))
            }
        }
    }
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

fn all_stencils_up_to(
    power_grid: &PowerGrid,
    (i, j): (usize, usize),
    max_size: usize,
) -> Vec<(usize, i64)> {
    let mut stencils: Vec<(usize, i64)> = Vec::with_capacity(max_size);
    let mut previous_stencil = 0;
    for size in 1..=max_size {
        let row_start = i + size - 1;
        let row_length = size - 1;
        let row = power_grid.slice((row_start, j), (1, row_length));

        let col_start = j + size - 1;
        let col_length = size;
        let col = power_grid.slice((i, col_start), (col_length, 1));

        let row_sum = row.iter().sum::<i64>();
        let col_sum = col.iter().sum::<i64>();
        let stencil = previous_stencil + row_sum + col_sum;
        stencils.push((size, stencil));
        previous_stencil = stencil;
    }
    stencils
}

fn max_stencil_up_to(
    power_grid: &PowerGrid,
    (i, j): (usize, usize),
    max_size: usize,
) -> (usize, i64) {
    all_stencils_up_to(power_grid, (i, j), max_size)
        .into_iter()
        .max_by_key(|&(_size, value)| value)
        .unwrap()
}

fn max_stencil(power_grid: &PowerGrid, (i, j): (usize, usize)) -> (usize, i64) {
    let size = power_grid.nrows();
    let max_size = size - i.max(j);
    max_stencil_up_to(power_grid, (i, j), max_size)
}

fn stencil_grid(power_grid: &PowerGrid, size: usize) -> StencilGrid {
    let matrix_size = 300 - size + 1;
    let stencil = |i, j| {
        let sum = power_grid.slice((i, j), (size, size)).iter().sum();
        (size, sum)
    };
    StencilGrid::from_fn(matrix_size, matrix_size, stencil)
}

fn max_stencil_grid(power_grid: &PowerGrid) -> DMatrix<(usize, i64)> {
    let (nrows, ncols) = power_grid.shape();
    DMatrix::from_fn(nrows, ncols, |i, j| max_stencil(power_grid, (i, j)))
}

fn max_stencil_xy(stencil_grid: &StencilGrid) -> (usize, usize, (usize, i64)) {
    let (nrows, ncols) = stencil_grid.shape();
    let (mut max_i, mut max_j) = (0, 0);
    let (mut max_size, mut max_value) = (0, 0);
    for i in 0..nrows {
        for j in 0..ncols {
            let (size, value) = stencil_grid[(i, j)];
            if value > max_value {
                max_size = size;
                max_value = value;
                max_i = i;
                max_j = j;
            }
        }
    }
    let (x, y) = ij_to_xy((max_i, max_j));
    (x, y, (max_size, max_value))
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
            let expected = "233,36";
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
            let expected = "231,107,14";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "18";
            let expected = "90,269,16";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "42";
            let expected = "232,251,12";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
