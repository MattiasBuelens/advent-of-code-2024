use crate::util::{Direction, Vector2D};
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::*;
use std::collections::HashSet;

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

impl Maze {
    fn contains(&self, pos: &Vector2D) -> bool {
        (0..self.width).contains(&pos.x()) && (0..self.height).contains(&pos.y())
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
struct State {
    // Current position
    pos: Vector2D,
    // Position from where we cheated to where we ended up
    cheat: Option<(Vector2D, Vector2D)>,
}

fn successors(state: &State, maze: &Maze, can_cheat: bool) -> Vec<State> {
    Direction::all()
        .into_iter()
        .flat_map(move |dir| {
            let mut next_state = state.clone();
            next_state.pos += dir.step();
            // Must always stay in bounds
            if !maze.contains(&next_state.pos) {
                return vec![];
            }
            if maze.walls.contains(&next_state.pos) {
                if !can_cheat || state.cheat.is_some() {
                    // Can no longer cheat
                    return vec![];
                }
                // Cheat through the wall
                return Direction::all()
                    .into_iter()
                    .filter_map(|second_dir| {
                        if second_dir == dir.opposite() {
                            // Don't backtrack
                            return None;
                        }
                        let mut next_state = next_state.clone();
                        next_state.pos += dir.step();
                        // Must always stay in bounds
                        if !maze.contains(&next_state.pos) {
                            return None;
                        }
                        next_state.cheat = Some((state.pos, next_state.pos));
                        Some(next_state)
                    })
                    .collect();
            }
            vec![next_state]
        })
        .collect()
}

fn find_cheats(maze: &Maze, min_saving: usize) -> usize {
    // Find the best path without cheating
    let path = bfs(
        &State {
            pos: maze.start,
            ..Default::default()
        },
        |state| {
            let successors = successors(state, maze, false);
            // There are no intersections (with 3 or more branches).
            assert!(successors.len() <= 2);
            successors
        },
        |state| state.pos == maze.end,
    )
    .expect("no solution found without cheating");
    // Try to cheat from any point on the path to a point further along
    let mut cheats = HashSet::<(Vector2D, Vector2D)>::new();
    for (i, state) in path.iter().enumerate() {
        for next_state in successors(state, maze, true) {
            if let Some(cheat) = next_state.cheat {
                debug_assert_eq!(cheat.0, state.pos);
                if let Some(j) = path.iter().position(|x| x.pos == cheat.1) {
                    // Cheating adds 2 picoseconds, but we skip directly
                    // to index j along the original path.
                    let saving = j.saturating_sub(i + 2);
                    if saving >= min_saving {
                        cheats.insert(cheat);
                    }
                }
            }
        }
    }
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
