use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use itertools::Itertools;

pub fn part_1(input_file: &str) {
    let map = parse_file(input_file);
    let antinodes = map.find_antinodes();
    let result: HashSet<Point> = antinodes.into_iter()
        .map(|antinode| antinode.position)
        .filter(|position| map.contains(position))
        .collect();
    println!("Unique antinode positions: {}", result.len());
}

pub fn part_2(input_file: &str) {
    let map = parse_file(input_file);
    let antinodes = map.find_antinodes_part_2();
    let result: HashSet<Point> = antinodes.into_iter()
        .map(|antinode| antinode.position)
        .collect();
    println!("Unique antinode positions: {}", result.len());
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn subtract(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

struct Map {
    antennas: HashMap<char, Vec<Point>>,
    width: i64,
    height: i64,
}

impl Map {
    fn find_antinodes(&self) -> Vec<AntiNode> {
        self.antennas.iter()
            .flat_map(|(frequency, antennas)| find_antinodes(*frequency, antennas))
            .collect()
    }

    fn find_antinodes_part_2(&self) -> Vec<AntiNode> {
        let equations = self.antennas.iter()
            .flat_map(|(f, antennas)| {
                antennas.iter()
                    .tuple_combinations()
                    .map(|(left, right)| (f, find_equation(left, right)))
                    .collect_vec()
            })
            .collect_vec();
        let antinodes = (0..self.width)
            .cartesian_product(0..self.height)
            .map(|(x, y)| Point { x, y })
            .filter_map(|point| {
                let x = equations.iter().find(|(_f, equation)| equation.evaluate(&point));
                if let Some((x, _)) = x {
                    Some(AntiNode { position: point, frequency: **x})
                } else {
                    None
                }
            })
            .collect_vec();
        // for y in 0..self.height {
        //     for x in 0..self.width {
        //         if let Some(antinode) = antinodes.iter().find(|a| a.position == Point { x, y }) {
        //             print!("{}", antinode.frequency);
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
        return antinodes;
    }

    fn contains(&self, point: &Point) -> bool {
        point.x >= 0
            && point.x < self.width as i64
            && point.y >= 0
            && point.y < self.height as i64
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct AntiNode {
    position: Point,
    frequency: char,
}


fn find_equation(left: &Point, right: &Point) -> LineEquation {
    let m = (right.y as f64 - left.y as f64) / (right.x as f64 - left.x as f64);
    let b = right.y as f64 - (m * right.x as f64);
    LineEquation { m, b }
}

// y = mx + b
#[derive(Debug)]
struct LineEquation {
    m: f64,
    b: f64
}

impl LineEquation {
    fn evaluate(&self, point: &Point) -> bool {
        let expected = point.x as f64 * self.m + self.b;
        // Floating point precision errors mean we can't just check if they're equal
        // I'll come back to this at some point and do it a different way (figuring out the step for each line)
        return (point.y as f64 - expected).abs() < 0.01;
    }
}

fn find_antinodes(frequency: char, antennas: &Vec<Point>) -> Vec<AntiNode> {
    antennas.iter()
        .tuple_combinations()
        .flat_map(|(p1, p2)| {
            let diff = p2.subtract(p1);
            vec![
                p2.add(&diff),
                p1.subtract(&diff),
            ]
        })
        .map(|position| AntiNode { frequency, position })
        .collect()
}

fn parse_file(file_name: &str) -> Map {
    let file = File::open(file_name).expect("file not found");
    let lines: Result<Vec<_>, _> = io::BufReader::new(file).lines().collect();
    let lines = lines.expect("Error reading lines");
    let width = lines[0].len() as i64;
    let height = lines.len() as i64;
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            antennas.entry(c).or_insert(Vec::new())
                .push(Point { x: x as i64, y: y as i64 });
        }
    }
    Map {
        antennas,
        width,
        height,
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[test]
    fn test_parse_file() {
        let map = parse_file("inputs/day-8-example.txt");
        assert_eq!(map.antennas.len(), 2);
        assert_eq!(map.antennas[&'0'], vec![
            Point { x: 8, y: 1 },
            Point { x: 5, y: 2 },
            Point { x: 7, y: 3 },
            Point { x: 4, y: 4 },
        ]);
        assert_eq!(map.antennas[&'A'], vec![
            Point { x: 6, y: 5 },
            Point { x: 8, y: 8 },
            Point { x: 9, y: 9 },
        ]);
    }

    #[test]
    fn test_find_antinodes() {
        let map = parse_file("inputs/day-8-example.txt");
        let result: HashSet<Point> = map.find_antinodes()
            .into_iter()
            .map(|antinode| antinode.position)
            .filter(|position| map.contains(&position))
            .collect();
        assert_eq!(result.len(), 14);
    }

    #[test]
    fn test_find_antinodes_part_2() {
        let map = parse_file("inputs/day-8-example.txt");
        let result: HashSet<Point> = map.find_antinodes_part_2()
            .into_iter()
            .map(|antinode| antinode.position)
            .collect();
        assert_eq!(result.len(), 34);
    }

    #[rstest]
    #[case(Point { x: 0, y: 0 }, Point { x: 2, y: 2 }, 1f64, 0f64)]
    #[case(Point { x: 0, y: 0 }, Point { x: 2, y: 1 }, 0.5f64, 0f64)]
    #[case(Point { x: 0, y: 0 }, Point { x: 1, y: 2 }, 2f64, 0f64)]
    #[case(Point { x: 0, y: 0 }, Point { x: 2, y: -2 }, -1f64, 0f64)]
    #[case(Point { x: 0, y: 0 }, Point { x: -2, y: -2 }, 1f64, 0f64)]
    #[case(Point { x: 2, y: 2 }, Point { x: 4, y: 3 }, 0.5f64, 1f64)]
    fn test_equation(#[case] left: Point, #[case] right: Point, #[case] m: f64, #[case] b: f64) {
        let result = find_equation(&left, &right);
        assert_eq!(result.m, m);
        assert_eq!(result.b, b);
    }
}