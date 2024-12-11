use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn part_1(input_file: &str) {
    let equations = parse_file(input_file);
    let result = equations
        .into_iter()
        .filter(|equation| !equation.find_solutions().is_empty())
        .map(|equation| equation.test_value)
        .sum::<u64>();
    println!("{}", result);
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Equation {
    test_value: u64,
    items: Vec<u64>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Operation {
    Multiply,
    Add,
}

impl Operation {
    fn all() -> Vec<Self> {
        vec![Self::Multiply, Self::Add]
    }

    fn evaluate(&self, left: u64, right: u64) -> u64 {
        match self {
            Self::Multiply => left * right,
            Self::Add => left + right,
        }
    }
}

impl Equation {
    fn find_solutions(&self) -> Vec<Vec<Operation>> {
        let candidates = vec![Operation::all(); self.items.len() - 1]
            .into_iter()
            .multi_cartesian_product()
            .collect_vec();
        candidates
            .into_iter()
            .filter(|operations| self.test_value == self.evaluate(operations))
            .collect()
    }

    fn evaluate(&self, operations: &Vec<Operation>) -> u64 {
        operations
            .iter()
            .enumerate()
            .fold(self.items[0], |total, (index, operation)| {
                operation.evaluate(total, self.items[index + 1])
            })
    }
}

fn parse_file(file_name: &str) -> Vec<Equation> {
    let file = File::open(file_name).expect("file not found");
    let lines: Result<Vec<_>, _> = io::BufReader::new(file).lines().collect();
    let lines = lines.expect("Error reading lines");
    lines
        .into_iter()
        .map(|line| parse_equation(&line))
        .collect()
}

fn parse_equation(line: &str) -> Equation {
    let (left, right) = line
        .split_once(":")
        .expect("Line not formatted as expected");
    let test_value = left.parse::<u64>().expect("Unable to parse test value");
    let items = right
        .trim()
        .split_whitespace()
        .map(|item| item.parse::<u64>().expect("Unable to parse item"))
        .collect();
    Equation { test_value, items }
}

#[cfg(test)]
mod tests {
    use crate::day_7::{parse_equation, Equation, Operation};
    use rstest::rstest;

    #[rstest]
    #[case("190: 10 19", Equation{ test_value: 190, items: vec![10, 19] })]
    #[case("3267: 81 40 27", Equation{ test_value: 3267, items: vec![81, 40, 27] })]
    #[case("83: 17 5", Equation{ test_value: 83, items: vec![17, 5] })]
    #[case("156: 15 6", Equation{ test_value: 156, items: vec![15, 6] })]
    #[case("7290: 6 8 6 15", Equation{ test_value: 7290, items: vec![6, 8, 6, 15] })]
    #[case("161011: 16 10 13", Equation{ test_value: 161011, items: vec![16, 10, 13] })]
    #[case("192: 17 8 14", Equation{ test_value: 192, items: vec![17, 8, 14] })]
    #[case("21037: 9 7 18 13", Equation{ test_value: 21037, items: vec![9, 7, 18, 13] })]
    #[case("292: 11 6 16 20", Equation{ test_value: 292, items: vec![11, 6, 16, 20] })]
    fn test_parse_line(#[case] line: &str, #[case] expected_result: Equation) {
        assert_eq!(parse_equation(line), expected_result);
    }

    #[rstest]
    #[case("190: 10 19", vec![ vec![Operation::Multiply] ])]
    #[case("3267: 81 40 27",
        vec![
            vec![Operation::Multiply, Operation::Add],
            vec![Operation::Add, Operation::Multiply]
        ])
    ]
    #[case("83: 17 5", vec![])]
    #[case("156: 15 6", vec![])]
    #[case("7290: 6 8 6 15", vec![])]
    #[case("161011: 16 10 13", vec![])]
    #[case("192: 17 8 14", vec![])]
    #[case("21037: 9 7 18 13", vec![])]
    #[case("292: 11 6 16 20", vec![ vec![Operation::Add, Operation::Multiply, Operation::Add] ])]
    fn test_find_solution(#[case] line: &str, #[case] solutions: Vec<Vec<Operation>>) {
        let equation = parse_equation(line);
        assert_eq!(equation.find_solutions(), solutions);
    }
}
