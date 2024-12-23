use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

struct NetworkMap {
    connections: HashMap<String, HashSet<String>>,
}

#[aoc_generator(day23)]
fn parse(input: &str) -> NetworkMap {
    let mut connections = HashMap::<String, HashSet<String>>::new();
    for line in input.lines() {
        let (first, second) = line.split_once('-').unwrap();
        let first = first.to_string();
        let second = second.to_string();
        connections
            .entry(first.clone())
            .or_default()
            .insert(second.clone());
        connections
            .entry(second.clone())
            .or_default()
            .insert(first.clone());
    }
    NetworkMap { connections }
}

impl NetworkMap {
    fn connected_computers(&self) -> HashSet<Vec<String>> {
        let mut components = HashSet::new();
        for (first, first_connections) in &self.connections {
            for second in first_connections {
                for third in self.connections.get(second).unwrap().iter() {
                    if third != first && first_connections.contains(third) {
                        let mut component = vec![first.clone(), second.clone(), third.clone()];
                        component.sort();
                        components.insert(component);
                    }
                }
            }
        }
        components
    }
}

#[aoc(day23, part1)]
fn part1(map: &NetworkMap) -> usize {
    map.connected_computers()
        .into_iter()
        .filter(|component| component.iter().any(|computer| computer.starts_with('t')))
        .count()
}

#[aoc(day23, part2)]
fn part2(map: &NetworkMap) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/2024/day23.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
