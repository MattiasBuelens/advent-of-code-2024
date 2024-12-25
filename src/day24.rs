use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

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

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Operation::And => write!(f, "AND"),
            Operation::Or => write!(f, "OR"),
            Operation::Xor => write!(f, "XOR"),
        }
    }
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

    fn input_name(prefix: char, bit: usize) -> String {
        format!("{prefix}{bit:#02}")
    }

    fn z(&self) -> u64 {
        let mut z = 0;
        for bit in 0.. {
            let Some(&value) = self.inputs.get(&Self::input_name('z', bit)) else {
                break;
            };
            z |= (value as u64) << bit;
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

impl Device {
    fn input_length(&self) -> usize {
        for bit in 0.. {
            if !self.inputs.contains_key(&Self::input_name('x', bit)) {
                return bit;
            }
        }
        unreachable!()
    }

    fn x_name(bit: usize) -> String {
        Self::input_name('x', bit)
    }

    fn y_name(bit: usize) -> String {
        Self::input_name('y', bit)
    }

    fn z_name(bit: usize) -> String {
        Self::input_name('z', bit)
    }

    fn fix_adder(&mut self) -> Vec<String> {
        let length = self.input_length();
        let mut swaps = Vec::with_capacity(8);
        let mut carry_in: Option<String> = None;
        for bit in 0..length {
            let mut carry_out = String::new();
            // Figure out which wires are being used for this bit.
            // (Assume swaps don't need wires used by different bits.)
            let used_wires = self.wires_for_bit(bit, carry_in.as_ref());
            // Check if this bit works correctly
            if !self.test_adder_for_bit(bit, &used_wires, carry_in.as_ref(), &mut carry_out) {
                println!("Need swap on bit {bit}");
                let mut fixed = false;
                for (left, right) in used_wires.iter().tuple_combinations() {
                    // Swap the output wires of two used gates.
                    println!("Try swapping {left} and {right}");
                    let mut device = self.clone();
                    device.swap_wires(left, right);
                    // Check if it works correctly now.
                    if device.test_adder_for_bit(
                        bit,
                        &used_wires,
                        carry_in.as_ref(),
                        &mut carry_out,
                    ) {
                        println!("Swap {left} and {right}");
                        self.swap_wires(left, right);
                        swaps.push(left.clone());
                        swaps.push(right.clone());
                        fixed = true;
                        break;
                    }
                }
                assert!(fixed);
            }
            carry_in = Some(carry_out);
        }
        swaps
    }

    fn wires_for_bit(&mut self, bit: usize, carry_in: Option<&String>) -> Vec<String> {
        // Put 0 + 0 (with carry-in 0) on the input wires
        self.inputs.clear();
        self.inputs.insert(Self::x_name(bit), false);
        self.inputs.insert(Self::y_name(bit), false);
        if let Some(carry_in) = carry_in {
            self.inputs.insert(carry_in.clone(), false);
        }
        self.solve();
        // Find which gate outputs were activated
        self.gates
            .iter()
            .map(|gate| &gate.output)
            .filter(|&wire| self.inputs.contains_key(wire))
            .cloned()
            .collect()
    }

    fn test_adder_for_bit(
        &mut self,
        bit: usize,
        used_wires: &[String],
        carry_in: Option<&String>,
        carry_out: &mut String,
    ) -> bool {
        // Figure out which wires hold the output and the carry-out
        let Some((output_name, carry_name)) = self.get_outputs(used_wires) else {
            return false;
        };
        *carry_out = carry_name.clone();
        // Run some tests
        // Operands:  1 + 0
        // Carry in:  0
        // Output:    1
        // Carry out: 0
        self.inputs.clear();
        self.inputs.insert(Self::x_name(bit), true);
        self.inputs.insert(Self::y_name(bit), false);
        if let Some(carry_in) = carry_in.cloned() {
            self.inputs.insert(carry_in, false);
        }
        self.solve();
        let expected_output = true;
        let expected_carry = false;
        dbg!(&output_name, &carry_name, &self.inputs);
        let actual_output = *self.inputs.get(&output_name).unwrap();
        let actual_carry = *self.inputs.get(&carry_name).unwrap();
        if expected_output != actual_output || expected_carry != actual_carry {
            println!("1 + 0 = 01 failed, output {output_name} = {actual_output}, carry {carry_name} = {actual_carry}");
            return false;
        }
        // Operands:  1 + 1
        // Carry in:  0
        // Output:    0
        // Carry out: 1
        self.inputs.clear();
        self.inputs.insert(Self::x_name(bit), true);
        self.inputs.insert(Self::y_name(bit), true);
        if let Some(carry_in) = carry_in.cloned() {
            self.inputs.insert(carry_in, false);
        }
        self.solve();
        let expected_output = false;
        let expected_carry = true;
        let actual_output = *self.inputs.get(&output_name).unwrap();
        let actual_carry = *self.inputs.get(&carry_name).unwrap();
        if expected_output != actual_output || expected_carry != actual_carry {
            println!("1 + 1 = 10 failed, output {output_name} = {actual_output}, carry {carry_name} = {actual_carry}");
            return false;
        }
        if let Some(carry_in) = carry_in.cloned() {
            // Operands:  0 + 1
            // Carry in:  1
            // Output:    1
            // Carry out: 0
            self.inputs.clear();
            self.inputs.insert(Self::x_name(bit), false);
            self.inputs.insert(Self::y_name(bit), true);
            self.inputs.insert(carry_in.clone(), true);
            self.solve();
            let expected_output = true;
            let expected_carry = false;
            let actual_output = *self.inputs.get(&output_name).unwrap();
            let actual_carry = *self.inputs.get(&carry_name).unwrap();
            if expected_output != actual_output || expected_carry != actual_carry {
                println!("0 + 1 (+ 1) = 10 failed, output {output_name} = {actual_output}, carry {carry_name} = {actual_carry}");
                return false;
            }
            // Operands:  1 + 1
            // Carry in:  1
            // Output:    1
            // Carry out: 1
            self.inputs.clear();
            self.inputs.insert(Self::x_name(bit), true);
            self.inputs.insert(Self::y_name(bit), true);
            self.inputs.insert(carry_in.clone(), true);
            self.solve();
            let expected_output = true;
            let expected_carry = true;
            let actual_output = *self.inputs.get(&output_name).unwrap();
            let actual_carry = *self.inputs.get(&carry_name).unwrap();
            if expected_output != actual_output || expected_carry != actual_carry {
                println!("1 + 1 (+ 1) = 11 failed, output {output_name} = {actual_output}, carry {carry_name} = {actual_carry}");
                return false;
            }
        }
        true
    }

    fn get_outputs(&self, used_wires: &[String]) -> Option<(String, String)> {
        let outputs = used_wires
            .iter()
            .filter(|input| self.is_output(input))
            .collect::<Vec<_>>();
        let carry_outs = used_wires
            .iter()
            .filter(|input| self.is_carry_out(input))
            .collect::<Vec<_>>();
        dbg!(&outputs, &carry_outs, &self.inputs);
        // Must have exactly one output and one carry-out
        match (outputs.as_slice(), carry_outs.as_slice()) {
            ([ref output], [ref carry]) => Some(((*output).clone(), (*carry).clone())),
            _ => None,
        }
    }

    fn is_output(&self, wire: &str) -> bool {
        // The output bit is a wire that is set, but is not connected to any gate
        if !self.inputs.contains_key(wire) {
            return false;
        }
        self.gates
            .iter()
            .all(|gate| gate.left != wire && gate.right != wire)
    }

    fn is_carry_out(&self, wire: &str) -> bool {
        // The carry bit is a wire that is set, but is connected to an unsatisfied gate.
        if !self.inputs.contains_key(wire) {
            return false;
        }
        self.gates.iter().any(|gate| {
            if gate.left == wire {
                !self.inputs.contains_key(&gate.right)
            } else if gate.right == wire {
                !self.inputs.contains_key(&gate.left)
            } else {
                false
            }
        })
    }

    fn swap_wires(&mut self, left: &String, right: &String) {
        for gate in &mut self.gates {
            if &gate.output == left {
                gate.output = right.clone();
            } else if &gate.output == right {
                gate.output = left.clone();
            }
        }
    }
}

#[aoc(day24, part2)]
fn part2(device: &Device) -> String {
    let mut device = device.clone();
    let mut swaps = device.fix_adder();
    swaps.sort();
    swaps.iter().join(",")
}

#[allow(unused)]
impl Device {
    fn print_mermaid(&self) {
        println!(
            r"---
config:
  layout: elk
---
flowchart LR"
        );
        for (i, gate) in self.gates.iter().enumerate() {
            println!("  gate_{i}[{}]", gate.op);
            if let Some(left) = self.gate_by_output_wire(&gate.left) {
                println!("  gate_{left} --> gate_{i}")
            } else {
                Self::print_io(&gate.left);
                println!("  {} --> gate_{i}", gate.left);
            }
            if let Some(right) = self.gate_by_output_wire(&gate.right) {
                println!("  gate_{right} --> gate_{i}")
            } else {
                Self::print_io(&gate.right);
                println!("  {} --> gate_{i}", gate.right);
            }
            if let Some(_) = self.gate_by_input_wire(&gate.output) {
                // Skip, connection will be printed by other gate
            } else {
                Self::print_io(&gate.output);
                println!("  gate_{i} --> {}", gate.output);
            }
        }
    }

    fn gate_by_output_wire(&self, wire: &str) -> Option<usize> {
        self.gates.iter().position(|gate| gate.output == wire)
    }

    fn gate_by_input_wire(&self, wire: &str) -> Option<usize> {
        self.gates
            .iter()
            .position(|gate| gate.left == wire || gate.right == wire)
    }

    fn print_io(wire: &str) {
        println!("  {wire}@{{ shape: \"circle\", label: \"{wire}\" }}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../examples/2024/day24-1.txt");
    const EXAMPLE2: &str = include_str!("../examples/2024/day24-2.txt");
    const INPUT: &str = include_str!("../input/2024/day24.txt");

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse(EXAMPLE1)), 4);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse(EXAMPLE2)), 2024);
    }

    #[test]
    fn part2_mermaid() {
        parse(INPUT).print_mermaid();
    }

    #[test]
    fn part2_example1_mermaid() {
        parse(EXAMPLE1).print_mermaid();
    }

    #[test]
    fn part2_example2_mermaid() {
        parse(EXAMPLE2).print_mermaid();
    }
}
