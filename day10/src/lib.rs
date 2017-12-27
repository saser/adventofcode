extern crate base;

use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day10)
}

struct Day10;

impl Solver for Day10 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        Err("day 10 not yet implemented".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let input = "3,4,1,5";

            let lengths = parse_input(input);
            let mut vector = initialize_vector(5);
            knot_hash(&mut vector, &lengths);
            let product = vector[0] * vector[1];

            assert_eq!(12, product);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "put some input here";
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
