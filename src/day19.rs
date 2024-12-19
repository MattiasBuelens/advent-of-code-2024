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

fn can_make<'a>(design: &'a str, towels: &[String], cache: &mut HashMap<&'a str, bool>) -> bool {
    if design.is_empty() {
        return true;
    }
    if let Some(&answer) = cache.get(design) {
        return answer;
    }
    let mut answer = false;
    for towel in towels {
        if let Some(remainder) = design.strip_prefix(towel) {
            if can_make(remainder, towels, cache) {
                answer = true;
                break;
            }
        }
    }
    cache.insert(design, answer);
    answer
}

#[aoc(day19, part1)]
fn part1(input: &Input) -> usize {
    let mut cache = HashMap::<&str, bool>::new();
    input
        .designs
        .iter()
        .filter(|design| can_make(design, &input.towels, &mut cache))
        .count()
}

#[aoc(day19, part2)]
fn part2(input: &Input) -> usize {
    todo!()
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
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
