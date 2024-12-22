use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn secret_step(mut secret: u64) -> u64 {
    secret = mix_and_prune(secret * 64, secret);
    secret = mix_and_prune(secret / 32, secret);
    secret = mix_and_prune(secret * 2048, secret);
    secret
}

fn mix_and_prune(value: u64, secret: u64) -> u64 {
    (value ^ secret) % 16777216
}

fn secrets(mut secret: u64) -> impl Iterator<Item = u64> {
    std::iter::from_fn(move || {
        let current = secret;
        secret = secret_step(secret);
        Some(current)
    })
}

#[aoc(day22, part1)]
fn part1(input: &[u64]) -> u64 {
    input
        .iter()
        .map(|&secret| secrets(secret).nth(2000).unwrap())
        .sum()
}

fn prices(secret: u64) -> impl Iterator<Item = (u64, i64)> {
    secrets(secret)
        .map(|secret| secret % 10)
        .tuple_windows()
        .map(|(prev_price, price)| {
            let change = price as i64 - prev_price as i64;
            (price, change)
        })
}

type Change = [i64; 4];

fn price_changes(secret: u64) -> impl Iterator<Item = (u64, Change)> {
    prices(secret).tuple_windows().map(
        |((_, change1), (_, change2), (_, change3), (price, change4))| {
            (price, [change1, change2, change3, change4])
        },
    )
}

fn compute_total_bananas(change: &Change, all_changes: &[Vec<(u64, Change)>]) -> u64 {
    let mut total = 0;
    for monkey_changes in all_changes {
        if let Some((price, _)) = monkey_changes.iter().find(|(_, c)| change == c) {
            total += price;
        }
    }
    total
}

#[aoc(day22, part2)]
fn part2(input: &[u64]) -> u64 {
    let all_changes = input
        .iter()
        .map(|&secret| price_changes(secret).take(2000 + 1).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut most_bananas = 0;
    let mut seen = HashSet::<&Change>::new();
    for changes in &all_changes {
        for (_, change) in changes {
            if seen.contains(change) {
                continue;
            }
            let total = compute_total_bananas(change, &all_changes);
            seen.insert(change);
            if total > most_bananas {
                dbg!(total, change);
                most_bananas = total;
            }
        }
    }
    most_bananas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_example() {
        let values = secrets(123).skip(1).take(10).collect::<Vec<_>>();
        assert_eq!(
            values,
            vec![
                15887950, 16495136, 527345, 704524, 1553684, //
                12683156, 11100544, 12249484, 7753432, 5908254,
            ]
        );
    }

    #[test]
    fn prices_example() {
        let values = prices(123).take(9).collect::<Vec<_>>();
        assert_eq!(
            values,
            vec![
                (0, -3),
                (6, 6),
                (5, -1),
                (4, -1),
                (4, 0),
                (6, 2),
                (4, -2),
                (4, 0),
                (2, -2),
            ]
        );
    }

    #[test]
    fn part1_example() {
        let input = vec![1, 10, 100, 2024];
        assert_eq!(part1(&input), 37327623);
    }

    #[test]
    fn part2_example() {
        let input = vec![1, 2, 3, 2024];
        assert_eq!(part2(&input), 23);
    }
}
