use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Program {
    registers: [i32; 3],
    pc: usize,
    code: Vec<u8>,
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Program {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let values: [i32; 3] = registers
        .lines()
        .zip('A'..='C')
        .map(|(line, name)| {
            let line = line.strip_prefix("Register ").unwrap();
            let line = line.strip_prefix(name).unwrap();
            let line = line.strip_prefix(": ").unwrap();
            line.parse::<i32>().unwrap()
        })
        .collect::<Vec<i32>>()
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
    fn run(&mut self) -> Vec<i32> {
        let mut out = Vec::new();
        while self.pc + 1 < self.code.len() {
            let opcode = Opcode::try_from(self.code[self.pc]).expect("invalid opcode");
            let operand = self.code[self.pc + 1];
            match opcode {
                Opcode::Adv => self.registers[0] >>= self.combo(operand),
                Opcode::Bxl => self.registers[1] ^= operand as i32,
                Opcode::Bst => self.registers[1] = self.combo(operand) & 0b111,
                Opcode::Jnz => {
                    if self.registers[0] != 0 {
                        self.pc = operand as usize;
                        continue;
                    }
                }
                Opcode::Bxc => self.registers[1] ^= self.registers[2],
                Opcode::Out => out.push(self.combo(operand) & 0b111),
                Opcode::Bdv => self.registers[1] = self.registers[0] >> self.combo(operand),
                Opcode::Cdv => self.registers[2] = self.registers[0] >> self.combo(operand),
            }
            self.pc += 2;
        }
        out
    }

    fn combo(&self, operand: u8) -> i32 {
        match operand {
            0..=3 => operand as i32,
            4..=6 => self.registers[(operand - 4) as usize],
            7 => panic!("reserved"),
            _ => panic!("invalid combo operand {operand}"),
        }
    }
}

#[aoc(day17, part1)]
fn part1(program: &Program) -> String {
    program.clone().run().iter().join(",")
}

#[aoc(day17, part2)]
fn part2(program: &Program) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/2024/day17.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "");
    }
}
