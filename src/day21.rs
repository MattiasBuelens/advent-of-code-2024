use crate::util::{Direction, Vector2D};
use aoc_runner_derive::{aoc, aoc_generator};
use bimap::BiHashMap;
use itertools::{Either, Itertools};
use lazy_static::lazy_static;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

#[aoc_generator(day21)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

type ButtonCosts = HashMap<(char, char), String>;

struct Keypad {
    buttons: BiHashMap<char, Vector2D>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    // The current position on this keypad
    pos: Vector2D,
    // Whether we've pushed the button on this keypad
    pushed: bool,
    // The last button pressed on the input keypad
    button: char,
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

    fn default_costs(&self) -> ButtonCosts {
        self.buttons
            .left_values()
            .cartesian_product(self.buttons.left_values())
            .map(|(&from, &to)| ((from, to), to.to_string()))
            .collect()
    }

    fn calculate_costs(&self, input: &ButtonCosts) -> ButtonCosts {
        self.buttons
            .left_values()
            .cartesian_product(self.buttons.left_values())
            .map(|(&from, &to)| ((from, to), self.get_path(input, from, to)))
            .collect()
    }

    fn get_path(&self, input: &ButtonCosts, from: char, to: char) -> String {
        let start = State {
            pos: *self.buttons.get_by_left(&from).unwrap(),
            pushed: false,
            // We always end with an "A" press on the input keypad
            button: 'A',
        };
        let goal = *self.buttons.get_by_left(&to).unwrap();
        let (path, cost) = dijkstra(
            &start,
            |state| self.successors(state.clone(), goal, input),
            |state| {
                // Must be on the right button and have pushed it
                state.pos == goal && state.pushed
            },
        )
        .expect("no path found");
        let mut buttons = String::with_capacity(cost);
        let mut prev_button = start.button;
        for state in path.into_iter().skip(1) {
            buttons.push_str(input.get(&(prev_button, state.button)).unwrap());
            prev_button = state.button;
        }
        debug_assert_eq!(buttons.len(), cost);
        buttons
    }

    fn successors<'a>(
        &'a self,
        state: State,
        goal: Vector2D,
        input: &'a ButtonCosts,
    ) -> impl Iterator<Item = (State, usize)> + 'a {
        let State {
            pos,
            pushed,
            button,
        } = state;
        if pos == goal {
            // Push the button!
            let cost = input.get(&(button, 'A')).unwrap().len();
            return Either::Left(std::iter::once((
                State {
                    pos,
                    pushed: true,
                    button: 'A',
                },
                cost,
            )));
        }
        debug_assert!(!pushed);
        let moves = Direction::all().into_iter().filter_map(move |dir| {
            let pos = pos + dir.step();
            if !self.buttons.contains_right(&pos) {
                // Must always be on a button
                return None;
            }
            let prev_button = button;
            let button = match dir {
                Direction::N => '^',
                Direction::E => '>',
                Direction::S => 'v',
                Direction::W => '<',
            };
            let cost = input.get(&(prev_button, button)).unwrap().len();
            Some((
                State {
                    pos,
                    pushed,
                    button,
                },
                cost,
            ))
        });
        Either::Right(moves)
    }

    fn solve(code: &str, input: &ButtonCosts) -> String {
        let mut buttons = String::new();
        let mut prev_button = 'A';
        for c in code.chars() {
            buttons.push_str(input.get(&(prev_button, c)).unwrap());
            prev_button = c;
        }
        buttons
    }
}

lazy_static! {
    static ref NUMERIC: Keypad = Keypad::new(&["789", "456", "123", " 0A"]);
    static ref DIRECTIONAL: Keypad = Keypad::new(&[" ^A", "<v>"]);
}

#[aoc(day21, part1)]
fn part1(codes: &[String]) -> usize {
    let costs = DIRECTIONAL.default_costs();
    let costs = DIRECTIONAL.calculate_costs(&costs);
    let costs = DIRECTIONAL.calculate_costs(&costs);
    let costs = NUMERIC.calculate_costs(&costs);

    codes
        .iter()
        .map(|code| {
            let numeric = code.strip_suffix("A").unwrap().parse::<usize>().unwrap();
            let buttons = Keypad::solve(code, &costs);
            numeric * buttons.len()
        })
        .sum()
}

#[aoc(day21, part2)]
fn part2(codes: &[String]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/2024/day21.txt");

    #[test]
    fn part1_steps() {
        let code = "029A";
        let costs = DIRECTIONAL.default_costs();
        let costs = DIRECTIONAL.calculate_costs(&costs);
        let costs = DIRECTIONAL.calculate_costs(&costs);
        let costs = NUMERIC.calculate_costs(&costs);

        assert_eq!(
            Keypad::solve(code, &costs).len(),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
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
