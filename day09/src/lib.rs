extern crate base;

use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day09)
}

struct Day09;

impl Solver for Day09 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        Err("day 09 not yet implemented".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "{}";
            let expected = "1";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "{{{}}}";
            let expected = "6";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "{{},{}}";
            let expected = "5";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_4() {
            let solver = get_solver();
            let input = "{{{},{},{{}}}}";
            let expected = "16";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_5() {
            let solver = get_solver();
            let input = "{<a>,<a>,<a>,<a>}";
            let expected = "1";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_6() {
            let solver = get_solver();
            let input = "{{<ab>},{<ab>},{<ab>},{<ab>}}";
            let expected = "9";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_7() {
            let solver = get_solver();
            let input = "{{<!!>},{<!!>},{<!!>},{<!!>}}";
            let expected = "9";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_8() {
            let solver = get_solver();
            let input = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
            let expected = "3";
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
