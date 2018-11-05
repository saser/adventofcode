use base::{Part, Solver};
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day17)
}

struct Day17;

impl Solver for Day17 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let length = parse_input(input);
        match part {
            Part::One => {
                let (vec, final_position) = build_ring_buffer(2017, length);
                Ok(vec[final_position + 1].to_string())
            }
            Part::Two => Ok(value_after_zero(50_000_000, length).to_string()),
        }
    }
}

fn parse_input(input: &str) -> usize {
    usize::from_str(input).unwrap()
}

fn build_ring_buffer(final_value: usize, length: usize) -> (Vec<usize>, usize) {
    let mut vec = Vec::with_capacity(final_value + 1);
    vec.push(0);
    let mut current_position = 0;
    for i in 1..final_value + 1 {
        let index_to_insert = ((current_position + length) % i) + 1;
        vec.insert(index_to_insert, i);
        current_position = index_to_insert;
    }
    (vec, current_position)
}

fn value_after_zero(final_value: usize, length: usize) -> usize {
    let mut index_for_zero = 0;
    let mut value_after_zero = 0;
    let mut current_position = index_for_zero;
    for i in 1..final_value {
        let index_to_insert = ((current_position + length) % i) + 1;
        if index_to_insert <= index_for_zero {
            index_for_zero += 1;
        } else if index_to_insert == index_for_zero + 1 {
            value_after_zero = i;
        }
        current_position = index_to_insert;
    }
    value_after_zero
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "3";
            let expected = "638";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }
}
