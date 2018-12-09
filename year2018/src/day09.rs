use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day09)
}

struct Day09;

impl Solver for Day09 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        match part {
            Part::One => Err("day 09 part 1 not yet implemented".to_string()),
            Part::Two => Err("day 09 part 2 not yet implemented".to_string()),
        }
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
            let input = include_str!("../../inputs/2018/09").trim();
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "9 players; last marble is worth 25 points";
            let expected = "32";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "10 players; last marble is worth 1618 points";
            let expected = "8317";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "13 players; last marble is worth 7999 points";
            let expected = "146373";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_4() {
            let solver = get_solver();
            let input = "17 players; last marble is worth 1104 points";
            let expected = "2764";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_5() {
            let solver = get_solver();
            let input = "21 players; last marble is worth 6111 points";
            let expected = "54718";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_6() {
            let solver = get_solver();
            let input = "30 players; last marble is worth 5807 points";
            let expected = "37305";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/09").trim();
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
