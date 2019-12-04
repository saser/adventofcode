use std::collections::{HashMap, VecDeque};
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
    let connections = parse_input(&input);
    match part {
        Part::One => Ok(find_group(&connections, 0).len().to_string()),
        Part::Two => Ok(find_all_groups(&connections).len().to_string()),
    }
}

fn parse_input(input: &str) -> HashMap<u64, Vec<u64>> {
    input.lines().map(parse_line).collect()
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

fn find_all_groups(connections: &HashMap<u64, Vec<u64>>) -> Vec<Vec<u64>> {
    let number_of_programs = connections.len();
    let mut groups: Vec<Vec<u64>> = Vec::new();
    let mut visited = vec![false; number_of_programs];
    let mut queue: VecDeque<u64> = (0..number_of_programs).map(|n| n as u64).collect();
    while let Some(unvisited_program) = queue.pop_front() {
        if visited[unvisited_program as usize] {
            continue;
        }

        let group = find_group(connections, unvisited_program as u64);
        for &program in &group {
            visited[program as usize] = true;
        }
        groups.push(group);
    }
    groups
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
    use crate::test;

    mod part1 {
        use super::*;

        test!(example, file "testdata/day12/ex", "6", part1);
        test!(actual, file "../../../inputs/2017/12", "141", part1);
    }

    mod part2 {
        use super::*;

        test!(example, file "testdata/day12/ex", "2", part2);
        test!(actual, file "../../../inputs/2017/12", "171", part2);
    }
}
