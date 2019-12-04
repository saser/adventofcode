use std::collections::HashMap;
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
    let layers = parse_input(&input);
    match part {
        Part::One => {
            let total_severity: u64 = layers
                .iter()
                .map(|(&layer, &depth)| severity(layer, depth, 0))
                .sum();
            Ok(total_severity.to_string())
        }
        Part::Two => Ok(find_min_delay(&layers).to_string()),
    }
}

fn parse_input(input: &str) -> HashMap<u64, u64> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> (u64, u64) {
    let parts: Vec<&str> = line.split(": ").collect();
    let layer = u64::from_str(parts[0]).unwrap();
    let depth = u64::from_str(parts[1]).unwrap();
    (layer, depth)
}

fn detected_when_entering(picosecond: u64, depth: u64, delay: u64) -> bool {
    (picosecond + delay) % ((depth - 1) * 2) == 0
}

fn severity(layer: u64, depth: u64, delay: u64) -> u64 {
    if detected_when_entering(layer, depth, delay) {
        layer * depth
    } else {
        0
    }
}

fn any_detection_with_delay(layers: &HashMap<u64, u64>, delay: u64) -> bool {
    layers
        .iter()
        .any(|(&layer, &depth)| detected_when_entering(layer, depth, delay))
}

fn find_min_delay(layers: &HashMap<u64, u64>) -> u64 {
    (0..)
        .find(|&delay| !any_detection_with_delay(layers, delay))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(example, file "testdata/day13/ex", "24", part1);
        test!(actual, file "../../../inputs/2017/13", "2508", part1);
    }

    mod part2 {
        use super::*;

        test!(example, file "testdata/day13/ex", "10", part2);
        test!(actual, file "../../../inputs/2017/13", "3913186", part2);
    }
}
