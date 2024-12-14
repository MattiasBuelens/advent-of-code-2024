use crate::util::{Direction, Vector2D};
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::dfs_reach;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
    area: usize,
    perimeter: usize,
}

#[derive(Debug, Default)]
struct Plots {
    plots: Vec<Rc<RefCell<Plot>>>,
    plots_by_pos: HashMap<Vector2D, Rc<RefCell<Plot>>>,
}

impl Garden {
    fn plants(&self) -> impl Iterator<Item = (Vector2D, char)> + use<'_> {
        self.plants.iter().enumerate().flat_map(move |(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, &plant)| (Vector2D::new(x as i32, y as i32), plant))
        })
    }

    fn plots(&self) -> Plots {
        let mut plots = Vec::new();
        let mut plots_by_pos = HashMap::<Vector2D, Rc<RefCell<Plot>>>::new();
        // Group plants into plots
        for (start_pos, plant) in self.plants() {
            if plots_by_pos.contains_key(&start_pos) {
                continue;
            }
            let plot = Rc::new(RefCell::new(Plot::default()));
            plots.push(plot.clone());
            plots_by_pos.insert(start_pos, plot.clone());
            dfs_reach(start_pos, |&curr| {
                Direction::all().into_iter().filter_map(move |dir| {
                    let neighbour_pos = curr + dir.step();
                    if self.get(neighbour_pos)? == plant {
                        Some(neighbour_pos)
                    } else {
                        None
                    }
                })
            })
            .for_each(|pos| {
                plots_by_pos.insert(pos, plot.clone());
                plot.borrow_mut().area += 1;
            });
        }
        // Compute perimeters
        for (&pos, plot) in &plots_by_pos {
            for dir in Direction::all() {
                let neighbour_pos = pos + dir.step();
                match plots_by_pos.get(&neighbour_pos) {
                    Some(neighbour) if Rc::ptr_eq(plot, neighbour) => {
                        // Same plot, no edge between them
                    }
                    _ => {
                        // Other plot, or outside of garden
                        plot.borrow_mut().perimeter += 1;
                    }
                }
            }
        }
        Plots {
            plots,
            plots_by_pos,
        }
    }
}

impl Plots {
    fn values(mut self) -> impl Iterator<Item = Plot> {
        self.plots_by_pos.clear();
        self.plots
            .into_iter()
            .map(|plot| Rc::into_inner(plot).unwrap().into_inner())
    }
}

#[aoc(day12, part1)]
fn part1(garden: &Garden) -> usize {
    garden
        .plots()
        .values()
        .map(|plot| plot.area * plot.perimeter)
        .sum()
}

#[aoc(day12, part2)]
fn part2(garden: &Garden) -> usize {
    todo!()
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
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE1)), 0);
    }
}
