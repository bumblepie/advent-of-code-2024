use std::fs::File;
use std::io;
use std::io::BufRead;
use itertools::Itertools;
use regex::Regex;

pub fn part_1(input_file: &str) {
    let (list, other_list) = parse_file(input_file);
    let result = sum_diffs(&list, &other_list);
    println!("Sum of diffs: {}", result);
}

pub fn part_2(input_file: &str) {
    let (list, other_list) = parse_file(input_file);
    let result = similarity_score(&list, &other_list);
    println!("Similarity score: {}", result);
}

fn parse_file(input_file: &str) -> (Vec<u32>,Vec<u32>) {
    let regex = Regex::new(r"(?<left>\d+)\s+(?<right>\d+)").unwrap();
    let file = File::open(input_file).expect("file not found");
    let lines: Result<Vec<_>, _> = io::BufReader::new(file).lines().collect();
    let lines = lines.expect("Error reading lines");

    let pairs = lines.into_iter().map(|line| {
        if let Some(captures) = regex.captures(&line) {
            return (
                captures["left"].parse::<u32>().unwrap(),
                captures["right"].parse::<u32>().unwrap()
            );
        }
        panic!("Could not parse line {}", &line);
    });

    pairs.unzip()
}

fn sum_diffs(list: &Vec<u32>, other_list: &Vec<u32>) -> u32 {
    let list_sorted = list.iter()
        .sorted();
    let other_sorted = other_list.iter().sorted();
    list_sorted
        .zip(other_sorted)
        .map(|(left,right)| left.abs_diff(*right))
        .sum()
}

fn similarity_score(list: &Vec<u32>, other_list: &Vec<u32>) -> u32 {
    let counts = other_list.iter().counts();
    list.iter()
        .map(|item| {
            let count = *counts.get(item).unwrap_or(&0) as u32;
            item * count
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case("inputs/day-1-example.txt", (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]))]
    fn test_parsing(#[case] input_file: &str, #[case] expected_result: (Vec<u32>, Vec<u32>)) {
        let (list, other_list) = parse_file(input_file);
        for i in 0..expected_result.0.len() {
            assert_eq!(list[i], expected_result.0[i]);
            assert_eq!(other_list[i], expected_result.1[i]);
        }
    }

    #[rstest]
    #[case(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3], 11)]
    fn test_sum_diffs(#[case] list: Vec<u32>, #[case] other_list: Vec<u32>, #[case] expected_result: u32) {
        let result = sum_diffs(&list, &other_list);
        assert_eq!(result, expected_result);
    }

    #[rstest]
    #[case(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3], 31)]
    fn test_similarity_score(#[case] list: Vec<u32>, #[case] other_list: Vec<u32>, #[case] expected_result: u32) {
        let result = similarity_score(&list, &other_list);
        assert_eq!(result, expected_result);
    }
}