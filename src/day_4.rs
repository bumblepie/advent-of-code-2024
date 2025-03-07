use std::fs::File;
use std::io;
use std::io::BufRead;
use itertools::Itertools;

pub fn part_1(input_file: &str) {
    let file = File::open(input_file).expect("file not found");
    let lines: Result<Vec<_>, _> = io::BufReader::new(file).lines().collect();
    let lines = lines.expect("Error reading lines");
    let result = find_xmas(lines);
    println!("XMAS appears {} times", result);
}

pub fn part_2(input_file: &str) {
    let file = File::open(input_file).expect("file not found");
    let lines: Result<Vec<_>, _> = io::BufReader::new(file).lines().collect();
    let lines = lines.expect("Error reading lines");
    let result = find_cross_mas(lines);
    println!("XMAS appears {} times", result);
}

fn find_xmas(lines: Vec<String>) -> usize {
    let offsets = dbg!(vec![-1, 0, 1]
        .into_iter()
        .cartesian_product(vec![-1, 0, 1].into_iter())
        .filter(|offset| offset.0 != 0 || offset.1 != 0)
        .collect());

    let mut count = 0;

    for (row, line) in lines.iter().enumerate() {
        for (column, char) in line.chars().enumerate() {
            if char == 'X' {
                count += count_word_occurrences((row as i32, column as i32), &lines, &offsets, "MAS");
            }
        }
    }

    count
}

fn count_word_occurrences(position: (i32, i32), lines: &Vec<String>, offsets: &Vec<(i32, i32)>, searching_for: &str) -> usize {
    if searching_for.is_empty() {
        return 1;
    }

    let mut count = 0;
    for offset in offsets {
        let new_position = (position.0 as i32 + offset.0, position.1 as i32 + offset.1);
        if new_position.0 < 0
            || new_position.0 >= lines.len() as i32
            || new_position.1 < 0
            || new_position.1 >= lines[new_position.0 as usize].len() as i32 {
            // new position is outside of the bounds of the word grid
            continue;
        }

        if lines[new_position.0 as usize].chars().nth(new_position.1 as usize) == searching_for.chars().nth(0) {
            count += count_word_occurrences(new_position, lines, &vec![offset.clone()], &searching_for[1..]);
        }
    }
    count
}

fn find_cross_mas(lines: Vec<String>) -> usize {
    let offsets = vec![
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ];

    let mut count = 0;

    for (row, line) in lines.iter().enumerate() {
        for (column, char) in line.chars().enumerate() {
            if char == 'A' {
                let num_mas_found = offsets.iter()
                    .map(|offset| {
                        let position = (row as i32 - offset.0 * 2, column as i32 - offset.1 * 2);
                        count_word_occurrences(position, &lines, &vec![offset.clone()], "MAS")
                    })
                    .sum::<usize>();
                if num_mas_found == 2 {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let file = File::open("inputs/day-4-example.txt").expect("file not found");
        let lines: Result<Vec<_>, _> = io::BufReader::new(file).lines().collect();
        let lines = lines.expect("Error reading lines");
        let result = find_xmas(lines);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part_1() {
        let file = File::open("inputs/day-4-input.txt").expect("file not found");
        let lines: Result<Vec<_>, _> = io::BufReader::new(file).lines().collect();
        let lines = lines.expect("Error reading lines");
        let result = find_xmas(lines);
        assert_eq!(result, 2390);
    }

    #[test]
    fn test_part_2_example() {
        let file = File::open("inputs/day-4-example.txt").expect("file not found");
        let lines: Result<Vec<_>, _> = io::BufReader::new(file).lines().collect();
        let lines = lines.expect("Error reading lines");
        let result = find_cross_mas(lines);
        assert_eq!(result, 9);
    }
}
