use aoc_runner_derive::{aoc, aoc_generator};

struct Crossword {
    size: usize,
    lines: Vec<String>,
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Crossword {
    let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let size = lines.len();
    assert_eq!(lines[0].len(), size);
    Crossword { size, lines }
}

impl Crossword {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.lines[y].as_bytes()[x]
    }

    fn horizontals(&self) -> Vec<Vec<u8>> {
        (0..self.size)
            .map(|y| (0..self.size).map(|x| self.get(x, y)).collect())
            .collect()
    }

    fn verticals(&self) -> Vec<Vec<u8>> {
        (0..self.size)
            .map(|x| (0..self.size).map(|y| self.get(x, y)).collect())
            .collect()
    }

    fn main_diagonals(&self) -> Vec<Vec<u8>> {
        let mut result = Vec::new();
        for offset in 0..self.size {
            result.push(self.get_diagonal(offset, 0, 1, 1));
            if offset != 0 {
                result.push(self.get_diagonal(0, offset, 1, 1));
            }
        }
        result
    }

    fn anti_diagonals(&self) -> Vec<Vec<u8>> {
        let mut result = Vec::new();
        for offset in 0..self.size {
            result.push(self.get_diagonal(offset, 0, -1, 1));
            if offset != 0 {
                result.push(self.get_diagonal(self.size - 1, offset, -1, 1));
            }
        }
        result
    }

    fn get_diagonal(&self, mut x: usize, mut y: usize, step_x: isize, step_y: isize) -> Vec<u8> {
        let mut result = Vec::new();
        while x < self.size && y < self.size {
            result.push(self.get(x, y));
            x = match x.checked_add_signed(step_x) {
                Some(x) => x,
                None => break,
            };
            y = match y.checked_add_signed(step_y) {
                Some(y) => y,
                None => break,
            };
        }
        result
    }
}

fn count_xmas(scan_line: &[u8]) -> usize {
    scan_line
        .windows(4)
        .filter(|&window| matches!(window, b"XMAS" | b"SAMX"))
        .count()
}

#[aoc(day4, part1)]
fn part1(crossword: &Crossword) -> usize {
    crossword
        .horizontals()
        .iter()
        .chain(crossword.verticals().iter())
        .chain(crossword.main_diagonals().iter())
        .chain(crossword.anti_diagonals().iter())
        .map(|scan_line| count_xmas(scan_line))
        .sum()
}

#[aoc(day4, part2)]
fn part2(crossword: &Crossword) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
