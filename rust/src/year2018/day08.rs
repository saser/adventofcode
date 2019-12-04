use std::io;
use std::str::FromStr;

use crate::base::Part;

pub fn part1(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::One)
}

pub fn part2(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::Two)
}

fn solve(r: &mut dyn io::Read, part: Part) -> Result<String, String> {
    let mut input = String::new();
    r.read_to_string(&mut input).map_err(|e| e.to_string())?;
    let numbers = parse_input(input.trim());
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
    use crate::test;

    mod part1 {
        use super::*;

        test!(example, "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2", "138", part1);
        test!(actual, file "../../../inputs/2018/08", "40908", part1);
    }

    mod part2 {
        use super::*;

        test!(example, "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2", "66", part2);
        test!(actual, file "../../../inputs/2018/08", "25910", part2);
    }
}
