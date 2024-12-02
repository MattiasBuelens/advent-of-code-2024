use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = Vec<Vec<i32>>;

#[aoc_generator(day2)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.split(' ').map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn is_safe<'a>(report: impl Iterator<Item = &'a i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    let mut valid_distance = true;
    for (&left, &right) in report.tuple_windows() {
        increasing = increasing && left < right;
        decreasing = decreasing && left > right;
        valid_distance = valid_distance && (1..=3).contains(&(left - right).abs());
    }
    (increasing || decreasing) && valid_distance
}

#[aoc(day2, part1)]
fn part1(input: &Input) -> usize {
    input.iter().filter(|report| is_safe(report.iter())).count()
}

fn is_safe_part2(report: &[i32]) -> bool {
    if is_safe(report.iter()) {
        // Safe without skipping any level
        return true;
    }
    (0..report.len()).any(|i| {
        // Skip the i-th level
        let (left, right) = report.split_at(i);
        let tolerated_report = left.iter().chain(right.iter().skip(1));
        // Check if this is now safe
        is_safe(tolerated_report)
    })
}

#[aoc(day2, part2)]
fn part2(input: &Input) -> usize {
    input.iter().filter(|report| is_safe_part2(report)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 4);
    }
}
