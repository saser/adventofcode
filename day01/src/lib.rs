extern crate base;

use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day01)
}

struct Day01;

impl Solver for Day01 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let digits = parse_input(input);
        let offset = match part {
            Part::One => 1,
            Part::Two => digits.len() / 2,
        };
        Ok(sum_matching(&digits, offset).to_string())
    }
}

fn parse_input(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn sum_matching(digits: &[u32], offset: usize) -> u32 {
    let n = digits.len();
    let mut sum = 0;
    for (idx, &d) in digits.iter().enumerate() {
        let u = d;
        let v_idx = (idx + offset) % n;
        let v = digits[v_idx];
        if u == v {
            sum += u;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_1() {
        let solver = get_solver();
        let input = "1122";
        let expected = "3".to_string();
        assert_eq!(expected, solver.solve(Part::One, input).unwrap());
    }

    #[test]
    fn part_one_2() {
        let solver = get_solver();
        let input = "1111";
        let expected = "4".to_string();
        assert_eq!(expected, solver.solve(Part::One, input).unwrap());
    }

    #[test]
    fn part_one_3() {
        let solver = get_solver();
        let input = "1234";
        let expected = "0".to_string();
        assert_eq!(expected, solver.solve(Part::One, input).unwrap());
    }

    #[test]
    fn part_one_4() {
        let solver = get_solver();
        let input = "91212129";
        let expected = "9".to_string();
        assert_eq!(expected, solver.solve(Part::One, input).unwrap());
    }

    #[test]
    fn part_two_1() {
        let solver = get_solver();
        let input = "1212";
        let expected = "6".to_string();
        assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
    }

    #[test]
    fn part_two_2() {
        let solver = get_solver();
        let input = "1221";
        let expected = "0".to_string();
        assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
    }

    #[test]
    fn part_two_3() {
        let solver = get_solver();
        let input = "123425";
        let expected = "4".to_string();
        assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
    }

    #[test]
    fn part_two_4() {
        let solver = get_solver();
        let input = "123123";
        let expected = "12".to_string();
        assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
    }

    #[test]
    fn part_two_5() {
        let solver = get_solver();
        let input = "12131415";
        let expected = "4".to_string();
        assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
    }
}
