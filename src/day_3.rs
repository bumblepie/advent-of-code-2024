use itertools::Itertools;
use regex::Regex;

pub fn part_1(input_file: &str) {
    let instructions = parse_instructions(input_file);
    let result = instructions.into_iter()
        .filter_map(|instruction| match instruction {
            Instruction::Multiply(mult_instruction) => Some(mult_instruction),
            _ => None,
        })
        .map(evaluate_mult)
        .sum::<u64>();
    println!("Sum of multiplications: {}", result);
}

struct EvaluationState {
    total: u64,
    enabled: bool,
}

pub fn part_2(input_file: &str) {
    let instructions = parse_instructions(input_file);
    let result = instructions.into_iter()
        .fold(
            EvaluationState {
                total: 0,
                enabled: true,
            },
            evaluate
        );
    println!("Sum of multiplications: {}", result.total);
}

#[derive(Debug, Eq, PartialEq)]
struct MultiplyInstruction {
    left: u64,
    right: u64
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Multiply(MultiplyInstruction),
    Do,
    Dont,
}

fn evaluate(state: EvaluationState, instruction: Instruction) -> EvaluationState {
    match instruction {
        Instruction::Multiply(multiply_instruction) => {
            if state.enabled {
                EvaluationState {
                    total: state.total + evaluate_mult(multiply_instruction),
                    ..state
                }
            } else {
                state
            }
        },
        Instruction::Do => {
            EvaluationState {
                enabled: true,
                ..state
            }
        },
        Instruction::Dont => {
            EvaluationState {
                enabled: false,
                ..state
            }
        }
    }
}

fn evaluate_mult(instruction: MultiplyInstruction) -> u64 {
    instruction.left * instruction.right
}

fn parse_instructions(input_file: &str) -> Vec<Instruction> {
    let multiply_regex = Regex::new(r"mul\((?<left>\d+),(?<right>\d+)\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();

    let instruction_regex = Regex::new(&vec![&multiply_regex, &do_regex, &dont_regex]
        .into_iter()
        .map(|regex| regex.as_str())
        .collect_vec()
        .join("|")).unwrap();

    let input = std::fs::read_to_string(&input_file).expect("Unable to read file");
    instruction_regex.find_iter(&input)
        .map(|instruction| {
            if let Some(captures) = multiply_regex.captures(instruction.as_str()) {
                return Instruction::Multiply(MultiplyInstruction {
                    left: captures["left"].parse().unwrap(),
                    right: captures["right"].parse().unwrap()
                });
            }

            if do_regex.captures(instruction.as_str()).is_some() {
                return Instruction::Do;
            }

            if dont_regex.captures(instruction.as_str()).is_some() {
                return Instruction::Dont;
            }

            panic!("Unknown instruction: {:?}", instruction);
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day_3::*;

    #[test]
    fn test_parse_instructions() {
        let expected = vec![
            Instruction::Multiply(MultiplyInstruction { left: 2, right: 4 }),
            Instruction::Multiply(MultiplyInstruction { left: 5, right: 5 }),
            Instruction::Multiply(MultiplyInstruction { left: 11, right: 8 }),
            Instruction::Multiply(MultiplyInstruction { left: 8, right: 5 }),
        ];
        assert_eq!(parse_instructions("inputs/day-3-example.txt"), expected);
    }

    #[test]
    fn test_parse_instructions_part_2() {
        let expected = vec![
            Instruction::Multiply(MultiplyInstruction { left: 2, right: 4 }),
            Instruction::Dont,
            Instruction::Multiply(MultiplyInstruction { left: 5, right: 5 }),
            Instruction::Multiply(MultiplyInstruction { left: 11, right: 8 }),
            Instruction::Do,
            Instruction::Multiply(MultiplyInstruction { left: 8, right: 5 }),
        ];
        assert_eq!(parse_instructions("inputs/day-3-part-2-example.txt"), expected);
    }
}