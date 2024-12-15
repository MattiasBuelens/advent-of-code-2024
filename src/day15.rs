use crate::util::{Direction, Vector2D};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
struct Map {
    walls: HashSet<Vector2D>,
    boxes: HashSet<Vector2D>,
    robot: Vector2D,
}

type Input = (Map, Vec<Direction>);

#[aoc_generator(day15)]
fn parse(input: &str) -> Input {
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();
    let mut robot = Vector2D::zero();
    let (map, moves) = input.split_once("\n\n").unwrap();
    for (y, line) in map.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Vector2D::new(x as i32, y as i32);
            match c {
                '.' => {}
                '#' => {
                    walls.insert(pos);
                }
                'O' => {
                    boxes.insert(pos);
                }
                '@' => {
                    robot = pos;
                }
                _ => panic!("Invalid map tile {c}"),
            };
        }
    }
    let map = Map {
        walls,
        boxes,
        robot,
    };
    let moves = moves
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| match c {
            '^' => Direction::N,
            '>' => Direction::E,
            'v' => Direction::S,
            '<' => Direction::W,
            _ => panic!("Invalid move '{c}'"),
        })
        .collect();
    (map, moves)
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_x = self.walls.iter().map(|pos| pos.x()).max().unwrap();
        let max_y = self.walls.iter().map(|pos| pos.y()).max().unwrap();
        for y in 0..=max_y {
            for x in 0..=max_x {
                let pos = Vector2D::new(x, y);
                let c = if self.walls.contains(&pos) {
                    '#'
                } else if self.boxes.contains(&pos) {
                    'O'
                } else if self.robot == pos {
                    '@'
                } else {
                    '.'
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn step(&mut self, dir: Direction) {
        if let Some(boxes_to_move) = self.try_step(self.robot, dir) {
            // Can move to new position by pushing the found boxes.
            self.boxes.retain(|pos| !boxes_to_move.contains(pos));
            self.boxes
                .extend(boxes_to_move.into_iter().map(|pos| pos + dir.step()));
            self.robot += dir.step();
        } else {
            // Cannot move.
        }
    }

    fn try_step(&self, pos: Vector2D, dir: Direction) -> Option<Vec<Vector2D>> {
        let next_pos = pos + dir.step();
        if self.walls.contains(&next_pos) {
            // Cannot move.
            None
        } else if self.boxes.contains(&next_pos) {
            // Can move only if we can push this box, along with any other boxes on the path.
            let mut boxes = self.try_step(next_pos, dir)?;
            boxes.push(next_pos);
            Some(boxes)
        } else {
            // Can move to an empty space.
            Some(Vec::new())
        }
    }
}

#[aoc(day15, part1)]
fn part1((map, moves): &Input) -> i32 {
    let mut map = map.clone();
    // println!("Initial state:");
    // println!("{map}");
    for &dir in moves {
        map.step(dir);
        // println!("Move {dir:?}:");
        // println!("{map}");
    }
    map.boxes.iter().map(|pos| pos.x() + 100 * pos.y()).sum()
}

fn double_pos(pos: &Vector2D) -> Vector2D {
    Vector2D::new(pos.x() * 2, pos.y())
}

fn stretch_pos(pos: &Vector2D) -> [Vector2D; 2] {
    [
        Vector2D::new(pos.x() * 2, pos.y()),
        Vector2D::new(pos.x() * 2 + 1, pos.y()),
    ]
}

impl Map {
    fn widen(&self) -> Self {
        let walls = self.walls.iter().flat_map(stretch_pos).collect();
        let boxes = self.boxes.iter().map(double_pos).collect();
        let robot = Vector2D::new(self.robot.x() * 2, self.robot.y());
        Map {
            walls,
            boxes,
            robot,
        }
    }

    fn step_part2(&mut self, dir: Direction) {
        if let Some(boxes_to_move) = self.try_step_part2(self.robot, dir) {
            // Can move to new position by pushing the found boxes.
            self.boxes.retain(|pos| !boxes_to_move.contains(pos));
            self.boxes
                .extend(boxes_to_move.into_iter().map(|pos| pos + dir.step()));
            self.robot += dir.step();
        } else {
            // Cannot move.
        }
    }

    fn try_step_part2(&self, pos: Vector2D, dir: Direction) -> Option<Vec<Vector2D>> {
        let next_pos = pos + dir.step();
        if self.walls.contains(&next_pos) {
            // Cannot move.
            None
        } else if self.boxes.contains(&next_pos) {
            // Can move only if we can push the left side of this box.
            self.try_push_box_part2(next_pos, dir)
        } else if self.boxes.contains(&(next_pos - Vector2D::new(1, 0))) {
            // Can move only if we can push the right side of this box.
            self.try_push_box_part2(next_pos - Vector2D::new(1, 0), dir)
        } else {
            // Can move to an empty space.
            Some(Vec::new())
        }
    }

    fn try_push_box_part2(&self, left_box_pos: Vector2D, dir: Direction) -> Option<Vec<Vector2D>> {
        // Can move only if we can push this box, along with any other boxes on the path.
        let right_box_pos = left_box_pos + Vector2D::new(1, 0);
        let mut boxes: Vec<Vector2D>;
        match dir {
            Direction::N | Direction::S => {
                // Must push both sides of the box up or down.
                boxes = self.try_step_part2(left_box_pos, dir)?;
                let boxes_right = self.try_step_part2(right_box_pos, dir)?;
                boxes.extend(boxes_right);
            }
            Direction::W => {
                // Must push from the left of this box.
                boxes = self.try_step_part2(left_box_pos, dir)?;
            }
            Direction::E => {
                // Must push from the right of this box.
                boxes = self.try_step_part2(right_box_pos, dir)?;
            }
        }
        // Always use the left side of the box as its identifier.
        boxes.push(left_box_pos);
        Some(boxes)
    }
}

#[aoc(day15, part2)]
fn part2((map, moves): &Input) -> i32 {
    let mut map = map.widen();
    // println!("Initial state:");
    // println!("{map}");
    for &dir in moves {
        map.step_part2(dir);
        // println!("Move {dir:?}:");
        // println!("{map}");
    }
    map.boxes.iter().map(|pos| pos.x() + 100 * pos.y()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_LARGE: &str = include_str!("../examples/2024/day15-large.txt");
    const EXAMPLE_SMALL: &str = include_str!("../examples/2024/day15-small.txt");

    #[test]
    fn part1_example_large() {
        assert_eq!(part1(&parse(EXAMPLE_LARGE)), 10092);
    }

    #[test]
    fn part1_example_small() {
        assert_eq!(part1(&parse(EXAMPLE_SMALL)), 2028);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_LARGE)), 9021);
    }
}
