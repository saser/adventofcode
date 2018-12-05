use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(DayXX)
}

struct DayXX;

impl Solver for DayXX {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        match part {
            Part::One => Err("day XX part 1 not yet implemented".to_string()),
            Part::Two => Err("day XX part 2 not yet implemented".to_string()),
        }
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
