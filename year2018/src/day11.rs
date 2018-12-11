use nalgebra::DMatrix;

use base::{Part, Solver};

type PowerGrid = DMatrix<i64>;
type RowSumGrid = DMatrix<i64>;
type ColSumGrid = DMatrix<i64>;
type StencilGrid = DMatrix<(usize, i64)>;

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day11)
}

struct Day11;

impl Solver for Day11 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let serial = input.parse::<i64>().unwrap();
        let power_grid = PowerGrid::from_fn(300, 300, power_level(serial));
        match part {
            Part::One => {
                let stencil_grid = stencil_grid(&power_grid, 3);
                let (x, y, (_size, _value)) = max_stencil_xy(&stencil_grid);
                Ok(format!("{},{}", x, y))
            }
            Part::Two => {
                let max_stencil_grid = max_stencil_grid(&power_grid);
                let (x, y, (size, _value)) = max_stencil_xy(&max_stencil_grid);
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

fn sum_grid(power_grid: &PowerGrid) -> (RowSumGrid, ColSumGrid) {
    let (nrows, ncols) = power_grid.shape();
    let mut row_sum_grid = RowSumGrid::zeros(nrows, ncols);
    let mut col_sum_grid = ColSumGrid::zeros(nrows, ncols);
    for i in (0..nrows).rev() {
        for j in (0..ncols).rev() {
            let value = power_grid[(i, j)];
            let below = (i + 1).min(nrows - 1);
            let right = (j + 1).min(ncols - 1);
            // println!("below: {}", below);
            // println!("right: {}", right);
            let below_sum = col_sum_grid[(below, j)];
            let right_sum = row_sum_grid[(i, right)];
            col_sum_grid[(i, j)] = value + below_sum;
            row_sum_grid[(i, j)] = value + right_sum;
        }
    }
    (row_sum_grid, col_sum_grid)
}

fn all_stencils_up_to(
    power_grid: &PowerGrid,
    row_sum_grid: &RowSumGrid,
    col_sum_grid: &ColSumGrid,
    (i, j): (usize, usize),
    max_size: usize,
) -> Vec<(usize, i64)> {
    let mut stencils = Vec::with_capacity(max_size);
    let mut previous_stencil = 0;
    for size in 1..=max_size {
        let row = i + size - 1;
        let col = j + size - 1;
        let corner_pos = (row, col);

        let row_sum_start = (row, j);
        let row_sum = row_sum_grid[row_sum_start] - row_sum_grid[corner_pos];

        let col_sum_start = (i, col);
        let col_sum = col_sum_grid[col_sum_start] - col_sum_grid[corner_pos];

        let corner = power_grid[corner_pos];
        let stencil = previous_stencil + row_sum + col_sum + corner;

        stencils.push((size, stencil));
        previous_stencil = stencil;
    }
    stencils
}

fn max_stencil_up_to(
    power_grid: &PowerGrid,
    row_sum_grid: &RowSumGrid,
    col_sum_grid: &ColSumGrid,
    (i, j): (usize, usize),
    max_size: usize,
) -> (usize, i64) {
    // let (row_sum_grid, col_sum_grid) = sum_grid(power_grid);
    all_stencils_up_to(power_grid, &row_sum_grid, &col_sum_grid, (i, j), max_size)
        .into_iter()
        .max_by_key(|&(_size, value)| value)
        .unwrap()
}

fn max_stencil(
    power_grid: &PowerGrid,
    row_sum_grid: &RowSumGrid,
    col_sum_grid: &ColSumGrid,
    (i, j): (usize, usize),
) -> (usize, i64) {
    let size = power_grid.nrows();
    let max_size = size - i.max(j);
    max_stencil_up_to(power_grid, row_sum_grid, col_sum_grid, (i, j), max_size)
}

fn stencil_grid(power_grid: &PowerGrid, size: usize) -> StencilGrid {
    let matrix_size = 300 - size + 1;
    let stencil = |i, j| {
        let sum = power_grid.slice((i, j), (size, size)).iter().sum();
        (size, sum)
    };
    StencilGrid::from_fn(matrix_size, matrix_size, stencil)
}

fn max_stencil_grid(power_grid: &PowerGrid) -> StencilGrid {
    let (nrows, ncols) = power_grid.shape();
    let (row_sum_grid, col_sum_grid) = sum_grid(power_grid);
    StencilGrid::from_fn(nrows, ncols, |i, j| {
        max_stencil(power_grid, &row_sum_grid, &col_sum_grid, (i, j))
    })
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
