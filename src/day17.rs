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
                    // dbg!(self.pc - 2, opcode, operand);
                    // dbg!(&self.code[(self.pc - 2)..]);
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
    // print_code(program);
    let is_real_input = program.code.len() == 16;
    let mut new_program = program.clone();
    let mut output = Vec::with_capacity(program.code.len());
    let mut longest_match = 0usize;
    for a in 0u64.. {
        if a % 100_000_000 == 0 {
            println!("a={a}");
        }
        output.clear();
        let success = if is_real_input {
            part2_decompiled(a, &program.code, &mut output);
            &output[..] == &program.code[..]
        } else {
            new_program.reset(program);
            new_program.registers[0] = a;
            new_program.run_while_matching(&mut output)
        };
        if output.len() >= longest_match {
            longest_match = output.len();
            println!("a={a}, longest_match={longest_match}, output={output:?}");
            // Check if the decompilation is correct
            if is_real_input {
                let real_output = std::mem::replace(&mut output, Vec::new());
                new_program.reset(program);
                new_program.registers[0] = a;
                new_program.run_while_matching(&mut output);
                assert_eq!(&output[..], &real_output[..]);
            }
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

fn part2_decompiled(mut a: u64, expected: &[u8], output: &mut Vec<u8>) {
    let mut b = 0u64;
    let mut c = 0u64;
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
    loop {
        b = a & 0b111;
        b ^= 4;
        c = a >> b;
        b ^= c;
        b ^= 4;
        output.push((b & 0b111) as u8);
        if &output[..] != &expected[..output.len()] {
            break;
        }
        a >>= 3;
        if a == 0 {
            break;
        }
    }
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
