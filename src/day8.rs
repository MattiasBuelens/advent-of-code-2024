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

    fn antinodes(&self) -> impl Iterator<Item = Vector2D> + use<'_> {
        self.antennas
            .values()
            .flat_map(|antennas| Map::antinodes_for(antennas))
            .filter(|&antinode| self.is_in_bounds(antinode))
    }

    fn antinodes_for(antennas: &[Vector2D]) -> impl Iterator<Item = Vector2D> + use<'_> {
        antennas.iter().combinations(2).flat_map(|vec| {
            let (left, right) = (*vec[0], *vec[1]);
            let offset = right - left;
            [left - offset, right + offset]
        })
    }
}

#[aoc(day8, part1)]
fn part1(map: &Map) -> usize {
    map.antinodes().unique().count()
}

#[aoc(day8, part2)]
fn part2(map: &Map) -> usize {
    todo!()
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
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
