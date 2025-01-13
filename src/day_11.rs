use std::collections::HashMap;
use itertools::Itertools;

pub fn part_1(input_file: &str) {
    let mut stones = parse_file(input_file);
    for _ in 0..25 {
        stones = blink(stones);
    }
    println!("{} stones total after 25 blinks", stones.values().sum::<usize>());
}

pub fn part_2(input_file: &str) {
    let mut stones = parse_file(input_file);
    for _ in 0..75 {
        stones = blink(stones);
    }
    println!("{} stones total after 75 blinks", stones.values().sum::<usize>());
}

fn parse_file(input_file: &str) -> HashMap<u64, usize> {
    let line = std::fs::read_to_string(input_file).expect("cannot read file");
    line.split_whitespace()
        .map(|number| number.parse::<u64>().expect("cannot parse number"))
        .counts()
}

fn blink(stones: HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut result = HashMap::new();
    for (number, count) in stones.iter() {
        let number_string = format!("{}", number);
        let new_numbers = if *number == 0 {
            vec![1]
        } else if number_string.len() % 2 == 0 {
            let half_len = number_string.len() / 2;
            vec![
                number_string[..half_len].parse::<u64>().unwrap(),
                number_string[half_len..].parse::<u64>().unwrap(),
            ]
        } else {
            vec![number * 2024]
        };
        for new_number in new_numbers {
            *result.entry(new_number).or_insert(0) += count;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use super::*;

    #[test]
    fn test_blink_short_example() {
        let stones = vec![0, 1, 10, 99, 999]
            .into_iter()
            .counts();
        let stones = blink(stones);
        assert_eq!(stones[&0], 1);
        assert_eq!(stones[&1], 2);
        assert_eq!(stones[&9], 2);
        assert_eq!(stones[&2024], 1);
        assert_eq!(stones[&2021976], 1);
    }

    #[test]
    fn test_blink_longer_example() {
        let stones = vec![125, 17]
            .into_iter()
            .counts();
        let stones = blink(stones);
        assert_eq!(stones[&253000], 1);
        assert_eq!(stones[&1], 1);
        assert_eq!(stones[&7], 1);

        let stones = blink(stones);
        assert_eq!(stones[&253], 1);
        assert_eq!(stones[&0], 1);
        assert_eq!(stones[&2024], 1);
        assert_eq!(stones[&14168], 1);

        let stones = blink(stones);
        assert_eq!(stones[&512072], 1);
        assert_eq!(stones[&1], 1);
        assert_eq!(stones[&20], 1);
        assert_eq!(stones[&24], 1);
        assert_eq!(stones[&28676032], 1);

        let stones = blink(stones);
        assert_eq!(stones[&512], 1);
        assert_eq!(stones[&72], 1);
        assert_eq!(stones[&2024], 1);
        assert_eq!(stones[&2], 2);
        assert_eq!(stones[&0], 1);
        assert_eq!(stones[&4], 1);
        assert_eq!(stones[&2867], 1);
        assert_eq!(stones[&6032], 1);

        let stones = blink(stones);
        assert_eq!(stones[&1036288], 1);
        assert_eq!(stones[&7], 1);
        assert_eq!(stones[&2], 1);
        assert_eq!(stones[&20], 1);
        assert_eq!(stones[&24], 1);
        assert_eq!(stones[&4048], 2);
        assert_eq!(stones[&1], 1);
        assert_eq!(stones[&8096], 1);
        assert_eq!(stones[&28], 1);
        assert_eq!(stones[&67], 1);
        assert_eq!(stones[&60], 1);
        assert_eq!(stones[&32], 1);

        let stones = blink(stones);
        assert_eq!(stones[&2097446912], 1);
        assert_eq!(stones[&14168], 1);
        assert_eq!(stones[&4048], 1);
        assert_eq!(stones[&2], 4);
        assert_eq!(stones[&0], 2);
        assert_eq!(stones[&4], 1);
        assert_eq!(stones[&40], 2);
        assert_eq!(stones[&48], 2);
        assert_eq!(stones[&2024], 1);
        assert_eq!(stones[&80], 1);
        assert_eq!(stones[&96], 1);
        assert_eq!(stones[&8], 1);
        assert_eq!(stones[&6], 2);
        assert_eq!(stones[&7], 1);
        assert_eq!(stones[&3], 1);
    }
}

