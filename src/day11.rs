use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<u64> {
    input.split(' ').map(|x| x.parse().unwrap()).collect()
}

fn blink(stones: &mut Vec<u64>) {
    let mut i = 0;
    while i < stones.len() {
        let stone = stones[i];
        if stone == 0 {
            stones[i] = 1;
        } else if count_digits(stone) % 2 == 0 {
            let divisor = 10u64.pow(count_digits(stone) / 2);
            stones[i] = stone / divisor;
            stones.insert(i + 1, stone % divisor);
            i += 1;
        } else {
            stones[i] = stone * 2024;
        }
        i += 1;
    }
}

fn count_digits(x: u64) -> u32 {
    x.ilog10() + 1
}

#[aoc(day11, part1)]
fn part1(stones: &Vec<u64>) -> usize {
    let mut stones = stones.clone();
    for _ in 0..25 {
        blink(&mut stones);
    }
    stones.len()
}

#[aoc(day11, part2)]
fn part2(stones: &Vec<u64>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let mut stones = parse("0 1 10 99 999");
        blink(&mut stones);
        assert_eq!(stones, vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }

    #[test]
    fn part1_example2() {
        let mut stones = parse("125 17");
        blink(&mut stones);
        assert_eq!(stones, vec![253000, 1, 7]);
        blink(&mut stones);
        assert_eq!(stones, vec![253, 0, 2024, 14168]);
        blink(&mut stones);
        assert_eq!(stones, vec![512072, 1, 20, 24, 28676032]);
        blink(&mut stones);
        assert_eq!(stones, vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);
        blink(&mut stones);
        assert_eq!(
            stones,
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );
        blink(&mut stones);
        assert_eq!(
            stones,
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), 0);
    }
}
