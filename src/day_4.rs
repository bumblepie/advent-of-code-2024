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

fn find_xmas(lines: Vec<String>) -> usize {
    get_lines_to_check(lines)
        .iter()
        .map(|line| count_xmas_occurrences(line.as_str()))
        .sum()
}

fn count_xmas_occurrences(line: &str) -> usize {
    let mut xmas_pointer = 0;
    let mut samx_pointer = 0;
    let mut count = 0;
    for c in line.chars() {
        match c {
            'X' => {
                xmas_pointer = 1;
                if samx_pointer == 3 {
                    count += 1;
                }
                samx_pointer = 0;
            },
            'M' => {
                xmas_pointer = if xmas_pointer == 1 { 2 } else { 0 };
                samx_pointer = if samx_pointer == 2 { 3 } else { 0 };
            },
            'A' => {
                xmas_pointer = if xmas_pointer == 2 { 3 } else { 0 };
                samx_pointer = if samx_pointer == 1 { 2 } else { 0 };
            },
            'S' => {
                if xmas_pointer == 3 {
                    count += 1;
                }
                xmas_pointer = 0;
                samx_pointer = 1;
            }
            _ => {
                xmas_pointer = 0;
                samx_pointer = 0;
            }
        }
    }
    count
}

fn transpose(lines: Vec<String>) -> Vec<String> {
    let max_line_length = lines.iter()
        .map(|line| line.len())
        .max()
        .unwrap();
    let mut transposed_lines: Vec<String> = vec![String::new(); max_line_length];
    for row in 0..lines.len() {
        let line = lines[row].as_str();
        for column in 0..lines[row].len() {
            if let Some(character) = line.chars().nth(column) {
                transposed_lines[column].push(character);
            }
        }
    }
    transposed_lines
}

fn get_lines_to_check(lines: Vec<String>) -> Vec<String> {
    let horizontal_lines = lines.clone();
    let vertical_lines = transpose(lines.clone());

    //Lines running diagonally from top left to bottom right
    let diagonal_lines_upper_right = transpose(lines.clone()
        .into_iter()
        .enumerate()
        .map(|(index, line)| line[index..].to_owned())
        .collect_vec());
    let mut diagonal_lines_bottom_left = transpose(lines.clone()
        .into_iter()
        .enumerate()
        .map(|(index, line)| {
            line[..index+1]
                .chars()
                .rev() // reverse to ensure diagonals line up when transposed
                .collect()
        })
        .collect_vec());
    diagonal_lines_bottom_left.remove(0); // The first diagonal is shared with diagonal_lines_upper_right

    // Lines running diagonally from top right to bottom left
    let diagonal_lines_upper_left = transpose(lines.clone()
        .into_iter()
        .enumerate()
        .map(|(index, line)| {
            line[..line.len()-index]
                .chars()
                .rev() // reverse to ensure diagonals line up when transposed
                .collect()
        })
        .collect_vec());
    let mut diagonal_lines_bottom_right = transpose(lines.clone()
        .into_iter()
        .enumerate()
        .map(|(index, line)| {
            line[(line.len()- index - 1)..].to_owned()
        })
        .collect_vec());
    diagonal_lines_bottom_right.remove(0); // The first diagonal is shared with diagonal_lines_upper_left

    horizontal_lines.into_iter()
        .chain(vertical_lines.into_iter())
        .chain(diagonal_lines_upper_right.into_iter())
        .chain(diagonal_lines_bottom_left.into_iter())
        .chain(diagonal_lines_upper_left.into_iter())
        .chain(diagonal_lines_bottom_right.into_iter())
        .collect()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[test]
    fn test_transpose()
    {
        /*
        ABC
        DEF
        GHI
         */
        let before = vec!["ABC".to_owned(), "DEF".to_owned(), "GHI".to_owned()];
        let expected = vec!["ADG".to_owned(), "BEH".to_owned(), "CFI".to_owned()];
        assert_eq!(transpose(before), expected);
    }

    #[test]
    fn test_get_lines_to_check()
    {
        /*
        ABCD
        EFGH
        IJKL
        MNOP
         */
        let before = vec!["ABCD".to_owned(), "EFGH".to_owned(), "IJKL".to_owned(), "MNOP".to_owned()];
        let expected = vec![
            "ABCD".to_owned(),
            "EFGH".to_owned(),
            "IJKL".to_owned(),
            "MNOP".to_owned(),
            "AEIM".to_owned(),
            "BFJN".to_owned(),
            "CGKO".to_owned(),
            "DHLP".to_owned(),
            "AFKP".to_owned(),
            "BGL".to_owned(),
            "CH".to_owned(),
            "D".to_owned(),
            "EJO".to_owned(),
            "IN".to_owned(),
            "M".to_owned(),
            "DGJM".to_owned(),
            "CFI".to_owned(),
            "BE".to_owned(),
            "A".to_owned(),
            "HKN".to_owned(),
            "LO".to_owned(),
            "P".to_owned()
        ];
        assert_eq!(get_lines_to_check(before), expected);
    }

    #[rstest]
    #[case("ADG", 0)]
    #[case("XMAS", 1)]
    #[case("SAMX", 1)]
    #[case("XMASAMX", 2)]
    #[case("XMASAMXMAS", 3)]
    #[case("XMASAMXMASAMX", 4)]
    #[case("XMAXXS", 0)]
    #[case("SAMSSX", 0)]
    fn test_count_xmas_occurrences(#[case] line: &str, #[case] expected: usize)
    {
        assert_eq!(count_xmas_occurrences(line), expected);
    }
}
