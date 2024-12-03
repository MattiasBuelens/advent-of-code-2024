use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Mul(i32, i32),
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .flat_map(|c| {
            let (_, [left, right]) = c.extract();
            Some(Instruction::Mul(left.parse().ok()?, right.parse().ok()?))
        })
        .collect::<Vec<_>>()
}

#[aoc(day3, part1)]
fn part1(input: &[Instruction]) -> i32 {
    input
        .iter()
        .map(|instruction| match instruction {
            Instruction::Mul(left, right) => left * right,
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &[Instruction]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
        r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 161);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
