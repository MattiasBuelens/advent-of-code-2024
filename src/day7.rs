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

fn solve(equation: &Equation, part2: bool) -> bool {
    fn inner(result: i64, current: i64, values: &[i64], part2: bool) -> bool {
        if current > result {
            // Temporary result cannot exceed final result,
            // since there's no subtract or divide operators.
            return false;
        }
        let Some((next, values)) = values.split_first() else {
            // All values used, do we have the right result now?
            return current == result;
        };
        // Try all operators
        inner(result, current + next, values, part2)
            || inner(result, current * next, values, part2)
            || (part2 && inner(result, concatenate(current, *next), values, part2))
    }

    inner(equation.test_value, 0, &equation.values, part2)
}

#[aoc(day7, part1)]
fn part1(input: &[Equation]) -> i64 {
    input
        .iter()
        .filter(|equation| solve(equation, false))
        .map(|equation| equation.test_value)
        .sum()
}

fn concatenate(left: i64, right: i64) -> i64 {
    let right_digits = right.ilog10() + 1;
    (left * 10_i64.pow(right_digits)) + right
}

#[aoc(day7, part2)]
fn part2(input: &[Equation]) -> i64 {
    input
        .iter()
        .filter(|equation| solve(equation, true))
        .map(|equation| equation.test_value)
        .sum()
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
        assert_eq!(part2(&parse(EXAMPLE)), 11387);
    }
}
