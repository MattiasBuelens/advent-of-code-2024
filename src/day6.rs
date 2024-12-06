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

#[aoc(day6, part1)]
fn part1((map, guard): &Input) -> usize {
    let (width, height) = (map.width, map.height);
    let mut guard = *guard;
    let mut dir = Direction::N;
    let mut visited = HashSet::new();
    while (0..width).contains(&guard.x()) && (0..height).contains(&guard.y()) {
        visited.insert(guard);
        let next_guard = guard + dir.step();
        if map.obstacles.contains(&next_guard) {
            dir = dir.rotate_right();
        } else {
            guard = next_guard;
        }
    }
    visited.len()
}

fn is_loop(map: &Map, mut guard: Vector2D) -> bool {
    let (width, height) = (map.width, map.height);
    let mut dir = Direction::N;
    let mut visited = HashSet::new();
    while (0..width).contains(&guard.x()) && (0..height).contains(&guard.y()) {
        let state = (guard, dir);
        if visited.contains(&state) {
            return true;
        }
        visited.insert(state);
        let next_guard = guard + dir.step();
        if map.obstacles.contains(&next_guard) {
            dir = dir.rotate_right();
        } else {
            guard = next_guard;
        }
    }
    false
}

#[aoc(day6, part2)]
fn part2((map, guard): &Input) -> usize {
    let (width, height) = (map.width, map.height);
    let mut loops = 0usize;
    for x in 0..width {
        for y in 0..height {
            let pos = Vector2D::new(x, y);
            if &pos == guard {
                continue;
            }
            if map.obstacles.contains(&pos) {
                continue;
            }
            let mut map = map.clone();
            map.obstacles.insert(pos);
            if is_loop(&map, *guard) {
                loops += 1;
            }
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
