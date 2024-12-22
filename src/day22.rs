use aoc_runner_derive::{aoc, aoc_generator};

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

fn secret_sequence(mut secret: u64) -> impl Iterator<Item = u64> {
    std::iter::from_fn(move || {
        secret = secret_step(secret);
        Some(secret)
    })
}

#[aoc(day22, part1)]
fn part1(input: &[u64]) -> u64 {
    input
        .iter()
        .map(|&secret| secret_sequence(secret).nth(2000 - 1).unwrap())
        .sum()
}

#[aoc(day22, part2)]
fn part2(input: &[u64]) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/2024/day22.txt");

    #[test]
    fn step_example() {
        let values = secret_sequence(123).take(10).collect::<Vec<_>>();
        assert_eq!(
            values,
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254,
            ]
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 37327623);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
