use aoc_runner_derive::{aoc, aoc_generator};

type Input = (Vec<i32>, Vec<i32>);

#[aoc_generator(day1)]
fn parse(input: &str) -> Input {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let (x, y) = line.split_once(' ').unwrap();
        left.push(x.trim().parse().unwrap());
        right.push(y.trim().parse().unwrap());
    }
    (left, right)
}

#[aoc(day1, part1)]
fn part1(input: &Input) -> i32 {
    let (mut left, mut right) = input.clone();
    left.sort();
    right.sort();
    left.iter().zip(right).map(|(x, y)| (x - y).abs()).sum()
}

#[aoc(day1, part2)]
fn part2(input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), todo!());
    }
}
