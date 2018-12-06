use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day05)
}

struct Day05;

impl Solver for Day05 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        match part {
            Part::One => {
                let mut chars = input.chars().collect();
                remove_destroyed(&mut chars, 0);
                Ok(chars.len().to_string())
            }
            Part::Two => Err("day 05 part 2 not yet implemented".to_string()),
        }
    }
}

fn remove_destroyed(chars: &mut Vec<char>, index: usize) {
    if chars.len() == 0 || index == chars.len() - 1 {
        return;
    }
    let c1 = chars[index];
    let c2 = chars[index + 1];
    if reacts(c1, c2) {
        // We remove from the same index twice, since the first remove causes all elements to shift
        // to the left, making `c2` now located at `index` insteaad of `index + 1`.
        chars.remove(index);
        chars.remove(index);
        let new_index = if index == 0 { 0 } else { index - 1 };
        remove_destroyed(chars, new_index);
    } else {
        remove_destroyed(chars, index + 1);
    }
}

fn reacts(c1: char, c2: char) -> bool {
    c1 != c2 && c1.to_ascii_uppercase() == c2.to_ascii_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example_1() {
            let solver = get_solver();
            let input = "aA";
            let expected = "0";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_2() {
            let solver = get_solver();
            let input = "abBA";
            let expected = "0";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_3() {
            let solver = get_solver();
            let input = "abAB";
            let expected = "4";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_4() {
            let solver = get_solver();
            let input = "aabAAB";
            let expected = "6";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example_5() {
            let solver = get_solver();
            let input = "dabAcCaCBAcCcaDA";
            let expected = "10";
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
