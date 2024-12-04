use regex::Regex;

pub fn part_1(input_file: &str) {
    let instructions = parse_instructions(input_file);
    let result = instructions.into_iter()
        .map(evaluate)
        .sum::<u64>();
    println!("Sum of multiplications: {}", result);
}

#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    left: u64,
    right: u64
}

fn parse_instructions(input_file: &str) -> Vec<Instruction> {
    let regex = Regex::new(r"mul\((?<left>\d+),(?<right>\d+)\)").unwrap();
    let input = std::fs::read_to_string(&input_file).expect("Unable to read file");
    regex.captures_iter(&input)
        .map(|captures| {
            Instruction {
                left: captures["left"].parse().unwrap(),
                right: captures["right"].parse().unwrap()
            }
        })
        .collect()
}

fn evaluate(instruction: Instruction) -> u64 {
    instruction.left * instruction.right
}

#[cfg(test)]
mod tests {
    use crate::day_3::{parse_instructions, Instruction};

    #[test]
    fn test_parse_instructions() {
        let expected = vec![
            Instruction { left: 2, right: 4 },
            Instruction { left: 5, right: 5 },
            Instruction { left: 11, right: 8 },
            Instruction { left: 8, right: 5 },
        ];
        assert_eq!(parse_instructions("inputs/day-3-example.txt"), expected);
    }
}