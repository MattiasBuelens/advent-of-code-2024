use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet};

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
        connections.entry(second).or_default().insert(first);
    }
    NetworkMap { connections }
}

impl NetworkMap {
    fn connected_computers(&self) -> HashSet<Vec<&str>> {
        let mut components = HashSet::new();
        for (first, first_connections) in &self.connections {
            for second in first_connections {
                for third in first_connections {
                    if second != third && self.connections.get(second).unwrap().contains(third) {
                        let mut component = vec![first.as_str(), second.as_str(), third.as_str()];
                        component.sort();
                        components.insert(component);
                    }
                }
            }
        }
        components
    }

    fn max_clique(&self) -> BTreeSet<&str> {
        let mut max_clique = BTreeSet::new();
        for (first, first_connections) in &self.connections {
            for second in first_connections {
                // Start with two connected computers in the clique
                let mut clique = BTreeSet::from([first.as_str(), second.as_str()]);
                // Extend this clique with other computers that are also connected to the entire clique
                for third in first_connections {
                    if !clique.contains(third.as_str()) && self.in_clique(&clique, third) {
                        clique.insert(third.as_str());
                        if clique.len() > max_clique.len() {
                            max_clique = clique.clone();
                        }
                    }
                }
            }
        }
        max_clique
    }

    fn in_clique(&self, clique: &BTreeSet<&str>, other: &str) -> bool {
        let other_connections = self.connections.get(other).unwrap();
        clique
            .iter()
            .all(|&existing| other_connections.contains(existing))
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
fn part2(map: &NetworkMap) -> String {
    // Already sorted alphabetically, because BTreeSet is ordered
    map.max_clique().iter().join(",")
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
        assert_eq!(part2(&parse(EXAMPLE)), "co,de,ka,ta");
    }
}
