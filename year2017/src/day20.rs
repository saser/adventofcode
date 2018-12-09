use base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day20)
}

struct Day20;

impl Solver for Day20 {
    fn solve(&self, _part: Part, _input: &str) -> Result<String, String> {
        Err("day 20 not yet implemented".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "put some input here";
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
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
