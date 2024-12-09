use std::fs::File;

use aoc::error::{AocError, Result};
use itertools::Itertools;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BlockType {
    FreeSpace(),
    File(FileID),
}

type FileID = usize;

struct FileStructure {
    disk_map: Vec<BlockType>,
}

impl FileStructure {
    fn new(input: &str) -> Self {
        let data: Vec<BlockType> = input
            .trim()
            .chars()
            .into_iter()
            .enumerate()
            .map(|(idx, amount)| {
                let amount = amount.to_digit(10).unwrap() as usize;

                match idx % 2 {
                    0 => vec![BlockType::File(idx / 2 as FileID); amount],
                    1 => vec![BlockType::FreeSpace(); amount],
                    _ => unreachable!(),
                }
            })
            .flatten()
            .collect();

        Self { disk_map: data }
    }

    fn stabilize(&mut self) {
        let mut free = self.disk_map.iter().position(|block| match block {
            BlockType::FreeSpace() => true,
            _ => false,
        });

        while let Some(free_idx) = free {
            let last_block = self.disk_map.pop().unwrap();

            if let BlockType::File(id) = last_block {
                self.disk_map[free_idx] = BlockType::File(id);
            }

            free = self.disk_map.iter().position(|block| match block {
                BlockType::FreeSpace() => true,
                _ => false,
            });
        }
    }

    fn stabilize_blocks(&mut self) -> Result<()> {
        let mut is_running = true;
        let mut last_index = 0;
        let mut steps = 0;

        while steps < 1000 && is_running {
            println!("last_index: {}", last_index);
            // find the first free block
            let free_idx = self
                .disk_map
                .iter()
                .skip(last_index)
                .position(|block| match block {
                    BlockType::FreeSpace() => true,
                    _ => false,
                })
                .ok_or(AocError::ComputeError("No free block found".to_owned()))?
                + last_index;

            // get the length of the free block
            let free_len = self
                .disk_map
                .iter()
                .skip(free_idx)
                .take_while(|block| match block {
                    BlockType::FreeSpace() => true,
                    _ => false,
                })
                .count();

            let mut file_ids = (0..self.disk_map.len())
                .map(|idx| match self.disk_map[idx] {
                    BlockType::File(id) => Some(id),
                    _ => None,
                })
                .filter(|id| id.is_some())
                .map(|id| id.unwrap())
                .chunk_by(|id| *id)
                .into_iter()
                .map(|(id, chunk)| (id, chunk.count()))
                .collect::<Vec<(FileID, usize)>>();

            file_ids.reverse();

            // find first file block that can fit in the free block
            let block = file_ids.iter().find(|(_, len)| *len <= free_len);

            // get the positions of the file block
            if let Some((id, len)) = block {
                let start_index = self
                    .disk_map
                    .iter()
                    .position(|block| match block {
                        BlockType::File(file_id) => *file_id == *id,
                        _ => false,
                    })
                    .ok_or(AocError::ComputeError("No file block found".to_owned()))?;

                println!("start_index: {}", start_index);

                // move the file block to the free block
                for idx in 0..*len {
                    self.disk_map[free_idx + idx] = BlockType::File(*id);
                    self.disk_map[start_index + idx] = BlockType::FreeSpace();
                }
            }

            last_index = free_idx + free_len;
            steps += 1;
        }

        Ok(())
    }

    fn checksum(&self) -> usize {
        self.disk_map
            .iter()
            .enumerate()
            .map(|(idx, block)| match block {
                BlockType::File(id) => idx * id,
                _ => 0,
            })
            .sum()
    }

    fn print(&self) {
        for block in &self.disk_map {
            match block {
                BlockType::FreeSpace() => print!("_"),
                BlockType::File(id) => print!("[{}]", id),
            }
        }
        println!();
    }
}

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let mut fs = FileStructure::new(input);

        fs.stabilize();
        let checksum = fs.checksum();

        Ok(checksum)
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let mut fs = FileStructure::new(input);

        fs.stabilize_blocks();
        let checksum = fs.checksum();

        Ok(checksum)
    }
}

fn main() {
    aoc::init_logging();

    let input = include_str!("../input.txt");
    let solution = aoc::Solution::new(input.to_string());

    let part1 = Part1;
    solution.run(&part1);

    let part2 = Part2;
    solution.run(&part2);
}

#[cfg(test)]
mod tests {
    use aoc::Part;

    use super::*;

    const SAMPLE: &str = r#"
2333133121414131402
"#;

    #[test]
    fn part1_sample_test() {
        let part1 = Part1;
        assert_eq!(part1.solve(SAMPLE).unwrap(), 1928);
    }

    #[test]
    fn part2_sample_test() {
        let part2 = Part2;
        assert_eq!(part2.solve(SAMPLE).unwrap(), 2858);
    }
}
