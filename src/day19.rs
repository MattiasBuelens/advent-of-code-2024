use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

struct Input {
    towels: Vec<String>,
    designs: Vec<String>,
}

#[aoc_generator(day19)]
fn parse(input: &str) -> Input {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").map(|s| s.to_string()).collect();
    let designs = designs.lines().map(|s| s.to_string()).collect();
    Input { towels, designs }
}

fn count_arrangements<'a>(
    design: &'a str,
    towels: &[String],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(&answer) = cache.get(design) {
        return answer;
    }
    let answer = towels
        .iter()
        .map(|towel| {
            if let Some(remainder) = design.strip_prefix(towel) {
                count_arrangements(remainder, towels, cache)
            } else {
                0
            }
        })
        .sum();
    cache.insert(design, answer);
    answer
}

#[aoc(day19, part1)]
fn part1(input: &Input) -> usize {
    let mut cache = HashMap::<&str, usize>::new();
    input
        .designs
        .iter()
        .filter(|design| count_arrangements(design, &input.towels, &mut cache) != 0)
        .count()
}

#[aoc(day19, part2)]
fn part2(input: &Input) -> usize {
    let mut cache = HashMap::<&str, usize>::new();
    input
        .designs
        .iter()
        .map(|design| count_arrangements(design, &input.towels, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/2024/day19.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 16);
    }
}
