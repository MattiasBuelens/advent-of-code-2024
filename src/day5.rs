use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Rule {
    before: u32,
    after: u32,
}

type Update = Vec<u32>;

type Input = (Vec<Rule>, Vec<Update>);

#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|line| {
            let (before, after) = line.split_once("|").unwrap();
            Rule {
                before: before.parse().unwrap(),
                after: after.parse().unwrap(),
            }
        })
        .collect();
    let updates = updates
        .lines()
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

fn is_update_valid(update: &Update, rules: &[Rule]) -> bool {
    for rule in rules {
        if let Some(before_index) = update.iter().position(|&x| x == rule.before) {
            if let Some(after_index) = update.iter().position(|&x| x == rule.after) {
                if before_index > after_index {
                    return false;
                }
            }
        }
    }
    true
}

#[aoc(day5, part1)]
fn part1((rules, updates): &Input) -> u32 {
    updates
        .iter()
        .filter(|update| is_update_valid(update, rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn fix_update(update: &Update, rules: &[Rule]) -> Update {
    debug_assert!(!is_update_valid(update, rules));
    let mut queue = VecDeque::from(update.to_vec());
    let mut update = Vec::new();
    while let Some(page) = queue.pop_front() {
        let mut index = update.len();
        // Add new page to the end
        update.insert(index, page);
        while !is_update_valid(&update, rules) {
            assert!(index > 0);
            // Move new page to the front
            update.swap(index - 1, index);
            index -= 1;
        }
    }
    debug_assert!(is_update_valid(&update, rules));
    update
}

#[aoc(day5, part2)]
fn part2((rules, updates): &Input) -> u32 {
    updates
        .iter()
        .filter(|update| !is_update_valid(update, rules))
        .map(|update| fix_update(update, rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/2024/day5.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 123);
    }
}
