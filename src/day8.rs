use crate::util::Vector2D;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

struct Map {
    width: i32,
    height: i32,
    antennas: HashMap<char, Vec<Vector2D>>,
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Map {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    let mut antennas = HashMap::<char, Vec<Vector2D>>::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => {
                    antennas
                        .entry(c)
                        .or_default()
                        .push(Vector2D::new(x as i32, y as i32));
                }
                '.' => {}
                c => panic!("unknown char: {}", c),
            };
        }
    }
    Map {
        width,
        height,
        antennas,
    }
}

impl Map {
    fn is_in_bounds(&self, pos: Vector2D) -> bool {
        (0..self.width).contains(&pos.x()) && (0..self.height).contains(&pos.y())
    }

    fn antinodes(&self, part2: bool) -> impl Iterator<Item = Vector2D> + use<'_> {
        self.antennas
            .values()
            .flat_map(move |antennas| self.antinodes_for(antennas, part2))
            .unique()
    }

    fn antinodes_for<'a>(
        &'a self,
        antennas: &'a [Vector2D],
        part2: bool,
    ) -> impl Iterator<Item = Vector2D> + 'a {
        antennas.iter().combinations(2).flat_map(move |vec| {
            let (left, right) = (*vec[0], *vec[1]);
            let offset = right - left;
            let mut result = vec![];
            if part2 {
                let mut pos = left;
                while self.is_in_bounds(pos) {
                    result.push(pos);
                    pos -= offset;
                }
                let mut pos = right;
                while self.is_in_bounds(pos) {
                    result.push(pos);
                    pos += offset;
                }
            } else {
                let left_antinode = left - offset;
                if self.is_in_bounds(left_antinode) {
                    result.push(left_antinode);
                }
                let right_antinode = right + offset;
                if self.is_in_bounds(right_antinode) {
                    result.push(right_antinode);
                }
            }
            result
        })
    }
}

#[aoc(day8, part1)]
fn part1(map: &Map) -> usize {
    map.antinodes(false).count()
}

#[aoc(day8, part2)]
fn part2(map: &Map) -> usize {
    map.antinodes(true).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/2024/day8.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 34);
    }
}
