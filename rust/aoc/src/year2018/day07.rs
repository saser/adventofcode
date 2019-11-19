use lazy_static::lazy_static;
use regex::Regex;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::base::{Part, Solver};

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

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day07)
}

struct Day07;

impl Solver for Day07 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let (dependencies, dependants) = parse_input(input);
        let workers = 5;
        match part {
            Part::One => Ok(determine_order(&dependencies, &dependants)),
            Part::Two => Ok(seconds_with_workers(workers, &dependencies, &dependants).to_string()),
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

fn determine_order(dependencies: &Dependencies, dependants: &Dependants) -> String {
    let mut order = String::new();
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
        order.push(next_step.0);
        done.insert(next_step.0);
        for (&c, local_dependencies) in dependencies.iter() {
            if local_dependencies.is_subset(&done) && !done.contains(&c) {
                available.push(RevChar(c));
            }
        }
    }
    order
}

fn seconds_with_workers(workers: u64, dependencies: &Dependencies, dependants: &Dependants) -> u64 {
    let dependencies_keys = dependencies.keys().cloned().collect::<HashSet<char>>();
    let dependants_keys = dependants.keys().cloned().collect::<HashSet<char>>();
    let all_steps = &dependencies_keys | &dependants_keys;

    let mut done = HashSet::new();
    let mut current_time = 0;
    let initially_available = (&dependants_keys - &dependencies_keys)
        .iter()
        .cloned()
        .collect::<Vec<char>>();
    let mut events = initially_available
        .iter()
        .map(|&c| (duration(c), c))
        .collect::<Vec<(u64, char)>>();
    events.sort();
    let mut available = Vec::new();
    let mut available_workers = workers - events.len() as u64;

    while done.len() < all_steps.len() {
        let (new_time, completed_step) = events.remove(0);
        current_time = new_time;
        done.insert(completed_step);
        available_workers += 1;

        if dependants.contains_key(&completed_step) {
            let newly_available = dependants[&completed_step]
                .iter()
                .cloned()
                .filter(|&dependant| {
                    is_available(dependant, dependencies, &done) && !available.contains(&dependant)
                })
                .collect::<Vec<char>>();
            available.extend(newly_available);

            available.sort();
        }

        while available.len() > 0 && available_workers > 0 {
            let next_step = available.remove(0);
            available_workers -= 1;
            events.push((current_time + duration(next_step), next_step));
        }
        events.sort();
    }
    current_time
}

fn duration(c: char) -> u64 {
    (c as u64) - 4
}

fn is_available(c: char, dependencies: &Dependencies, done: &HashSet<char>) -> bool {
    !dependencies.contains_key(&c) || !done.contains(&c) && dependencies[&c].is_subset(done)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../../inputs/2018/07");
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
            let input = include_str!("../../../inputs/2018/07");
            let expected = "948";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
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
            // This expected answer is with the 60+ second duration rule, and 5 workers. The
            // example given in the description of the problem used durations equal to the order in
            // the alphabet (so 'A' = 1 s, 'B' = 2 s, ...) and used two workers. In that case,
            // the answer should be 15.
            let expected = "253";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
