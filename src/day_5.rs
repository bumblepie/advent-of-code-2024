use std::collections::HashMap;
use itertools::Itertools;

pub fn part_1(input_file: &str) {
    let (ordering_rules, page_updates) = parse_file(input_file);
    let result = page_updates.iter()
        .filter(|&update| is_correctly_ordered(update, &ordering_rules))
        .map(|update| {
            if update.len() % 2 == 0 {
                panic!("Update has an even number of pages!")
            }
            update[update.len() / 2]
        })
        .sum::<u64>();
    println!("Part 1: {}", result);
}

pub fn part_2(input_file: &str) {
    let (ordering_rules, page_updates) = parse_file(input_file);
    let badly_ordered_updates = page_updates.iter()
        .filter(|&update| !is_correctly_ordered(update, &ordering_rules))
        .collect_vec();
    let result = badly_ordered_updates.into_iter()
        .map(|update| reorder_update(update, &ordering_rules))
        .map(|update| {
            if update.len() % 2 == 0 {
                panic!("Update has an even number of pages!")
            }
            update[update.len() / 2]
        })
        .sum::<u64>();
    println!("Part 2: {}", result);
}

#[derive(Debug)]
struct OrderingRules {
    must_come_before: Vec<u64>,
    must_come_after: Vec<u64>,
}

impl OrderingRules {
    fn new() -> Self {
        Self {
            must_come_before: Vec::new(),
            must_come_after: Vec::new(),
        }
    }
}

fn parse_file(file_name: &str) -> (HashMap<u64, OrderingRules>, Vec<Vec<u64>>) {
    let full_input = std::fs::read_to_string(&file_name)
        .expect("Failed to read file");
    let (rules_part, updates_part) = full_input.split_once("\n\n")
        .expect("File not formatted as expected");

    let rules = rules_part.lines()
        .map(|line| line.split_once("|").expect("Rules line not formatted as expected"))
        .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
        .fold(
            HashMap::new(),
            |mut rules, (left, right)| {
                let entry = rules.entry(left).or_insert_with(OrderingRules::new);
                entry.must_come_before.push(right);
                let entry = rules.entry(right).or_insert_with(OrderingRules::new);
                entry.must_come_after.push(left);
                rules
            }
        );

    let updates = updates_part.lines()
        .map(|line|
            line.split(",")
                .map(|num| num.parse().unwrap())
                .collect_vec()
        )
        .collect();
    (rules, updates)
}

fn is_correctly_ordered(update: &Vec<u64>, rules: &HashMap<u64, OrderingRules>) -> bool {
    let update_indexes = update.iter()
        .enumerate()
        .map(|(index, num)| (*num, index))
        .collect::<HashMap<u64, usize>>();

    update.iter()
        .all(|update| {
            let my_index = update_indexes.get(update).unwrap();
            let default_rules = OrderingRules::new();
            let relevant_rules = rules.get(update).unwrap_or(&default_rules);
            if !relevant_rules.must_come_before.iter()
                .filter_map(|item| update_indexes.get(item))
                .all(|other_index| *my_index < *other_index) {
                return false;
            }
            if !relevant_rules.must_come_after.iter()
                .filter_map(|item| update_indexes.get(item))
                .all(|other_index| *my_index > *other_index) {
                return false;
            }
            return true;
        })
}

fn reorder_update(update: &Vec<u64>, rules: &HashMap<u64, OrderingRules>) -> Vec<u64> {
    let default_rules = OrderingRules::new();
    let mut result = Vec::new();
    for item in update {
        // check against any existing items
        // add in any spot possible?
        if result.is_empty() {
            result.push(*item);
            continue;
        }
        let relevant_rules = rules.get(item).unwrap_or(&default_rules);
        let min_index = relevant_rules.must_come_after.iter()
            .filter_map(|other_item| result.iter().position(|x| x == other_item))
            .map(|index| index + 1)
            .max()
            .unwrap_or(0);
        result.insert(min_index, *item);
    }
    result
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case(vec![75,47,61,53,29], true)]
    #[case(vec![97,61,53,29,13], true)]
    #[case(vec![75,29,13], true)]
    #[case(vec![75,97,47,61,53], false)]
    #[case(vec![61,13,29], false)]
    #[case(vec![97,13,75,29,47], false)]
    fn test_is_correctly_ordered(#[case] update: Vec<u64>, #[case] expected: bool) {
        let (ordering_rules, _) = parse_file("inputs/day-5-example.txt");
        assert_eq!(is_correctly_ordered(&update, &ordering_rules), expected)
    }

    #[test]
    fn parse_file_is_ok()
    {
        let (ordering_rules, updates) = parse_file("inputs/day-5-example.txt");
        dbg!(ordering_rules);
        dbg!(updates);
    }

    #[rstest]
    #[case(vec![75,97,47,61,53], vec![97,75,47,61,53])]
    #[case(vec![61,13,29], vec![61,29,13])]
    #[case(vec![97,13,75,29,47], vec![97,75,47,29,13])]
    fn test_reordering(#[case] update: Vec<u64>, #[case] expected: Vec<u64>) {
        let (ordering_rules, _) = parse_file("inputs/day-5-example.txt");
        assert_eq!(reorder_update(&update, &ordering_rules), expected)
    }

}