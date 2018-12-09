use lazy_static::lazy_static;
use regex::Regex;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use base::{Part, Solver};

type Dependencies = HashMap<char, HashSet<char>>;
type Dependants = HashMap<char, HashSet<char>>;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct RevChar(char);

impl PartialOrd for RevChar {
    fn partial_cmp(&self, other: &RevChar) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RevChar {
    fn cmp(&self, other: &RevChar) -> Ordering {
        match self.0.cmp(&other.0) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

pub fn get_solver() -> Box<Solver> {
    Box::new(Day07)
}

struct Day07;

impl Solver for Day07 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let (dependencies, dependants) = parse_input(input);
        match part {
            Part::One => Ok(run_until_completion(&dependencies, &dependants)),
            Part::Two => Err("day 07 part 2 not yet implemented".to_string()),
        }
    }
}

fn parse_input(input: &str) -> (Dependencies, Dependants) {
    let mut dependencies = HashMap::new();
    let mut dependants = HashMap::new();
    input
        .lines()
        .map(parse_instruction)
        .for_each(|(dependency, dependant)| {
            dependencies
                .entry(dependant)
                .or_insert_with(HashSet::new)
                .insert(dependency);
            dependants
                .entry(dependency)
                .or_insert_with(HashSet::new)
                .insert(dependant);
        });
    (dependencies, dependants)
}

fn parse_instruction(instruction: &str) -> (char, char) {
    lazy_static! {
        static ref INSTR_RE: Regex = Regex::new(
            r"Step (?P<dependency>\w) must be finished before step (?P<dependant>\w) can begin."
        )
        .unwrap();
    }
    let captures = INSTR_RE.captures(instruction).unwrap();
    let dependency = captures["dependency"].chars().collect::<Vec<char>>()[0];
    let dependant = captures["dependant"].chars().collect::<Vec<char>>()[0];
    (dependency, dependant)
}

fn run_until_completion(dependencies: &Dependencies, dependants: &Dependants) -> String {
    let mut s = String::new();
    let mut done = HashSet::new();
    let mut available = (&dependants.keys().cloned().collect::<HashSet<char>>()
        - &dependencies.keys().cloned().collect::<HashSet<char>>())
        .iter()
        .cloned()
        .map(RevChar)
        .collect::<BinaryHeap<RevChar>>();
    while !available.is_empty() {
        println!("available: {:?}", available);
        println!("done: {:?}", done);
        let next_step = available.pop().unwrap();
        if done.contains(&next_step.0) {
            continue;
        }
        s.push(next_step.0);
        done.insert(next_step.0);
        for (&c, local_dependencies) in dependencies.iter() {
            if local_dependencies.is_subset(&done) && !done.contains(&c) {
                available.push(RevChar(c));
            }
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/07");
            let expected = "MNQKRSFWGXPZJCOTVYEBLAHIUD";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.\
                ";
            let expected = "CABDFE";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/07");
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
