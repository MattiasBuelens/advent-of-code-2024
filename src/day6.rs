use crate::util::{Direction, Vector2D};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Map {
    width: i32,
    height: i32,
    obstacles: HashSet<Vector2D>,
}

type Input = (Map, Vector2D);

#[aoc_generator(day6)]
fn parse(input: &str) -> Input {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    let mut obstacles = HashSet::new();
    let mut guard = None;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    obstacles.insert(Vector2D::new(x as i32, y as i32));
                }
                '.' => {}
                '^' => {
                    guard = Some(Vector2D::new(x as i32, y as i32));
                }
                c => panic!("unknown char: {}", c),
            };
        }
    }
    let map = Map {
        width,
        height,
        obstacles,
    };
    (map, guard.expect("no guard found"))
}

fn guard_path(map: &Map, mut guard: Vector2D) -> Option<HashSet<(Vector2D, Direction)>> {
    let (width, height) = (map.width, map.height);
    let mut dir = Direction::N;
    let mut path = HashSet::new();
    while (0..width).contains(&guard.x()) && (0..height).contains(&guard.y()) {
        let state = (guard, dir);
        if path.contains(&state) {
            return None;
        }
        path.insert(state);
        let next_guard = guard + dir.step();
        if map.obstacles.contains(&next_guard) {
            dir = dir.rotate_right();
        } else {
            guard = next_guard;
        }
    }
    Some(path)
}

#[aoc(day6, part1)]
fn part1((map, guard): &Input) -> usize {
    let path = guard_path(map, *guard).unwrap();
    let visited = path.into_iter().map(|(pos, _)| pos).collect::<HashSet<_>>();
    visited.len()
}

#[aoc(day6, part2)]
fn part2((map, guard): &Input) -> usize {
    let path = guard_path(map, *guard).unwrap();
    let mut loops = 0usize;
    let mut seen = HashSet::new();
    // New obstacle must be "seen" along the original path
    for (pos, dir) in path {
        let pos = pos + dir.step();
        if seen.contains(&pos) {
            // Already tested
            continue;
        }
        seen.insert(pos);
        if &pos == guard {
            // New obstacle cannot be at guard's initial position
            continue;
        }
        if map.obstacles.contains(&pos) {
            // New obstacle cannot be at existing obstacle
            continue;
        }
        let mut map = map.clone();
        map.obstacles.insert(pos);
        if guard_path(&map, *guard).is_none() {
            loops += 1;
        }
    }
    loops
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/2024/day6.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 6);
    }
}
