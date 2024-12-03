use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"(mul|do|don't)\((?:(\d+),(\d+))?\)").unwrap();
    re.captures_iter(input)
        .flat_map(|c| {
            Some(match c.get(1).unwrap().as_str() {
                "mul" => Instruction::Mul(
                    c.get(2)?.as_str().parse().ok()?,
                    c.get(3)?.as_str().parse().ok()?,
                ),
                "do" => Instruction::Do,
                "don't" => Instruction::Dont,
                _ => return None,
            })
        })
        .collect::<Vec<_>>()
}

#[aoc(day3, part1)]
fn part1(input: &[Instruction]) -> i32 {
    let mut sum = 0;
    for instruction in input {
        if let Instruction::Mul(left, right) = instruction { sum += left * right };
    }
    sum
}

#[aoc(day3, part2)]
fn part2(input: &[Instruction]) -> i32 {
    let mut sum = 0;
    let mut enabled = true;
    for instruction in input {
        match instruction {
            Instruction::Mul(left, right) => {
                if enabled {
                    sum += left * right
                }
            }
            Instruction::Do => {
                enabled = true;
            }
            Instruction::Dont => {
                enabled = false;
            }
        };
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(&parse(example)), 161);
    }

    #[test]
    fn part2_example() {
        let example = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(&parse(example)), 48);
    }
}
