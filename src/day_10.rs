use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn part_1(input_file: &str) {
    let map = parse_file(input_file);
    let result: usize = map.trail_heads
        .iter()
        .map(|trailhead| score(trailhead, &map.heights))
        .sum();
    println!("Total: {}", result);
}

pub fn part_2(input_file: &str) {
    let map = parse_file(input_file);
    let result: usize = map.trail_heads
        .iter()
        .map(|trailhead| score_v2(trailhead, &map.heights))
        .sum();
    println!("Total: {}", result);
}

#[derive(Debug)]
struct Map {
    heights: HashMap<Point, u32>,
    trail_heads: Vec<Point>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn neighbours(&self) -> Vec<Point> {
        vec![
            Point { x: -1, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: -1 },
            Point { x: 0, y: 1 },
        ].into_iter()
            .map(|diff| self.add(&diff))
            .collect()
    }

    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn parse_file(file_name: &str) -> Map {
    let file = File::open(file_name).expect("file not found");
    let lines: Result<Vec<_>, _> = io::BufReader::new(file).lines().collect();
    let lines = lines.expect("Error reading lines");

    let mut heights = HashMap::new();
    let mut trail_heads = Vec::new();
    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let point = Point { x: x as i64, y: y as i64 };
            let height = c.to_digit(10).unwrap();
            heights.insert(point.clone(), height);
            if height == 0 {
                trail_heads.push(point);
            }
        }
    }
    Map {
        heights,
        trail_heads,
    }
}

fn score(trailhead: &Point, map: &HashMap<Point, u32>) -> usize {
    let mut explore_queue = VecDeque::new();
    explore_queue.push_back((trailhead.clone(), 1));
    let mut seen = HashSet::new();
    let mut peaks = 0;
    while let Some((point, next_height)) = explore_queue.pop_front() {
        if seen.contains(&point) {
            continue;
        }
        seen.insert(point.clone());
        if next_height == 10 {
            peaks += 1;
            continue;
        }
        for neighbour in point.neighbours() {
            if map.get(&neighbour) == Some(&next_height) && !seen.contains(&neighbour) {
                explore_queue.push_back((neighbour, next_height + 1));
            }
        }
    }
    peaks
}

fn score_v2(trailhead: &Point, map: &HashMap<Point, u32>) -> usize {
    let mut explore_queue = VecDeque::new();
    explore_queue.push_back((trailhead.clone(), 1));
    let mut peaks = 0;
    while let Some((point, next_height)) = explore_queue.pop_front() {
        if next_height == 10 {
            peaks += 1;
            continue;
        }
        for neighbour in point.neighbours() {
            if map.get(&neighbour) == Some(&next_height) {
                explore_queue.push_back((neighbour, next_height + 1));
            }
        }
    }
    peaks
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use super::*;

    #[test]
    fn test_example() {
        let map = parse_file("inputs/day-10-example.txt");
        let result = map.trail_heads
            .iter()
            .map(|trail_head| score(trail_head, &map.heights))
            .collect_vec();
        let expected = vec![
            5, 6, 5, 3, 1, 3, 5, 3, 5,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_v2() {
        let map = parse_file("inputs/day-10-example.txt");
        let result = map.trail_heads
            .iter()
            .map(|trail_head| score_v2(trail_head, &map.heights))
            .collect_vec();
        let expected = vec![
            20, 24, 10, 4, 1, 4, 5, 8, 5,
        ];
        assert_eq!(result, expected);
    }
}