use crate::util::Vector2D;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Behavior {
    button_a: Vector2D,
    button_b: Vector2D,
    prize: Vector2D,
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Behavior> {
    fn parse_button(s: &str) -> Vector2D {
        let s = s.strip_prefix("X+").unwrap();
        let (dx, dy) = s.split_once(", Y+").unwrap();
        Vector2D::new(dx.parse().unwrap(), dy.parse().unwrap())
    }

    fn parse_prize(s: &str) -> Vector2D {
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
    fn solve(&self) -> Option<(i32, i32)> {
        // Ax * a + Bx * b = Px
        // Ay * a + By * b = Py
        //
        // a = (Px - Bx * b) / Ax
        // (Ay / Ax) * (Px - Bx * b) + By * b = Py
        // Ay * Px - Ay * Bx * b + Ax * By * b = Ax * Py
        // (Ax * By - Ay * Bx) * b = (Ax * Py) - (Ay * Px)
        // determinant = Ax * By - Ay * Bx
        let determinant =
            self.button_a.x() * self.button_b.y() - self.button_a.y() * self.button_b.x();
        if determinant == 0 {
            // Button A and button B are collinear
            // Use the cheapest button (B) to reach the prize
            let num_b_presses = self.prize.x() / self.button_b.x();
            return if self.button_b * num_b_presses == self.prize {
                Some((0, num_b_presses))
            } else {
                // Prize is not collinear
                None
            };
        }
        // b = (Ax * Py - Ay * Px) / determinant
        let quotient = self.button_a.x() * self.prize.y() - self.button_a.y() * self.prize.x();
        if quotient % determinant != 0 {
            // Can't do half presses, this isn't Super Mario 64
            return None;
        }
        let num_b_presses = quotient / determinant;
        let num_a_presses =
            (self.prize.x() - self.button_b.x() * num_b_presses) / self.button_a.x();
        Some((num_a_presses, num_b_presses))
    }
}

#[aoc(day13, part1)]
fn part1(input: &[Behavior]) -> i32 {
    input
        .iter()
        .filter_map(Behavior::solve)
        .map(|solution| 3 * solution.0 + 1 * solution.1)
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Behavior]) -> i32 {
    todo!()
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
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
