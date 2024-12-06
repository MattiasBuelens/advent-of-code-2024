use aoc_runner_derive::{aoc, aoc_generator};

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

#[aoc(day5, part2)]
fn part2((rules, updates): &Input) -> u32 {
    todo!()
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
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
