use crate::util::{Direction, Vector2D};
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::dijkstra;
use std::collections::HashSet;

type Input = Vec<Vector2D>;

#[aoc_generator(day18)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Vector2D::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn successors<'a>(
    pos: Vector2D,
    bytes: &'a HashSet<&Vector2D>,
    size: i32,
) -> impl Iterator<Item = (Vector2D, usize)> + use<'a> {
    Direction::all().into_iter().filter_map(move |dir| {
        let next = pos + dir.step();
        if (0..=size).contains(&next.x())
            && (0..=size).contains(&next.y())
            && !bytes.contains(&next)
        {
            Some((next, 1))
        } else {
            None
        }
    })
}

fn find_path(bytes: &[Vector2D], size: i32) -> Option<usize> {
    let bytes = bytes.iter().collect::<HashSet<_>>();
    let goal = Vector2D::new(size, size);
    let (_path, cost) = dijkstra(
        &Vector2D::zero(),
        |&pos| successors(pos, &bytes, size),
        |pos| pos == &goal,
    )?;
    Some(cost)
}

#[aoc(day18, part1)]
fn part1(input: &Input) -> usize {
    find_path(&input[0..1024], 70).expect("no solution found")
}

#[aoc(day18, part2)]
fn part2(input: &Input) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/2024/day18.txt");

    #[test]
    fn part1_example() {
        let input = parse(EXAMPLE);
        assert_eq!(find_path(&input[0..12], 6), Some(22));
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
