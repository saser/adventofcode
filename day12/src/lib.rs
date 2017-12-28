extern crate base;

use base::{Part, Solver};
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day12)
}

struct Day12;

impl Solver for Day12 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let connections = parse_input(input);
        let group = find_group(&connections, 0);
        Ok(group.len().to_string())
    }
}

fn parse_input(input: &str) -> HashMap<u64, Vec<u64>> {
    input.lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let parts: Vec<&str> = line.split(" <-> ").collect();
    let program = u64::from_str(parts[0]).unwrap();
    let connected: Vec<u64> = parts[1]
        .split(", ")
        .map(u64::from_str)
        .map(Result::unwrap)
        .collect();
    (program, connected)
}

fn find_group(connections: &HashMap<u64, Vec<u64>>, included_program: u64) -> Vec<u64> {
    let mut queue: VecDeque<u64> = VecDeque::new();
    let mut group: Vec<u64> = Vec::new();
    let mut visited = vec![false; connections.len()];
    queue.push_back(included_program);

    while let Some(program) = queue.pop_front() {
        if visited[program as usize] {
            continue;
        }

        group.push(program);
        visited[program as usize] = true;
        let connected_programs = connections.get(&program).unwrap();
        for &connected_program in connected_programs {
            queue.push_back(connected_program);
        }
    }
    group
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5\
            ";
            let expected = "6";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5\
            ";
            let expected = "2";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
