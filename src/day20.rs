use crate::util::{Direction, Vector2D};
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::*;
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

impl Maze {
    fn contains(&self, pos: &Vector2D) -> bool {
        (0..self.width).contains(&pos.x()) && (0..self.height).contains(&pos.y())
    }
}

fn successors(pos: Vector2D, maze: &Maze) -> Vec<Vector2D> {
    Direction::all()
        .into_iter()
        .map(|dir| pos + dir.step())
        .filter(|next_pos| maze.contains(next_pos) && !maze.walls.contains(next_pos))
        .collect()
}

/// Returns all positions in range for cheating,
/// i.e. a diamond pattern around `pos` with a maximum Manhattan distance of 2.
fn cheat_range(pos: Vector2D, cheat_duration: i32) -> impl Iterator<Item = Vector2D> {
    (-cheat_duration..=cheat_duration)
        .flat_map(move |dy| {
            let max_dx = cheat_duration - dy.abs();
            (-max_dx..=max_dx).map(move |dx| pos + Vector2D::new(dx, dy))
        })
        .filter(move |&x| x != pos)
}

fn find_cheats(maze: &Maze, cheat_duration: i32, min_saving: usize) -> usize {
    // Find the best path without cheating
    let path = bfs(
        &maze.start,
        |&pos| {
            let successors = successors(pos, maze);
            // There are no intersections (with 3 or more branches).
            assert!(successors.len() <= 2);
            successors
        },
        |&pos| pos == maze.end,
    )
    .expect("no solution found without cheating");
    // Map position along the path to its path index
    let path_index_by_pos = path
        .iter()
        .enumerate()
        .map(|(i, &pos)| (pos, i))
        .collect::<HashMap<_, _>>();
    // Try to cheat from any point on the path to a point further along
    let mut cheats = HashSet::<(Vector2D, Vector2D)>::new();
    for (i, &pos) in path.iter().enumerate() {
        for cheat_pos in cheat_range(pos, cheat_duration) {
            if let Some(j) = path_index_by_pos.get(&cheat_pos) {
                let cheat_time = (cheat_pos - pos).manhattan_distance() as usize;
                let saving = j.saturating_sub(i + cheat_time);
                if saving >= min_saving {
                    cheats.insert((pos, cheat_pos));
                }
            }
        }
    }
    cheats.len()
}

#[aoc(day20, part1)]
fn part1(maze: &Maze) -> usize {
    find_cheats(maze, 2, 100)
}

#[aoc(day20, part2)]
fn part2(maze: &Maze) -> usize {
    find_cheats(maze, 20, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/2024/day20.txt");

    #[test]
    fn test_cheat_range() {
        assert_eq!(
            cheat_range(Vector2D::new(0, 0), 2).collect::<Vec<_>>(),
            vec![
                Vector2D::new(0, -2),
                Vector2D::new(-1, -1),
                Vector2D::new(0, -1),
                Vector2D::new(1, -1),
                Vector2D::new(-2, 0),
                Vector2D::new(-1, 0),
                Vector2D::new(1, 0),
                Vector2D::new(2, 0),
                Vector2D::new(-1, 1),
                Vector2D::new(0, 1),
                Vector2D::new(1, 1),
                Vector2D::new(0, 2)
            ]
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(find_cheats(&parse(EXAMPLE), 2, 1), 44);
    }

    #[test]
    fn part2_example() {
        assert_eq!(find_cheats(&parse(EXAMPLE), 20, 50), 285);
    }
}
