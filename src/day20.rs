use crate::util::{astar, astar_bag, Direction, Vector2D};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Maze {
    width: i32,
    height: i32,
    start: Vector2D,
    end: Vector2D,
    walls: HashSet<Vector2D>,
}

#[aoc_generator(day20)]
fn parse(input: &str) -> Maze {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    let mut start = Vector2D::zero();
    let mut end = Vector2D::zero();
    let mut walls = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Vector2D::new(x as i32, y as i32);
            match c {
                'S' => start = pos,
                'E' => end = pos,
                '#' => {
                    walls.insert(pos);
                }
                '.' => {}
                _ => panic!("invalid character {c}"),
            }
        }
    }
    Maze {
        width,
        height,
        start,
        end,
        walls,
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
struct State {
    // Current position
    pos: Vector2D,
    // Position where we started cheating
    start: Option<Vector2D>,
    // Position where we stopped cheating
    end: Option<Vector2D>,
    // Time remaining to cheat
    time_remaining: usize,
}

fn successors(state: State, maze: &Maze) -> impl Iterator<Item = State> + '_ {
    Direction::all().into_iter().filter_map(move |dir| {
        let mut next_state = state.clone();
        next_state.pos += dir.step();
        // Must always stay in bounds
        if !(0..maze.width).contains(&next_state.pos.x()) {
            return None;
        }
        if !(0..maze.height).contains(&next_state.pos.y()) {
            return None;
        }
        let mut cheating = state.start.is_some() && state.time_remaining > 0;
        // If we're currently cheating, advance the timer
        if cheating {
            next_state.time_remaining = next_state.time_remaining.saturating_sub(1);
            if next_state.time_remaining == 0 {
                next_state.end = Some(next_state.pos);
                cheating = false;
            }
        }
        if maze.walls.contains(&next_state.pos) {
            if state.start.is_none() && state.time_remaining > 0 {
                // Start cheating
                next_state.start = Some(state.pos);
            } else if !cheating {
                // Cannot cheat anymore
                return None;
            }
        }
        Some(next_state)
    })
}

fn find_cheats(maze: &Maze, min_saving: usize) -> usize {
    // Find the best path without cheating
    let (_, cost_without_cheating) = astar(
        &State {
            pos: maze.start,
            time_remaining: 0,
            ..Default::default()
        },
        |state| successors(state.clone(), maze).map(|state| (state, 1usize)),
        |state| (state.pos - maze.end).manhattan_distance() as usize,
        |state, _cost| state.pos == maze.end,
    )
    .expect("no solution found without cheating");
    // Find paths that improve over the best path by cheating
    let max_cost = cost_without_cheating - min_saving;
    let mut cheats = HashMap::<(Vector2D, Vector2D), usize>::default();
    let (solution, _) = astar_bag(
        &State {
            pos: maze.start,
            time_remaining: 1,
            ..Default::default()
        },
        |state| successors(state.clone(), maze).map(|state| (state, 1usize)),
        |state| (state.pos - maze.end).manhattan_distance() as usize,
        |state, cost| {
            if state.pos == maze.end {
                let cheat_cost = cheats
                    .entry((state.start.unwrap(), state.end.unwrap()))
                    .or_insert(usize::MAX);
                *cheat_cost = (*cheat_cost).min(cost);
                true
            } else {
                false
            }
        },
        Some(max_cost),
    )
    .expect("no solutions found");
    solution.for_each(|_| {});
    cheats.len()
}

#[aoc(day20, part1)]
fn part1(maze: &Maze) -> usize {
    find_cheats(maze, 100)
}

#[aoc(day20, part2)]
fn part2(maze: &Maze) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/2024/day20.txt");

    #[test]
    fn part1_example() {
        assert_eq!(find_cheats(&parse(EXAMPLE), 1), 44);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
