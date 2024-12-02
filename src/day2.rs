use crate::util::array_windows;
use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<Vec<i32>>;

#[aoc_generator(day2)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.split(' ').map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn is_safe(report: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    let mut valid_distance = true;
    for &[left, right] in array_windows::<i32, 2>(report) {
        increasing = increasing && left < right;
        decreasing = decreasing && left > right;
        valid_distance = valid_distance && (1..=3).contains(&(left - right).abs());
    }
    (increasing || decreasing) && valid_distance
}

#[aoc(day2, part1)]
fn part1(input: &Input) -> usize {
    input.iter().filter(|report| is_safe(report)).count()
}

#[aoc(day2, part2)]
fn part2(input: &Input) -> usize {
    todo!()
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
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
