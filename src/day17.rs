use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Program {
    registers: [u64; 3],
    pc: usize,
    code: Vec<u8>,
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Program {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let values: [u64; 3] = registers
        .lines()
        .zip('A'..='C')
        .map(|(line, name)| {
            let line = line.strip_prefix("Register ").unwrap();
            let line = line.strip_prefix(name).unwrap();
            let line = line.strip_prefix(": ").unwrap();
            line.parse::<u64>().unwrap()
        })
        .collect::<Vec<u64>>()
        .try_into()
        .unwrap();
    let code = program.strip_prefix("Program: ").unwrap().trim_end();
    let code = code.split(',').map(|x| x.parse().unwrap()).collect();
    Program {
        registers: values,
        pc: 0,
        code,
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl TryFrom<u8> for Opcode {
    type Error = ();
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            0 => Opcode::Adv,
            1 => Opcode::Bxl,
            2 => Opcode::Bst,
            3 => Opcode::Jnz,
            4 => Opcode::Bxc,
            5 => Opcode::Out,
            6 => Opcode::Bdv,
            7 => Opcode::Cdv,
            _ => return Err(()),
        })
    }
}

impl Program {
    fn run(&mut self) -> Vec<u64> {
        let mut output = Vec::new();
        while self.pc + 1 < self.code.len() {
            if let Some(value) = self.step() {
                output.push(value);
            }
        }
        output
    }

    fn run_while_matching(&mut self) -> bool {
        let mut output_index = 0;
        while self.pc + 1 < self.code.len() {
            if let Some(value) = self.step() {
                if value != self.code[output_index] as u64 {
                    return false;
                }
                output_index += 1;
            }
        }
        output_index == self.code.len()
    }

    fn step(&mut self) -> Option<u64> {
        let opcode = Opcode::try_from(self.code[self.pc]).expect("invalid opcode");
        let operand = self.code[self.pc + 1];
        self.pc += 2;
        match opcode {
            Opcode::Adv => self.registers[0] >>= self.combo(operand),
            Opcode::Bxl => self.registers[1] ^= operand as u64,
            Opcode::Bst => self.registers[1] = self.combo(operand) & 0b111,
            Opcode::Jnz => {
                if self.registers[0] != 0 {
                    self.pc = operand as usize;
                }
            }
            Opcode::Bxc => self.registers[1] ^= self.registers[2],
            Opcode::Out => return Some(self.combo(operand) & 0b111),
            Opcode::Bdv => self.registers[1] = self.registers[0] >> self.combo(operand),
            Opcode::Cdv => self.registers[2] = self.registers[0] >> self.combo(operand),
        }
        None
    }

    fn combo(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4..=6 => self.registers[(operand - 4) as usize],
            7 => panic!("reserved"),
            _ => panic!("invalid combo operand {operand}"),
        }
    }

    fn reset(&mut self, program: &Program) {
        self.registers = program.registers;
        self.pc = 0;
    }
}

#[aoc(day17, part1)]
fn part1(program: &Program) -> String {
    program.clone().run().iter().join(",")
}

#[aoc(day17, part2)]
fn part2(program: &Program) -> u64 {
    let mut new_program = program.clone();
    for a in 0u64.. {
        if a % 10_000_000 == 0 {
            dbg!(a);
        }
        new_program.reset(program);
        new_program.registers[0] = a;
        if new_program.run_while_matching() {
            return a;
        }
    }
    panic!("no value found for A");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../examples/2024/day17-1.txt");
    const EXAMPLE2: &str = include_str!("../examples/2024/day17-2.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE1)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE2)), 117440);
    }
}
