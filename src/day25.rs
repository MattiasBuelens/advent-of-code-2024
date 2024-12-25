use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Columns = [u8; 5];

#[derive(Debug, Clone, Eq, PartialEq)]
struct Input {
    locks: Vec<Columns>,
    keys: Vec<Columns>,
}

#[aoc_generator(day25)]
fn parse(input: &str) -> Input {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for block in input.split("\n\n") {
        let mut lines = block.lines().peekable();
        // The locks are schematics that have the top row filled (#) and the bottom row empty (.);
        // the keys have the top row empty and the bottom row filled.
        let is_lock = lines.next().unwrap().starts_with("#");
        let mut columns = Columns::default();
        while let Some(line) = lines.next() {
            if lines.peek().is_none() {
                // Skip last line (always entirely # or .)
                break;
            }
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    columns[i] += 1;
                }
            }
        }
        if is_lock {
            locks.push(columns);
        } else {
            keys.push(columns);
        }
    }
    Input { locks, keys }
}

const MAX_HEIGHT: u8 = 5;

#[aoc(day25, part1)]
fn part1(input: &Input) -> usize {
    input
        .locks
        .iter()
        .cartesian_product(input.keys.iter())
        .filter(|(lock, key)| {
            lock.iter()
                .zip(key.iter())
                .all(|(lock_height, key_height)| lock_height + key_height <= MAX_HEIGHT)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/2024/day25.txt");

    #[test]
    fn parse_example() {
        assert_eq!(
            parse(EXAMPLE),
            Input {
                locks: vec![[0, 5, 3, 4, 3], [1, 2, 0, 5, 3]],
                keys: vec![[5, 0, 2, 1, 3], [4, 3, 4, 0, 2], [3, 0, 2, 0, 1]]
            }
        )
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3);
    }
}
