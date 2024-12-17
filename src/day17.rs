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
    fn run(&mut self) -> Vec<u8> {
        let mut output = Vec::new();
        while self.pc + 1 < self.code.len() {
            if let Some(value) = self.step() {
                output.push(value);
            }
        }
        output
    }

    fn run_while_matching(&mut self, output: &mut Vec<u8>) -> bool {
        let mut output_index = 0;
        while self.pc + 1 < self.code.len() {
            if let Some(value) = self.step() {
                output.push(value);
                if value != self.code[output_index] {
                    return false;
                }
                output_index += 1;
            }
        }
        output_index == self.code.len()
    }

    fn step(&mut self) -> Option<u8> {
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
            Opcode::Out => return Some((self.combo(operand) & 0b111) as u8),
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
    let is_real_input = program.code.len() == 16;
    if is_real_input {
        let a = part2_reverse(&program.code).unwrap();
        // Double check
        let mut program = program.clone();
        program.registers[0] = a;
        let output = program.run();
        assert_eq!(&output[..], &program.code);
        a
    } else {
        part2_naive(program)
    }
}

fn part2_naive(program: &Program) -> u64 {
    // print_code(program);
    let mut new_program = program.clone();
    let mut output = Vec::with_capacity(program.code.len());
    let mut longest_match = 0usize;
    for a in 0u64.. {
        if a % 100_000_000 == 0 {
            println!("a={a}");
        }
        output.clear();
        new_program.reset(program);
        new_program.registers[0] = a;
        let success = new_program.run_while_matching(&mut output);
        if output.len() >= longest_match {
            longest_match = output.len();
            println!("a={a}, longest_match={longest_match}, output={output:?}");
        }
        if success {
            return a;
        }
    }
    panic!("no value found for A");
}

#[allow(unused)]
fn print_code(program: &Program) {
    for (i, chunk) in program.code.chunks(2).enumerate() {
        let opcode = Opcode::try_from(chunk[0]).expect("invalid opcode");
        let operand = chunk[1];
        // println!("{opcode:?} {operand}");
        print!("{i:#02}: ");
        match opcode {
            Opcode::Adv => {
                // self.registers[0] >>= self.combo(operand),
                print!("A >>= ");
                print_combo(operand);
            }
            Opcode::Bxl => {
                // self.registers[1] ^= operand as u64,
                print!("B ^= {operand}");
            }
            Opcode::Bst => {
                // self.registers[1] = self.combo(operand) & 0b111
                print!("B = ");
                print_combo(operand);
                print!(" & 0b111")
            }
            Opcode::Jnz => {
                // if self.registers[0] != 0 { self.pc = operand as usize; }
                print!("if (A != 0) JUMP {}", operand / 2);
            }
            Opcode::Bxc => {
                // self.registers[1] ^= self.registers[2]
                print!("B ^= C");
            }
            Opcode::Out => {
                // return Some(self.combo(operand) & 0b111)
                print!("OUTPUT ");
                print_combo(operand);
                print!(" & 0b111")
            }
            Opcode::Bdv => {
                // self.registers[1] = self.registers[0] >> self.combo(operand)
                print!("B = A >> ");
                print_combo(operand);
            }
            Opcode::Cdv => {
                // self.registers[2] = self.registers[0] >> self.combo(operand)
                print!("C = A >> ");
                print_combo(operand);
            }
        }
        println!();
    }
}

fn print_combo(operand: u8) {
    match operand {
        0..=3 => print!("{operand}"),
        4 => print!("A"),
        5 => print!("B"),
        6 => print!("C"),
        7 => panic!("reserved"),
        _ => panic!("invalid combo operand {operand}"),
    }
}

fn part2_decompiled(mut a: u64, expected: &[u8], mut output: Option<&mut Vec<u8>>) -> bool {
    // let mut b = 0u64;
    // let mut c = 0u64;
    /*
    00: B = A & 0b111
    01: B ^= 4
    02: C = A >> B
    03: B ^= C
    04: B ^= 4
    05: OUTPUT B & 0b111
    06: A >>= 3
    07: if (A != 0) JUMP 0
     */
    let mut output_index = 0;
    loop {
        let mut b = a & 0b111;
        // b ^= 4;
        // c = a >> b;
        // b ^= c;
        // b ^= 4;
        b ^= a >> (b ^ 0b100);
        // This output value depends only on the last 10 bits of A
        // - Up to 7 bits from (a >> (b ^ 4)), where b <= 7
        // - 3 bits from (b = a & 0b111)
        let value = (b & 0b111) as u8;
        if let Some(output) = output.as_mut() {
            output.push(value);
        }
        if value != expected[output_index] {
            return false;
        }
        output_index += 1;
        a >>= 3;
        if a == 0 {
            break;
        }
    }
    output_index == expected.len()
}

fn part2_reverse(output: &[u8]) -> Option<u64> {
    part2_reverse_inner(output, 0, output.len() - 1)
}

fn part2_reverse_inner(output: &[u8], mut a: u64, index: usize) -> Option<u64> {
    // Shift for the next iteration
    a <<= 3;
    // Try all possible 10 bit values
    for bits in 0u64..1024 {
        let a = a | bits;
        // Check if we get the expected output starting from index
        if part2_decompiled(a, &output[index..], None) {
            if index == 0 {
                // Matched entire output
                return Some(a);
            } else if let Some(a) = part2_reverse_inner(output, a, index - 1) {
                // Recurse
                return Some(a);
            }
        }
    }
    None
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
