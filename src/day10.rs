use crate::util::Vector2D;
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::{count_paths, dfs_reach};
use std::collections::HashMap;

struct HeightMap {
    width: i32,
    height: i32,
    heights: HashMap<Vector2D, u32>,
}

#[aoc_generator(day10)]
fn parse(input: &str) -> HeightMap {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    let mut heights = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            heights.insert(Vector2D::new(x as i32, y as i32), c.to_digit(10).unwrap());
        }
    }
    HeightMap {
        width,
        height,
        heights,
    }
}

impl HeightMap {
    fn trailheads(&self) -> impl Iterator<Item = Vector2D> + use<'_> {
        self.heights
            .iter()
            .filter_map(|(&pos, &height)| if height == 0 { Some(pos) } else { None })
    }

    fn trailhead_score(&self, start: Vector2D) -> usize {
        let start_height = *self.heights.get(&start).unwrap();
        dfs_reach((start, start_height), |&(pos, height)| {
            self.neighbours(pos, height)
        })
        .filter(|&(_, height)| height == 9)
        .count()
    }

    fn neighbours(
        &self,
        pos: Vector2D,
        height: u32,
    ) -> impl Iterator<Item = (Vector2D, u32)> + use<'_> {
        pos.neighbours().filter_map(move |neighbour| {
            let neighbour_height = *self.heights.get(&neighbour)?;
            if neighbour_height == height + 1 {
                Some((neighbour, neighbour_height))
            } else {
                None
            }
        })
    }
}

#[aoc(day10, part1)]
fn part1(input: &HeightMap) -> usize {
    input
        .trailheads()
        .map(|trailhead| input.trailhead_score(trailhead))
        .sum()
}

impl HeightMap {
    fn trailhead_rating(&self, start: Vector2D) -> usize {
        let start_height = *self.heights.get(&start).unwrap();
        count_paths(
            (start, start_height),
            |&(pos, height)| self.neighbours(pos, height),
            |&(_, height)| height == 9,
        )
    }
}

#[aoc(day10, part2)]
fn part2(input: &HeightMap) -> usize {
    input
        .trailheads()
        .map(|trailhead| input.trailhead_rating(trailhead))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/2024/day10.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 81);
    }
}
