use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Equation {
    test_value: i64,
    values: Vec<i64>,
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (test_values, values) = line.split_once(": ").unwrap();
            let test_value = test_values.parse().unwrap();
            let values = values.split(' ').map(|x| x.parse().unwrap()).collect();
            Equation { test_value, values }
        })
        .collect()
}

fn solve(equation: &Equation) -> bool {
    fn inner(result: i64, current: i64, values: &[i64]) -> bool {
        if current > result {
            // Temporary result cannot exceed final result,
            // since there's no subtract or divide operators.
            return false;
        }
        let Some((next, values)) = values.split_first() else {
            // All values used, do we have the right result now?
            return current == result;
        };
        // Try
        inner(result, current + next, values) || inner(result, current * next, values)
    }

    inner(equation.test_value, 0, &equation.values)
}

#[aoc(day7, part1)]
fn part1(input: &[Equation]) -> i64 {
    input
        .iter()
        .filter(|equation| solve(equation))
        .map(|equation| equation.test_value)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[Equation]) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/2024/day7.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
