use std::fs::File;
use std::io;
use std::io::BufRead;
use itertools::Itertools;

pub fn part_1(input_file: &str) {
    let reports = parse_file(input_file);
    let result = reports.into_iter().filter(is_safe)
        .count();
    println!("Safe reports: {}", result);
}

pub fn part_2(input_file: &str) {
    let reports = parse_file(input_file);
    let result = reports.into_iter().filter(is_safe_with_problem_dampener)
        .count();
    println!("Safe reports: {}", result);
}

type Report = Vec<i64>;

fn parse_file(file_name: &str) -> Vec<Report>
{
    let file = File::open(file_name).expect("file not found");
    let lines: Result<Vec<_>, _> = io::BufReader::new(file).lines().collect();
    let lines = lines.expect("Error reading lines");
    lines.into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|reading| {
                    reading.trim().parse()
                        .expect(&format!("Failed to parse number '{}'", reading))
                })
                .collect_vec()
        })
        .collect_vec()
}

fn is_safe(report: &Report) -> bool
{
    let diffs = report.iter()
        .tuple_windows::<(_, _)>()
        .map(|window| window.1 - window.0)
        .collect_vec();
    diffs.iter().all(|&diff| i64::abs(diff) > 0 && i64::abs(diff) < 4)
    && diffs.iter().map(|diff| diff.signum()).all_equal()
}

fn is_safe_with_problem_dampener(report: &Report) -> bool
{
    let diffs = report.iter()
        .tuple_windows::<(_, _)>()
        .map(|window| window.1 - window.0)
        .collect_vec();

    if diffs.is_empty() {
        return true;
    }

    let expected_sign = diffs[0].signum();

    for (index, diff) in diffs.iter().enumerate() {
        if i64::abs(*diff) < 1 || i64::abs(*diff) > 3 || diff.signum() != expected_sign {
            let mut indexes_to_try_removing = vec![index, index + 1];
            if index > 0 {
                indexes_to_try_removing.push(index - 1);
            }
            let dampened_reports = indexes_to_try_removing.into_iter()
                .map(|index_to_remove| {
                    let mut dampened_readings = report.clone();
                    dampened_readings.remove(index_to_remove);
                    dampened_readings
                }).collect_vec();
            return dampened_reports.iter().any(|report| is_safe(report));
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[test]
    fn test_parse_file()
    {
        let result = parse_file("inputs/day-2-example.txt");
        assert_eq!(result.len(), 6);
        assert_eq!(result[0], vec![7, 6, 4, 2, 1]);
        assert_eq!(result[1], vec![1, 2, 7, 8, 9]);
        assert_eq!(result[2], vec![9, 7, 6, 2, 1]);
        assert_eq!(result[3], vec![1, 3, 2, 4, 5]);
        assert_eq!(result[4], vec![8, 6, 4, 4, 1]);
        assert_eq!(result[5], vec![1, 3, 6, 7, 9]);
    }

    #[rstest]
    #[case(vec![7, 6, 4, 2, 1], true)]
    #[case(vec![1, 2, 7, 8, 9], false)]
    #[case(vec![9, 7, 6, 2, 1], false)]
    #[case(vec![1, 3, 2, 4, 5], false)]
    #[case(vec![8, 6, 4, 4, 1], false)]
    #[case(vec![1, 3, 6, 7, 9], true)]
    fn test_is_safe(#[case] report: Report, #[case] expected: bool)
    {
        assert_eq!(is_safe(&report), expected);
    }

    #[rstest]
    #[case(vec![7, 6, 4, 2, 1], true)]
    #[case(vec![1, 2, 7, 8, 9], false)]
    #[case(vec![9, 7, 6, 2, 1], false)]
    #[case(vec![1, 3, 2, 4, 5], true)]
    #[case(vec![8, 6, 4, 4, 1], true)]
    #[case(vec![1, 3, 6, 7, 9], true)]
    fn test_is_safe_with_problem_dampener(#[case] report: Report, #[case] expected: bool)
    {
        assert_eq!(is_safe_with_problem_dampener(&report), expected);
    }
}