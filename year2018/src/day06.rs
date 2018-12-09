use base::grid::Point;
use base::{Part, Solver};

use std::collections::HashMap;
use std::str::FromStr;

type Coordinates = HashMap<char, Point>;
type Distances = HashMap<char, u64>;

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day06)
}

struct Day06;

impl Solver for Day06 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let coordinates = parse_input(input);
        let bb = BoundingBox::from_points(
            coordinates
                .values()
                .cloned()
                .collect::<Vec<Point>>()
                .as_slice(),
        );
        let minimal_distances = bounding_box_minimal_distances(&bb, &coordinates);
        match part {
            Part::One => Err("day 06 part 1 not yet implemented".to_string()),
            Part::Two => Err("day 06 part 2 not yet implemented".to_string()),
        }
    }
}

fn print_minimal_distances(minimal_distances: &HashMap<Point, Vec<char>>) {
    let bb = BoundingBox::from_points(
        minimal_distances
            .keys()
            .cloned()
            .collect::<Vec<Point>>()
            .as_slice(),
    );
    let adjusted_distances = minimal_distances.iter().map(|(&point, distances)| {
        let adjusted_point = point
            - Point {
                x: bb.x as i64,
                y: bb.y as i64,
            };
        (adjusted_point, distances)
    });
    let mut grid: Vec<Vec<char>> = vec![vec![' '; bb.width as usize]; bb.height as usize];
    for (point, distances) in adjusted_distances {
        let c = if distances.len() == 1 {
            distances[0]
        } else {
            '.'
        };
        grid[point.y as usize][point.x as usize] = c;
    }
    for row in &grid {
        let mut s = String::new();
        s.extend(row);
        println!("{}", s);
    }
}

fn parse_input(input: &str) -> Coordinates {
    let alphabet = (b'A'..=b'Z').map(char::from);
    let points = input.lines().map(Point::from_str).map(Result::unwrap);
    alphabet.zip(points).collect()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct BoundingBox {
    x: u64,
    y: u64,
    width: u64,
    height: u64,
}

impl BoundingBox {
    fn from_points(points: &[Point]) -> Self {
        let top_left_x = points.iter().map(|&point| point.x).min().unwrap();
        let top_left_y = points.iter().map(|&point| point.y).min().unwrap();
        let bottom_right_x = points.iter().map(|&point| point.x).max().unwrap();
        let bottom_right_y = points.iter().map(|&point| point.y).max().unwrap();
        BoundingBox {
            x: top_left_x as u64,
            y: top_left_y as u64,
            width: 1 + (bottom_right_x - top_left_x) as u64,
            height: 1 + (bottom_right_y - top_left_y) as u64,
        }
    }

    fn points(&self) -> Vec<Point> {
        (self.x..self.x + self.width)
            .flat_map(|x| {
                (self.y..self.y + self.height).map(move |y| Point {
                    x: x as i64,
                    y: y as i64,
                })
            })
            .collect()
    }
}

fn distances(point: &Point, coordinates: &Coordinates) -> Distances {
    coordinates
        .iter()
        .map(|(&c, &coord_point)| (c, point.manhattan_distance_to(coord_point)))
        .collect()
}

fn bounding_box_distances(
    bb: &BoundingBox,
    coordinates: &Coordinates,
) -> HashMap<Point, Distances> {
    bb.points()
        .iter()
        .map(|&point| (point, distances(&point, coordinates)))
        .collect()
}

fn minimal_distances(distances: &Distances) -> Vec<char> {
    let minimal_distance = *distances.values().min().unwrap();
    distances
        .keys()
        .cloned()
        .filter(|k| distances[k] == minimal_distance)
        .collect()
}

fn bounding_box_minimal_distances(
    bb: &BoundingBox,
    coordinates: &Coordinates,
) -> HashMap<Point, Vec<char>> {
    bounding_box_distances(bb, coordinates)
        .iter()
        .map(|(&point, distances)| (point, minimal_distances(distances)))
        .collect()
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
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9\
            ";
            let expected = "17";
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
