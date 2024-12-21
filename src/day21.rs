use crate::util::Vector2D;
use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use std::collections::HashMap;

#[aoc_generator(day21)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

struct Keypad {
    buttons: HashMap<char, Vector2D>,
}

impl Keypad {
    fn new(buttons: &[&str]) -> Self {
        let buttons = buttons
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == ' ' {
                        None
                    } else {
                        Some((c, Vector2D::new(x as i32, y as i32)))
                    }
                })
            })
            .collect();
        Self { buttons }
    }

    fn solve(&self, code: &str) -> String {
        let mut moves = String::new();
        let mut current = *self.buttons.get(&'A').unwrap();
        for c in code.chars() {
            let next = *self.buttons.get(&c).unwrap();
            while current.x() < next.x() {
                *current.x_mut() += 1;
                moves.push('>');
                debug_assert!(self.buttons.values().any(|v| v == &current));
            }
            while current.y() > next.y() {
                *current.y_mut() -= 1;
                moves.push('^');
                debug_assert!(self.buttons.values().any(|v| v == &current));
            }
            while current.y() < next.y() {
                *current.y_mut() += 1;
                moves.push('v');
                debug_assert!(self.buttons.values().any(|v| v == &current));
            }
            while current.x() > next.x() {
                *current.x_mut() -= 1;
                moves.push('<');
                debug_assert!(self.buttons.values().any(|v| v == &current));
            }
            moves.push('A');
            debug_assert_eq!(current, next);
        }
        moves
    }
}

lazy_static! {
    static ref NUMERIC: Keypad = Keypad::new(&["789", "456", "123", " 0A"]);
    static ref DIRECTIONAL: Keypad = Keypad::new(&[" ^A", "<v>"]);
}

fn solve(code: &str) -> String {
    let code = NUMERIC.solve(code);
    let code = DIRECTIONAL.solve(&code);
    
    DIRECTIONAL.solve(&code)
}

#[aoc(day21, part1)]
fn part1(codes: &[String]) -> u64 {
    codes
        .iter()
        .map(|code| {
            let numeric = code.strip_suffix("A").unwrap().parse::<u64>().unwrap();
            let length = solve(code).len() as u64;
            dbg!(numeric, length);
            numeric * length
        })
        .sum()
}

#[aoc(day21, part2)]
fn part2(codes: &[String]) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/2024/day21.txt");

    #[test]
    fn part1_steps() {
        assert_eq!(NUMERIC.solve("029A"), "<A^A>^^AvvvA");
        assert_eq!(
            DIRECTIONAL.solve("<A^A>^^AvvvA"),
            "v<<A>>^A<A>AvA^<AA>Av<AAA>^A"
        );
        assert_eq!(
            DIRECTIONAL.solve("v<<A>>^A<A>AvA<^AA>A<vAAA>^A"),
            "v<A<AA>>^AvAA^<A>Av<<A>>^AvA^Av<A>^Av<<A>^A>AAvA^Av<<A>A>^AAAvA^<A>A"
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 126384);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
