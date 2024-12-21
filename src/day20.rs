use crate::util::{Direction, Vector2D};
use aoc_runner_derive::{aoc, aoc_generator};
use indexmap::map::Entry;
use indexmap::IndexMap;
use nohash_hasher::IntMap;
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

fn successors(state: State, maze: &Maze, can_cheat: bool) -> impl Iterator<Item = State> + '_ {
    Direction::all().into_iter().filter_map(move |dir| {
        let state = state.clone();
        let mut next_state = state.clone();
        next_state.pos += dir.step();
        // Must always stay in bounds
        if !maze.contains(&next_state.pos) {
            return None;
        }
        if maze.walls.contains(&next_state.pos) {
            if !can_cheat || state.cheat.is_some() {
                // Can no longer cheat
                return None;
            }
            // Cheat through the wall
            next_state.cheat = Some((state.pos, next_state.pos));
        }
        Some(next_state)
    })
}

fn find_cheats(maze: &Maze, min_saving: usize) -> usize {
    // Find the best path without cheating
    let (_, cost_without_cheating) = astar(
        &State {
            pos: maze.start,
            ..Default::default()
        },
        |state| successors(state.clone(), maze, false).map(|state| (state, 1usize)),
        |state| (state.pos - maze.end).manhattan_distance() as usize,
        |state| state.pos == maze.end,
    )
    .expect("no solution found without cheating");
    // Find paths that improve over the best path by cheating
    // This is breadth-first search, but with a maximum path cost
    let max_cost = cost_without_cheating - min_saving;
    let mut parents = IndexMap::<State, usize>::new();
    parents.insert(
        State {
            pos: maze.start,
            ..Default::default()
        },
        0,
    );
    let mut cheats = HashSet::<(Vector2D, Vector2D)>::new();
    let mut cheats_by_savings = IntMap::<usize, usize>::default();
    let mut i = 0;
    while let Some((parent, &cost)) = parents.get_index(i) {
        if cost >= max_cost {
            // Path too long.
            break;
        }
        if parent.pos == maze.end {
            // Cheated our way to the end!
            cheats.insert(parent.cheat.unwrap());
            *cheats_by_savings
                .entry(cost_without_cheating - cost)
                .or_default() += 1;
        } else {
            for next in successors(parent.clone(), maze, true) {
                let cost = cost + 1;
                if let Some(cheat) = next.cheat {
                    if cheats.contains(&cheat) {
                        // Already seen this cheat before.
                        continue;
                    }
                }
                if let Entry::Vacant(entry) = parents.entry(next) {
                    entry.insert(cost);
                }
            }
        }
        i += 1;
    }
    // dbg!(&cheats_by_savings);
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
