use std::collections::{HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn part_1(input_file: &str) {
    let (map, position) = parse_file(input_file);
    let positions: HashSet<Point> = traverse(&map, &position);
    println!("Positions travelled: {}", positions.len());
}

pub fn part_2(input_file: &str) {
    let (map, position) = parse_file(input_file);
    let positions: HashSet<Point> = find_loops(&map, &position);
    println!("Positions we can put stuff to make a paradox: {}", positions.len());
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
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Direction {
    Up, Down, Left, Right,
}

impl Direction {
    fn vector(&self) -> Point {
        match self {
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }

    fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Position {
    point: Point,
    direction: Direction
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Map {
    obstacles: HashSet<Point>,
    width: usize,
    height: usize,
}

impl Map {
    fn contains(&self, point: &Point) -> bool {
        point.x >= 0
        && point.x < self.width as i64
        && point.y >= 0
        && point.y < self.height as i64
    }
}

fn parse_file(file_name: &str) -> (Map, Position) {
    let file = File::open(file_name).expect("file not found");
    let lines: Result<Vec<_>, _> = io::BufReader::new(file).lines().collect();
    let lines = lines.expect("Error reading lines");

    let height = lines.len();
    let width = lines[0].len();

    let mut obstacles = HashSet::new();
    let mut position = None;

    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                obstacles.insert(Point { x: x as i64, y: y as i64 });
            }
            if c == '^' {
                position = Some(Position {
                    point: Point { x: x as i64, y: y as i64 },
                    direction: Direction::Up,
                });
            }
        }
    }

    let map = Map {
        obstacles,
        width,
        height,
    };

    (map, position.unwrap())
}

fn traverse(map: &Map, position: &Position) -> HashSet<Point> {
    let mut result = HashSet::new();
    let mut current_position = position.clone();
    while map.contains(&current_position.point) {
        result.insert(current_position.point.clone());

        let next_point = current_position.point.add(&current_position.direction.vector());
        if map.obstacles.contains(&next_point) {
            current_position.direction = current_position.direction.next();
        } else {
            current_position.point = next_point;
        }
    }
    result
}

fn find_loops(map: &Map, starting_position: &Position) -> HashSet<Point> {
    let mut result = HashSet::new();
    let mut current_position = starting_position.clone();
    while map.contains(&current_position.point) {

        let next_point = current_position.point.add(&current_position.direction.vector());
        if map.obstacles.contains(&next_point) {
            current_position.direction = current_position.direction.next();
            continue;
        }

        if map.contains(&next_point) && next_point != starting_position.point {
            let mut changed_map = map.clone();
            changed_map.obstacles.insert(next_point.clone());
            if loop_is_possible(changed_map, starting_position.clone()) {
                result.insert(next_point.clone());
            }
        }

        current_position.point = next_point;
    }
    result
}

fn loop_is_possible(map: Map, loop_check_starting_position: Position) -> bool {
    let mut seen_positions = HashSet::new();
    let mut current_position = loop_check_starting_position.clone();

    while map.contains(&current_position.point) {
        if seen_positions.contains(&current_position) {
            return true;
        }

        seen_positions.insert(current_position.clone());

        let next_point = current_position.point.add(&current_position.direction.vector());
        if map.obstacles.contains(&next_point) {
            current_position.direction = current_position.direction.next();
        } else {
            current_position.point = next_point;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let (map, position) = parse_file("inputs/day-6-example.txt");
        let expected_obstacles = vec![
            Point { x: 4, y: 0 },
            Point { x: 9, y: 1 },
            Point { x: 2, y: 3 },
            Point { x: 7, y: 4 },
            Point { x: 1, y: 6 },
            Point { x: 8, y: 7 },
            Point { x: 0, y: 8 },
            Point { x: 6, y: 9 },
        ].into_iter().collect();
        assert_eq!(map.obstacles, expected_obstacles);
        let expected_position = Position {
            point: Point {
                x: 4,
                y: 6,
            },
            direction: Direction::Up,
        };
        assert_eq!(position, expected_position);
    }

    #[test]
    fn test_part_1() {
        let (map, position) = parse_file("inputs/day-6-example.txt");
        let positions = traverse(&map, &position);
        for y in 0..map.height {
            for x in 0..map.width {
                if map.obstacles.contains(&Point { x: x as i64, y: y as i64 }) {
                    print!("#");
                } else if position.point.x == x as i64 && position.point.y == y as i64 {
                    print!("^");
                } else if positions.contains(&Point { x: x as i64, y: y as i64 }) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        assert_eq!(positions.len(), 41);
    }

    #[test]
    fn test_part_2() {
        let (map, position) = parse_file("inputs/day-6-input.txt");
        let positions = find_loops(&map, &position);
        assert_eq!(positions.len(), 6);
    }
}