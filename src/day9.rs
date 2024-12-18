use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone)]
struct DiskSection {
    file_id: u64,
    start: u64,
    end: u64,
    moved: bool,
}

#[derive(Debug, Clone)]
struct DiskMap {
    sections: Vec<DiskSection>,
}

#[aoc_generator(day9)]
fn parse(input: &str) -> DiskMap {
    let mut sections = Vec::new();
    let mut file_id = 0u64;
    let mut block_index = 0u64;
    let mut free_space = false;
    for c in input.chars() {
        let size = c.to_digit(10).expect("bad digit") as u64;
        if !free_space {
            sections.push(DiskSection {
                file_id,
                start: block_index,
                end: block_index + size,
                moved: false,
            });
            file_id += 1;
        }
        block_index += size;
        free_space = !free_space;
    }
    DiskMap { sections }
}

impl DiskMap {
    fn compact_blocks(&mut self) -> &mut Self {
        while let Some((index, free_start, free_amount)) = self.next_free_range(1) {
            let last_section = self.sections.last_mut().unwrap();
            let last_file_id = last_section.file_id;
            let last_size = last_section.end - last_section.start;
            let new_section = if last_size < free_amount {
                self.sections.pop().unwrap();
                DiskSection {
                    file_id: last_file_id,
                    start: free_start,
                    end: free_start + last_size,
                    moved: true,
                }
            } else {
                last_section.end -= free_amount;
                DiskSection {
                    file_id: last_file_id,
                    start: free_start,
                    end: free_start + free_amount,
                    moved: true,
                }
            };
            self.sections.insert(index, new_section);
        }
        self
    }

    /// Find the next range of free blocks with at least `size` blocks, if any.
    /// Returns (index, start of range, number of free blocks)
    fn next_free_range(&self, min_size: u64) -> Option<(usize, u64, u64)> {
        let mut iter = self.sections.iter().enumerate().peekable();
        while let (Some((_, curr)), Some((next_index, next))) = (iter.next(), iter.peek()) {
            if curr.end + min_size <= next.start {
                return Some((*next_index, curr.end, next.start - curr.end));
            }
        }
        None
    }

    fn checksum(&self) -> u64 {
        self.sections
            .iter()
            .map(|section| section.file_id * (section.start..section.end).sum::<u64>())
            .sum()
    }
}

#[aoc(day9, part1)]
fn part1(input: &DiskMap) -> u64 {
    input.clone().compact_blocks().checksum()
}

impl DiskMap {
    fn compact_files(&mut self) -> &mut Self {
        while let Some((index, file)) = self.next_unmoved_file() {
            let size = file.end - file.start;
            if let Some((free_index, free_start, _)) = self.next_free_range(size) {
                // Must only move to the front
                if free_index <= index {
                    // Move the file
                    let mut file = self.sections.remove(index);
                    file.start = free_start;
                    file.end = free_start + size;
                    file.moved = true;
                    self.sections.insert(free_index, file);
                    continue;
                }
            }
            // Not enough free space left for this file, skip it.
            let file = &mut self.sections[index];
            file.moved = true;
        }
        self
    }

    fn next_unmoved_file(&self) -> Option<(usize, &DiskSection)> {
        self.sections
            .iter()
            .enumerate()
            .rev()
            .find(|(_, section)| !section.moved)
    }
}

#[aoc(day9, part2)]
fn part2(input: &DiskMap) -> u64 {
    input.clone().compact_files().checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 2858);
    }
}
