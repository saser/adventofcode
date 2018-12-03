use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day03)
}

struct Day03;

impl Solver for Day03 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        Err("day 03 not yet implemented".to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Claim {
    id: isize,
    start_coords: (isize, isize),
    size: (isize, isize),
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parsing {
        use super::*;

        #[test]
        fn single_digits() {
            let input = "#1 @ 1,3: 4x4";
            let expected = Claim {
                id: 1,
                start_coords: (1, 3),
                size: (4, 4),
            };
            assert_eq!(expected, Claim::from_str(input).unwrap());
        }

        #[test]
        fn multiple_digits() {
            let input = "#123 @ 19,443: 40x32";
            let expected = Claim {
                id: 123,
                start_coords: (19, 443),
                size: (40, 32),
            };
            assert_eq!(expected, Claim::from_str(input).unwrap());
        }
    }

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2\
            ";
            let expected = "4";
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
