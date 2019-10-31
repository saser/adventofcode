use std::str::FromStr;

use base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day08)
}

struct Day08;

impl Solver for Day08 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let numbers = parse_input(input);
        let root = parse_tree(&numbers);
        match part {
            Part::One => {
                let sum = root.metadata_sum();
                Ok(sum.to_string())
            }
            Part::Two => {
                let sum = root.value_sum();
                Ok(sum.to_string())
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split(' ')
        .map(u64::from_str)
        .map(Result::unwrap)
        .collect()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn metadata_sum(&self) -> usize {
        let self_sum = self.metadata.iter().sum::<usize>();
        let children_sum = self.children.iter().map(Node::metadata_sum).sum::<usize>();
        self_sum + children_sum
    }

    fn value_sum(&self) -> usize {
        if self.children.is_empty() {
            return self.metadata_sum();
        }
        self.metadata
            .iter()
            .filter(|&&idx| idx != 0 && idx <= self.children.len())
            .map(|&idx| &self.children[idx - 1])
            .map(Node::value_sum)
            .sum()
    }
}

fn parse_tree(numbers: &[u64]) -> Node {
    let (root, _remaining) = parse_tree_aux(numbers);
    root
}

fn parse_tree_aux(numbers: &[u64]) -> (Node, &[u64]) {
    let num_children = numbers[0] as usize;
    let mut children = Vec::with_capacity(num_children);
    let num_metadata = numbers[1] as usize;
    let mut metadata = Vec::with_capacity(num_metadata);

    let mut child_numbers = &numbers[2..];
    for _ in 0..num_children {
        let (child, next_child_numbers) = parse_tree_aux(child_numbers);
        children.push(child);
        child_numbers = next_child_numbers;
    }
    for &data in &child_numbers[..num_metadata] {
        metadata.push(data as usize);
    }
    (Node { children, metadata }, &child_numbers[num_metadata..])
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/08").trim();
            let expected = "40908";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
            let expected = "138";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/08").trim();
            let expected = "25910";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
            let expected = "66";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
