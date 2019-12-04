use std::collections::{HashMap, HashSet};
use std::io;
use std::str::FromStr;

use rayon::prelude::*;

use crate::base::grid::Point;
use crate::base::Part;

type Coordinates = HashMap<char, Point>;
type Distances = HashMap<char, u64>;

pub fn part1(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::One)
}

pub fn part2(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::Two)
}

fn solve(r: &mut dyn io::Read, part: Part) -> Result<String, String> {
    let mut input = String::new();
    r.read_to_string(&mut input).map_err(|e| e.to_string())?;
    let coordinates = parse_input(input.trim());
    let bb = BoundingBox::from_points(
        coordinates
            .values()
            .cloned()
            .collect::<Vec<Point>>()
            .as_slice(),
    );
    let distances = bounding_box_distances(&bb, &coordinates);
    match part {
        Part::One => {
            let minimal_distances = all_minimal_distances(&distances);
            let edge_points = bb.edge_points();
            let mut infinite_coordinates = HashSet::new();
            let mut closest_points = HashMap::new();
            for (point, coordinates) in minimal_distances.iter() {
                if coordinates.len() == 1 {
                    let c = coordinates[0];
                    closest_points.entry(c).or_insert_with(Vec::new).push(point);
                    if edge_points.contains(point) {
                        infinite_coordinates.insert(c);
                    }
                }
            }
            let max_area = closest_points
                .par_iter()
                .filter(|(c, _points)| !infinite_coordinates.contains(c))
                .map(|(_c, points)| points.len())
                .max()
                .unwrap();
            Ok(max_area.to_string())
        }
        Part::Two => {
            let limit = 10_000;
            let count = distances
                .par_iter()
                .map(|(_point, distance_map)| distance_map.values().cloned().collect::<Vec<u64>>())
                .map(|ds| ds.iter().sum::<u64>())
                .filter(|&sum| sum < limit)
                .count();
            Ok(count.to_string())
        }
    }
}

fn parse_input(input: &str) -> Coordinates {
    let alphabet = (b'A'..).map(char::from);
    let points = input.lines().map(Point::from_str).map(Result::unwrap);
    alphabet.zip(points).collect()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct BoundingBox {
    x_min: i64,
    y_min: i64,
    x_max: i64,
    y_max: i64,
}

impl BoundingBox {
    fn from_points(points: &[Point]) -> Self {
        let first_point = points[0];
        let mut x_min = first_point.x;
        let mut x_max = first_point.x;
        let mut y_min = first_point.y;
        let mut y_max = first_point.y;
        for &point in &points[1..] {
            x_min = x_min.min(point.x);
            y_min = y_min.min(point.y);
            x_max = x_max.max(point.x);
            y_max = y_max.max(point.y);
        }
        BoundingBox {
            x_min,
            y_min,
            x_max,
            y_max,
        }
    }

    fn height(&self) -> u64 {
        1 + (self.y_min - self.y_max).abs() as u64
    }

    fn width(&self) -> u64 {
        1 + (self.x_min - self.x_max).abs() as u64
    }

    fn points(&self) -> HashSet<Point> {
        let mut points = HashSet::with_capacity((self.width() * self.height()) as usize);
        for x in self.x_min..=self.x_max {
            for y in self.y_min..=self.y_max {
                points.insert(Point { x, y });
            }
        }
        points
    }

    fn edge_points(&self) -> HashSet<Point> {
        let mut points = Vec::with_capacity(2 * (self.width() + self.height()) as usize - 4);
        for x in self.x_min..self.x_max {
            points.push(Point { x, y: self.y_min });
            points.push(Point { x, y: self.y_max });
        }
        for y in self.y_min..self.y_max {
            points.push(Point { x: self.x_min, y });
            points.push(Point { x: self.x_max, y });
        }
        let mut set = HashSet::with_capacity(points.len());
        set.extend(points);
        set
    }
}

fn distances(point: &Point, coordinates: &Coordinates) -> Distances {
    coordinates
        .par_iter()
        .map(|(&c, &coord_point)| (c, point.manhattan_distance_to(coord_point)))
        .collect()
}

fn bounding_box_distances(
    bb: &BoundingBox,
    coordinates: &Coordinates,
) -> HashMap<Point, Distances> {
    bb.points()
        .par_iter()
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

fn all_minimal_distances(map: &HashMap<Point, Distances>) -> HashMap<Point, Vec<char>> {
    map.par_iter()
        .map(|(&point, distances)| (point, minimal_distances(distances)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(example, file "./testdata/day06/ex", "17", part1);
        test!(actual, file "../../../inputs/2018/06", "3687", part1);
    }

    mod part2 {
        use super::*;

        test!(actual, file "../../../inputs/2018/06", "40134", part2);
    }
}
