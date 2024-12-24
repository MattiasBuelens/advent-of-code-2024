use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Device {
    inputs: HashMap<String, bool>,
    gates: Vec<Gate>,
}

#[derive(Debug, Clone)]
struct Gate {
    op: Operation,
    left: String,
    right: String,
    output: String,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operation {
    And,
    Or,
    Xor,
}

#[aoc_generator(day24)]
fn parse(input: &str) -> Device {
    let (inputs, gates) = input.split_once("\n\n").unwrap();
    let inputs = inputs
        .lines()
        .map(|line| {
            let (name, value) = line.split_once(": ").unwrap();
            (name.to_string(), value == "1")
        })
        .collect();
    let gate_re = Regex::new(r"^(\S+) (AND|OR|XOR) (\S+) -> (\S+)$").unwrap();
    let gates = gates
        .lines()
        .map(|line| {
            let (_, [left, op, right, output]) = gate_re.captures(line).unwrap().extract();
            let op = match op {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "XOR" => Operation::Xor,
                _ => panic!("invalid operation: {op}"),
            };
            Gate {
                op,
                left: left.to_string(),
                right: right.to_string(),
                output: output.to_string(),
            }
        })
        .collect();
    Device { inputs, gates }
}

impl Device {
    fn solve(&mut self) {
        loop {
            let mut output_changed = false;
            for gate in &self.gates {
                if self.inputs.contains_key(&gate.output) {
                    // Already computed
                    continue;
                }
                // Compute output if left and right inputs are known
                let Some(&left) = self.inputs.get(&gate.left) else {
                    continue;
                };
                let Some(&right) = self.inputs.get(&gate.right) else {
                    continue;
                };
                let output = match gate.op {
                    Operation::And => left && right,
                    Operation::Or => left || right,
                    Operation::Xor => left ^ right,
                };
                self.inputs.insert(gate.output.clone(), output);
                output_changed = true;
            }
            if !output_changed {
                // No changes, all possible outputs must have been computed
                break;
            }
        }
    }

    fn z(&self) -> u64 {
        let mut z = 0;
        for i in 0.. {
            let name = format!("z{i:#02}");
            let Some(&value) = self.inputs.get(&name) else {
                break;
            };
            z |= (value as u64) << i;
        }
        z
    }
}

#[aoc(day24, part1)]
fn part1(device: &Device) -> u64 {
    let mut device = device.clone();
    device.solve();
    device.z()
}

#[aoc(day24, part2)]
fn part2(device: &Device) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../examples/2024/day24-1.txt");
    const EXAMPLE2: &str = include_str!("../examples/2024/day24-2.txt");

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse(EXAMPLE1)), 4);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse(EXAMPLE2)), 2024);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE1)), 0);
    }
}
