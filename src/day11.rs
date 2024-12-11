use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Either;
use nohash_hasher::IntMap;

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<u64> {
    input.split(' ').map(|x| x.parse().unwrap()).collect()
}

fn blink_single(stone: u64) -> Either<u64, (u64, u64)> {
    if stone == 0 {
        Either::Left(1)
    } else if count_digits(stone) % 2 == 0 {
        let divisor = 10u64.pow(count_digits(stone) / 2);
        Either::Right((stone / divisor, stone % divisor))
    } else {
        Either::Left(stone * 2024)
    }
}

fn blink(stones: &mut Vec<u64>) {
    let mut i = 0;
    while i < stones.len() {
        match blink_single(stones[i]) {
            Either::Left(stone) => {
                stones[i] = stone;
                i += 1
            }
            Either::Right((left, right)) => {
                stones[i] = left;
                stones.insert(i + 1, right);
                i += 2
            }
        }
    }
}

fn count_digits(x: u64) -> u32 {
    x.ilog10() + 1
}

fn blink_times(stones: &Vec<u64>, times: u32) -> Vec<u64> {
    let mut stones = stones.clone();
    for _ in 1..=times {
        blink(&mut stones);
    }
    stones
}

#[aoc(day11, part1)]
fn part1(stones: &Vec<u64>) -> usize {
    blink_times(stones, 25).len()
}

type StoneCounts = IntMap<u64, usize>;

fn to_counts(stones: &Vec<u64>) -> StoneCounts {
    let mut counts = StoneCounts::default();
    for &stone in stones {
        *counts.entry(stone).or_default() += 1;
    }
    counts
}

fn blink_counts(stones: StoneCounts) -> StoneCounts {
    let mut new_counts = StoneCounts::default();
    for (stone, count) in stones {
        match blink_single(stone) {
            Either::Left(stone) => {
                *new_counts.entry(stone).or_default() += count;
            }
            Either::Right((left, right)) => {
                *new_counts.entry(left).or_default() += count;
                *new_counts.entry(right).or_default() += count;
            }
        }
    }
    new_counts
}

fn blink_counts_times(stones: &Vec<u64>, times: usize) -> usize {
    let mut counts = to_counts(stones);
    for _ in 1..=times {
        counts = blink_counts(counts);
    }
    counts.values().sum()
}

#[aoc(day11, part2)]
fn part2(stones: &Vec<u64>) -> usize {
    blink_counts_times(stones, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let mut stones = parse("0 1 10 99 999");
        blink(&mut stones);
        assert_eq!(stones, vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }

    #[test]
    fn part1_example2() {
        let mut stones = parse("125 17");
        blink(&mut stones);
        assert_eq!(stones, vec![253000, 1, 7]);
        blink(&mut stones);
        assert_eq!(stones, vec![253, 0, 2024, 14168]);
        blink(&mut stones);
        assert_eq!(stones, vec![512072, 1, 20, 24, 28676032]);
        blink(&mut stones);
        assert_eq!(stones, vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);
        blink(&mut stones);
        assert_eq!(
            stones,
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );
        blink(&mut stones);
        assert_eq!(
            stones,
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );
    }

    #[test]
    fn part2_example1() {
        let stones = parse("0 1 10 99 999");
        assert_eq!(blink_counts_times(&stones, 1), 7);
    }

    #[test]
    fn part2_example2() {
        let stones = parse("125 17");
        assert_eq!(blink_counts_times(&stones, 6), 22);
    }
}
