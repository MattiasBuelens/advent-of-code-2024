use crate::util::Vector2D;
use aoc_runner_derive::{aoc, aoc_generator};
use num_traits::Euclid;
use regex::Regex;

#[derive(Debug, Clone)]
struct Robot {
    pos: Vector2D,
    vel: Vector2D,
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"^p=([0-9-]+),([0-9-]+) v=([0-9-]+),([0-9-]+)$").unwrap();
    input
        .lines()
        .map(|line| {
            let (_, [px, py, vx, vy]) = re.captures(line).unwrap().extract();
            Robot {
                pos: Vector2D::new(px.parse().unwrap(), py.parse().unwrap()),
                vel: Vector2D::new(vx.parse().unwrap(), vy.parse().unwrap()),
            }
        })
        .collect()
}

#[aoc(day14, part1)]
fn part1(input: &[Robot]) -> usize {
    solve_part1(input, 101, 103)
}

fn solve_part1(robots: &[Robot], width: i32, height: i32) -> usize {
    let mut robots = robots.to_vec();
    let bounds = Vector2D::new(width, height);
    for _ in 0..100 {
        simulate(&mut robots, bounds);
    }
    safety_score(&robots, width, height)
}

fn simulate(robots: &mut Vec<Robot>, bounds: Vector2D) {
    for robot in robots.iter_mut() {
        robot.pos = (robot.pos + robot.vel).rem_euclid(&bounds);
    }
}

fn safety_score(robots: &[Robot], width: i32, height: i32) -> usize {
    let q1 = robots
        .iter()
        .filter(|r| r.pos.x() < width / 2 && r.pos.y() < height / 2)
        .count();
    let q2 = robots
        .iter()
        .filter(|r| r.pos.x() > width / 2 && r.pos.y() < height / 2)
        .count();
    let q3 = robots
        .iter()
        .filter(|r| r.pos.x() < width / 2 && r.pos.y() > height / 2)
        .count();
    let q4 = robots
        .iter()
        .filter(|r| r.pos.x() > width / 2 && r.pos.y() > height / 2)
        .count();
    q1 * q2 * q3 * q4
}

#[aoc(day14, part2)]
fn part2(input: &[Robot]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Either;

    const EXAMPLE: &str = include_str!("../examples/2024/day14.txt");
    const EXPECTED_ROBOTS: &str = include_str!("../examples/2024/day14-expected.txt");

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse(EXAMPLE), 11, 7), 12);
    }

    #[test]
    fn part1_robots() {
        let mut robots = parse(EXAMPLE);
        let bounds = Vector2D::new(11, 7);
        for _ in 0..100 {
            simulate(&mut robots, bounds);
        }
        let mut positions = robots.iter().map(|r| r.pos).collect::<Vec<_>>();
        let mut expected = parse_expected_robots(EXPECTED_ROBOTS);
        positions.sort();
        expected.sort();
        assert_eq!(positions, expected);
    }

    fn parse_expected_robots(input: &str) -> Vec<Vector2D> {
        input
            .lines()
            .enumerate()
            .flat_map(move |(y, line)| {
                line.chars().enumerate().flat_map(move |(x, c)| match c {
                    '.' => Either::Left(std::iter::empty()),
                    '0'..='9' => Either::Right(std::iter::repeat_n(
                        Vector2D::new(x as i32, y as i32),
                        c.to_digit(10).unwrap() as usize,
                    )),
                    _ => panic!("invalid character {c}"),
                })
            })
            .collect()
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
