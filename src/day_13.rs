use itertools::Itertools;
use regex::Regex;

pub fn part_1(input_file: &str) {
    let machines = parse_file(input_file);
    let tokens: u64 = machines.iter()
        .filter_map(Machine::solve)
        .map(|(a, b)| a * 3 + b)
        .sum();
    println!("Tokens Spent: {}", tokens);
}

pub fn part_2(input_file: &str) {
    let machines = parse_file(input_file);
    let tokens: u64 = machines.iter()
        .filter_map(Machine::solve_part_2)
        .map(|(a, b)| a * 3 + b)
        .sum();
    println!("Tokens Spent: {}", tokens);
}

#[derive(Debug, PartialEq, Eq)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: (u64, u64),
}

#[derive(Debug, PartialEq, Eq)]
struct Button {
    x: u64,
    y: u64,
}

impl Machine {
    fn solve(self: &Self) -> Option<(u64, u64)>
    {
        let b_result: f64 = (
            (self.prize.0 as f64 * self.button_a.y as f64)
                - (self.prize.1 as f64 * self.button_a.x as f64)
        ) / (
            (self.button_b.x as f64 * self.button_a.y as f64)
                - (self.button_b.y as f64 * self.button_a.x as f64)
        );
        if b_result <= 0.0 || b_result > 100.0 || b_result.fract() != 0.0 {
            return None;
        }
        let a_result = (self.prize.0 as f64 - self.button_b.x as f64 * b_result) / self.button_a.x as f64;
        if a_result <= 0.0 || a_result > 100.0 || a_result.fract() != 0.0 {
            return None;
        }
        Some((a_result as u64, b_result as u64))
    }

    fn solve_part_2(self: &Self) -> Option<(u64, u64)>
    {
        let prize_x = self.prize.0 as f64 + 10000000000000.0;
        let prize_y = self.prize.1 as f64 + 10000000000000.0;

        let b_result: f64 = (
            (prize_x as f64 * self.button_a.y as f64)
                - (prize_y as f64 * self.button_a.x as f64)
        ) / (
            (self.button_b.x as f64 * self.button_a.y as f64)
                - (self.button_b.y as f64 * self.button_a.x as f64)
        );
        if b_result <= 0.0 || b_result.fract() != 0.0 {
            return None;
        }
        let a_result = (prize_x as f64 - self.button_b.x as f64 * b_result) / self.button_a.x as f64;
        if a_result <= 0.0 || a_result.fract() != 0.0 {
            return None;
        }
        Some((a_result as u64, b_result as u64))
    }
}

fn parse_file(file_name: &str) -> Vec<Machine> {
    let input = std::fs::read_to_string(&file_name)
        .expect(&format!("Failed to open {}", file_name));
    input.split("\n\n")
        .map(|section| section.lines().collect_vec())
        .map(|section| parse_machine(&section))
        .collect()
}

fn parse_machine(lines: &Vec<&str>) -> Machine {
    let button_regex = Regex::new(r"Button (?<button>.+): X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap();

    let captures = button_regex.captures(lines[0])
        .expect(&format!("Failed to capture button {}", lines[0]));
    let button_a = Button {
        x: captures["x"].parse().expect("Failed to parse number"),
        y: captures["y"].parse().expect("Failed to parse number"),
    };

    let captures = button_regex.captures(lines[1])
        .expect(&format!("Failed to capture button {}", lines[1]));
    let button_b = Button {
        x: captures["x"].parse().expect("Failed to parse number"),
        y: captures["y"].parse().expect("Failed to parse number"),
    };

    let captures = prize_regex.captures(lines[2])
        .expect(&format!("Failed to capture prize {}", lines[2]));
    let prize = (
        captures["x"].parse().expect("Failed to parse number"),
        captures["y"].parse().expect("Failed to parse number"),
    );

    Machine {
        button_a,
        button_b,
        prize,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_machine() {
        let input = vec![
            "Button A: X+94, Y+34",
            "Button B: X+22, Y+67",
            "Prize: X=8400, Y=5400",
        ];
        let expected = Machine {
            button_a: Button { x: 94, y: 34 },
            button_b: Button { x: 22, y: 67 },
            prize: (8400, 5400),
        };
        assert_eq!(parse_machine(&input), expected);
    }

    #[test]
    fn test_part_machine_solve_example_1() {
        let machine = Machine {
            button_a: Button { x: 94, y: 34 },
            button_b: Button { x: 22, y: 67 },
            prize: (8400, 5400),
        };
        assert_eq!(machine.solve(), Some((80, 40)));
    }

    #[test]
    fn test_part_machine_solve_example_2() {
        let machine = Machine {
            button_a: Button { x: 26, y: 66 },
            button_b: Button { x: 67, y: 21 },
            prize: (12748, 12176),
        };
        assert_eq!(machine.solve(), None);
    }

    #[test]
    fn test_part_machine_solve_example_3() {
        let machine = Machine {
            button_a: Button { x: 17, y: 86 },
            button_b: Button { x: 84, y: 37 },
            prize: (7870, 6450),
        };
        assert_eq!(machine.solve(), Some((38, 86)));
    }

    #[test]
    fn test_part_machine_solve_example_4() {
        let machine = Machine {
            button_a: Button { x: 69, y: 23 },
            button_b: Button { x: 27, y: 71 },
            prize: (18641, 10279),
        };
        assert_eq!(machine.solve(), None);
    }

    #[test]
    fn test_part_machine_solve_example_1_part_2() {
        let machine = Machine {
            button_a: Button { x: 94, y: 34 },
            button_b: Button { x: 22, y: 67 },
            prize: (8400, 5400),
        };
        assert_eq!(machine.solve_part_2(), None);
    }

    #[test]
    fn test_part_machine_solve_example_2_part_2() {
        let machine = Machine {
            button_a: Button { x: 26, y: 66 },
            button_b: Button { x: 67, y: 21 },
            prize: (12748, 12176),
        };
        assert_eq!(machine.solve_part_2().is_some(), true);
    }

    #[test]
    fn test_part_machine_solve_example_3_part_2() {
        let machine = Machine {
            button_a: Button { x: 17, y: 86 },
            button_b: Button { x: 84, y: 37 },
            prize: (7870, 6450),
        };
        assert_eq!(machine.solve_part_2(), None);
    }

    #[test]
    fn test_part_machine_solve_example_4_part_2() {
        let machine = Machine {
            button_a: Button { x: 69, y: 23 },
            button_b: Button { x: 27, y: 71 },
            prize: (18641, 10279),
        };
        assert_eq!(machine.solve_part_2().is_some(), true);
    }
}