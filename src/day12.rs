use crate::util::{Direction, Vector2D};
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::dfs_reach;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Garden {
    plants: Vec<Vec<char>>,
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Garden {
    let plants = input.lines().map(|line| line.chars().collect()).collect();
    Garden { plants }
}

impl Garden {
    fn get(&self, pos: Vector2D) -> Option<char> {
        self.plants
            .get(pos.y() as usize)?
            .get(pos.x() as usize)
            .copied()
    }
}

#[derive(Debug, Default)]
struct Plot {
    plants: HashSet<Vector2D>,
}

impl Garden {
    fn plants(&self) -> impl Iterator<Item = (Vector2D, char)> + use<'_> {
        self.plants.iter().enumerate().flat_map(move |(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, &plant)| (Vector2D::new(x as i32, y as i32), plant))
        })
    }

    fn plots(&self) -> Vec<Plot> {
        let mut plots = Vec::new();
        let mut plots_by_pos = HashMap::<Vector2D, Weak<Plot>>::new();
        // Group plants into plots
        for (start_pos, plant) in self.plants() {
            if plots_by_pos.contains_key(&start_pos) {
                continue;
            }
            let plants = dfs_reach(start_pos, |&curr| {
                Direction::all().into_iter().filter_map(move |dir| {
                    let neighbour_pos = curr + dir.step();
                    if self.get(neighbour_pos)? == plant {
                        Some(neighbour_pos)
                    } else {
                        None
                    }
                })
            })
            .collect::<HashSet<_>>();
            let plot = Rc::new(Plot { plants });
            for &pos in &plot.plants {
                plots_by_pos.insert(pos, Rc::downgrade(&plot));
            }
            plots.push(plot);
        }
        plots
            .into_iter()
            .map(|plot| Rc::into_inner(plot).unwrap())
            .collect()
    }
}

impl Plot {
    fn area(&self) -> usize {
        self.plants.len()
    }

    fn perimeter(&self) -> usize {
        let mut perimeter = 0usize;
        for &pos in self.plants.iter() {
            for dir in Direction::all() {
                if self.plants.contains(&(pos + dir.step())) {
                    // Same plot, no edge between them
                } else {
                    // Other plot, or outside of garden
                    perimeter += 1;
                }
            }
        }
        perimeter
    }
}

#[aoc(day12, part1)]
fn part1(garden: &Garden) -> usize {
    garden
        .plots()
        .into_iter()
        .map(|plot| plot.area() * plot.perimeter())
        .sum()
}

impl Plot {
    fn sides(&self) -> usize {
        let mut sides = 0usize;
        // A point can be a corner, so it'll appear on two sides.
        // Distinguish them with the direction of the edge.
        let mut seen = HashSet::<(Vector2D, Direction)>::new();
        for &pos in self.plants.iter() {
            for dir in Direction::all() {
                let neighbour_pos = pos + dir.step();
                if self.plants.contains(&neighbour_pos) {
                    // Same plot, no edge between them
                } else if seen.contains(&(pos, dir)) {
                    // Already seen this side
                } else {
                    // New side
                    sides += 1;
                    // Mark all plants along this side
                    let left_dir = dir.rotate_left();
                    let mut left_pos = pos + left_dir.step();
                    while self.plants.contains(&left_pos)
                        && !self.plants.contains(&(left_pos + dir.step()))
                    {
                        seen.insert((left_pos, dir));
                        left_pos += left_dir.step();
                    }
                    let right_dir = dir.rotate_right();
                    let mut right_pos = pos + right_dir.step();
                    while self.plants.contains(&right_pos)
                        && !self.plants.contains(&(right_pos + dir.step()))
                    {
                        seen.insert((right_pos, dir));
                        right_pos += right_dir.step();
                    }
                }
            }
        }
        sides
    }
}

#[aoc(day12, part2)]
fn part2(garden: &Garden) -> usize {
    garden
        .plots()
        .into_iter()
        .map(|plot| plot.area() * plot.sides())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r"AAAA
BBCD
BBCC
EEEC";

    const EXAMPLE2: &str = r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const EXAMPLE3: &str = include_str!("../examples/2024/day12-large.txt");

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse(EXAMPLE1)), 140);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse(EXAMPLE2)), 772);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part1(&parse(EXAMPLE3)), 1930);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(&parse(EXAMPLE1)), 80);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2(&parse(EXAMPLE2)), 436);
    }

    const EXAMPLE_E: &str = r"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    #[test]
    fn part2_example_e() {
        assert_eq!(part2(&parse(EXAMPLE_E)), 236);
    }

    const EXAMPLE_AB: &str = r"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn part2_example_ab() {
        assert_eq!(part2(&parse(EXAMPLE_AB)), 368);
    }

    #[test]
    fn part2_example3() {
        assert_eq!(part2(&parse(EXAMPLE3)), 1206);
    }
}
