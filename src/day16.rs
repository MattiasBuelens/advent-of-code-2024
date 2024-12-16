use crate::util::{Direction, Vector2D};
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::astar;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Maze {
    start: Vector2D,
    end: Vector2D,
    walls: HashSet<Vector2D>,
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Maze {
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
    Maze { start, end, walls }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    pos: Vector2D,
    dir: Direction,
}

fn successors<'a>(state: &State, maze: &'a Maze) -> impl Iterator<Item = (State, i32)> + use<'a> {
    let &State { pos, dir } = state;
    // Move one step forward for 1 point
    let move_forward = std::iter::once(State {
        pos: pos + dir.step(),
        dir,
    })
    .filter(|state| !maze.walls.contains(&state.pos))
    .map(|state| (state, 1));
    // Turn clockwise or counterclockwise for 1000 points
    let turn = [dir.rotate_left(), dir.rotate_right()]
        .into_iter()
        .map(move |dir| State { pos, dir })
        .map(|state| (state, 1000));
    move_forward.chain(turn)
}

#[aoc(day16, part1)]
fn part1(maze: &Maze) -> i32 {
    let (_path, cost) = astar(
        &State {
            pos: maze.start,
            dir: Direction::E,
        },
        |state| successors(state, maze),
        |state| (state.pos - maze.end).manhattan_distance(),
        |state| state.pos == maze.end,
    )
    .expect("no path found");
    cost
}

#[aoc(day16, part2)]
fn part2(input: &Maze) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../examples/2024/day16-1.txt");
    const EXAMPLE2: &str = include_str!("../examples/2024/day16-2.txt");

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse(EXAMPLE1)), 7036);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse(EXAMPLE2)), 11048);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE1)), 0);
    }
}
