use crate::util::Vector2D;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Behavior {
    button_a: Vector2D<i64>,
    button_b: Vector2D<i64>,
    prize: Vector2D<i64>,
}

const COST_A: i64 = 3;
const COST_B: i64 = 1;

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Behavior> {
    fn parse_button(s: &str) -> Vector2D<i64> {
        let s = s.strip_prefix("X+").unwrap();
        let (dx, dy) = s.split_once(", Y+").unwrap();
        Vector2D::new(dx.parse().unwrap(), dy.parse().unwrap())
    }

    fn parse_prize(s: &str) -> Vector2D<i64> {
        let s = s.strip_prefix("Prize: X=").unwrap();
        let (x, y) = s.split_once(", Y=").unwrap();
        Vector2D::new(x.parse().unwrap(), y.parse().unwrap())
    }

    input
        .split("\n\n")
        .map(|block| {
            let mut lines = block.lines();
            let button_a = parse_button(lines.next().unwrap().strip_prefix("Button A: ").unwrap());
            let button_b = parse_button(lines.next().unwrap().strip_prefix("Button B: ").unwrap());
            let prize = parse_prize(lines.next().unwrap());
            Behavior {
                button_a,
                button_b,
                prize,
            }
        })
        .collect()
}

impl Behavior {
    fn solve(&self) -> Option<i64> {
        // Ax * a + Bx * b = Px
        // Ay * a + By * b = Py
        //
        // a = (Px - Bx * b) / Ax
        // (Ay / Ax) * (Px - Bx * b) + By * b = Py
        // Ay * Px - Ay * Bx * b + Ax * By * b = Ax * Py
        // (Ax * By - Ay * Bx) * b = (Ax * Py) - (Ay * Px)
        // determinant = Ax * By - Ay * Bx
        let determinant = self.button_a.x().checked_mul(self.button_b.y()).unwrap()
            - self.button_a.y().checked_mul(self.button_b.x()).unwrap();
        if determinant == 0 {
            // Button A and button B are collinear
            // Use the cheapest button (B) to reach the prize
            let num_b_presses = self.prize.x() / self.button_b.x();
            return if self.check(0, num_b_presses) {
                Some(Self::cost(0, num_b_presses))
            } else {
                None
            };
        }
        // b = (Ax * Py - Ay * Px) / determinant
        let quotient_b = self.button_a.x().checked_mul(self.prize.y()).unwrap()
            - self.button_a.y().checked_mul(self.prize.x()).unwrap();
        if quotient_b % determinant != 0 {
            // Can't do half presses, this isn't Super Mario 64
            return None;
        }
        let num_b_presses = quotient_b / determinant;
        // a = (Px - Bx * b) / Ax
        let quotient_a = self.prize.x() - self.button_b.x().checked_mul(num_b_presses).unwrap();
        if quotient_a % self.button_a.x() != 0 {
            return None;
        }
        let num_a_presses = quotient_a / self.button_a.x();
        assert!(self.check(num_a_presses, num_b_presses));
        Some(Self::cost(num_a_presses, num_b_presses))
    }

    fn check(&self, num_a_presses: i64, num_b_presses: i64) -> bool {
        self.button_a * num_a_presses + self.button_b * num_b_presses == self.prize
    }

    fn cost(num_a_presses: i64, num_b_presses: i64) -> i64 {
        COST_A * num_a_presses + COST_B * num_b_presses
    }
}

#[aoc(day13, part1)]
fn part1(input: &[Behavior]) -> i64 {
    input.iter().filter_map(Behavior::solve).sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Behavior]) -> i64 {
    let new_input = input
        .iter()
        .cloned()
        .map(|mut behavior| {
            behavior.prize += Vector2D::new(10000000000000_i64, 10000000000000_i64);
            behavior
        })
        .collect::<Vec<_>>();
    part1(&new_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/2024/day13.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 480);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 875318608908);
    }
}
