use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use itertools::Itertools;

pub fn part_1(input_file: &str) {
    let map = parse_file(input_file);
    let regions = calculate_regions(&map);
    let fence_price: usize = regions.into_iter()
        .map(|region| region.area * region.fences)
        .sum();
    println!("Fence price: {}", fence_price);
}

pub fn part_2(input_file: &str) {
    let map = parse_file(input_file);
    let regions = calculate_regions(&map);
    let fence_price: usize = regions.into_iter()
        .map(|region| region.area * region.edges)
        .sum();
    println!("Fence price: {}", fence_price);
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

    let mut map = Map {
        points_by_plant: HashMap::new(),
        plants_at_point: HashMap::new(),
    };

    for (y, line) in lines.into_iter().enumerate() {
        for  (x, c) in line.chars().enumerate() {
            map.plants_at_point.insert(Point { x: x as i64, y: y as i64 }, c);
            map.points_by_plant.entry(c)
                .or_insert(Vec::new())
                .push(Point { x: x as i64, y: y as i64 });
        }
    }
    map
}

fn count_fences(region: &HashSet<Point>) -> usize {
    region.iter()
        .map(|point| {
            point.neighbours()
                .into_iter()
                .filter(|neighbour| !region.contains(neighbour))
                .count()
        })
        .sum()
}

fn count_edges(region: &HashSet<Point>) -> usize {
    let fences = region.iter()
        .flat_map(|point| {
            let diffs = vec![
                Point { x: -1, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 0, y: -1 },
                Point { x: 0, y: 1 },
            ];
            diffs.into_iter()
                .filter(|diff| !region.contains(&point.add(&diff)))
                .map(|diff| (diff, point.clone()))
        })
        .into_group_map();

    let mut edges_count = 0;
    for (fence_direction, locations) in fences {
        let groups = if fence_direction.y == 0 {
            locations.iter()
                .map(|location| (location.x, location.y))
                .into_group_map()
        } else {
            locations.iter()
                .map(|location| (location.y, location.x))
                .into_group_map()
        };
        let n = groups.into_iter()
            .map(|(_, values)| {
                if values.len() == 1 {
                    return 1;
                }
                values.into_iter()
                    .sorted()
                    .tuple_windows()
                    .map(|(value, next_value)| next_value - value)
                    .fold(1, |acc, diff| {
                        if diff == 1 {
                            acc
                        } else {
                            acc + 1
                        }
                    })
            })
            .sum::<usize>();
        edges_count += n;
    }
    edges_count
}

#[derive(Debug)]
struct Map {
    plants_at_point: HashMap<Point, char>,
    points_by_plant: HashMap<char, Vec<Point>>,
}

#[derive(Debug, Eq, PartialEq)]
struct RegionStats {
    plant: char,
    area: usize,
    fences: usize,
    edges: usize,
}

fn calculate_regions(map: &Map) -> Vec<RegionStats> {
    map.points_by_plant.keys()
        .flat_map(|plant| calculate_regions_for_plant(*plant, map))
        .collect()
}

fn calculate_regions_for_plant(plant: char, map: &Map) -> Vec<RegionStats> {
    let points = map.points_by_plant.get(&plant).unwrap();
    let mut regions = points.iter()
        .map(|point| vec![point.clone()].into_iter().collect())
        .collect::<Vec<HashSet<Point>>>();
    loop {
        let (merge_occurred, merged_regions) = merge_regions_step(regions);
        regions = merged_regions;
        if !merge_occurred {
            break;
        }
    }

    regions.into_iter()
        .map(|region| {
            RegionStats {
                plant,
                area: region.len(),
                fences: count_fences(&region),
                edges: count_edges(&region),
            }
        })
        .collect()
}

fn merge_regions_step(mut regions: Vec<HashSet<Point>>) -> (bool, Vec<HashSet<Point>>) {
    let mut pointer = 0;
    let mut merge_occured = false;

    while pointer < regions.len() {
        let mut region = regions[pointer].clone();
        let neighbours = region.iter()
            .flat_map(|point| point.neighbours())
            .collect::<HashSet<Point>>();
        let mut other_pointer = pointer + 1;
        while other_pointer < regions.len() {
            let other_region = regions[other_pointer].clone();
            if neighbours.is_disjoint(&other_region) {
                other_pointer += 1;
            } else {
                merge_occured = true;
                region.extend(other_region.into_iter());
                regions.remove(other_pointer);
            }
        }
        regions[pointer] = region;
        pointer += 1;
    }
    (merge_occured, regions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_regions_example_1() {
        let map = parse_file("inputs/day-12-example-1.txt");
        let regions = calculate_regions(&map);
        assert!(regions.contains(&RegionStats { plant: 'A', area: 4, fences: 10, edges: 4 }));
        assert!(regions.contains(&RegionStats { plant: 'B', area: 4, fences: 8, edges: 4 }));
        assert!(regions.contains(&RegionStats { plant: 'C', area: 4, fences: 10, edges: 8 }));
        assert!(regions.contains(&RegionStats { plant: 'D', area: 1, fences: 4, edges: 4 }));
        assert!(regions.contains(&RegionStats { plant: 'E', area: 3, fences: 8, edges: 4 }));
    }

    #[test]
    fn test_calculate_regions_example_2() {
        let map = parse_file("inputs/day-12-example-2.txt");
        let regions = calculate_regions(&map);
        assert!(regions.contains(&RegionStats { plant: 'O', area: 21, fences: 36, edges: 20 }));
        let count = regions.iter()
            .filter(|&region| region == &RegionStats { plant: 'X', area: 1, fences: 4, edges: 4 })
            .count();
        assert_eq!(count, 4);
    }

    #[test]
    fn test_calculate_regions_example_3() {
        let map = parse_file("inputs/day-12-example-3.txt");
        let regions = calculate_regions(&map);
        assert!(regions.contains(&RegionStats { plant: 'R', area: 12, fences: 18, edges: 10 }));
        assert!(regions.contains(&RegionStats { plant: 'I', area: 4, fences: 8, edges: 4 }));
        assert!(regions.contains(&RegionStats { plant: 'C', area: 14, fences: 28, edges: 22 }));
        assert!(regions.contains(&RegionStats { plant: 'F', area: 10, fences: 18, edges: 12 }));
        assert!(regions.contains(&RegionStats { plant: 'V', area: 13, fences: 20, edges: 10 }));
        assert!(regions.contains(&RegionStats { plant: 'J', area: 11, fences: 20, edges: 12 }));
        assert!(regions.contains(&RegionStats { plant: 'C', area: 1, fences: 4, edges: 4 }));
        assert!(regions.contains(&RegionStats { plant: 'E', area: 13, fences: 18, edges: 8 }));
        assert!(regions.contains(&RegionStats { plant: 'I', area: 14, fences: 22, edges: 16 }));
        assert!(regions.contains(&RegionStats { plant: 'M', area: 5, fences: 12, edges: 6 }));
        assert!(regions.contains(&RegionStats { plant: 'S', area: 3, fences: 8, edges: 6 }));
    }
}