use std::{fs::File, iter::Skip};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BlockTypeChunk {
    FreeSpace(usize),
    File(FileID, usize),
}

type FileID = usize;

struct FileStructure {
    disk_map: Vec<BlockType>,
    disk_map_chunks: Vec<BlockTypeChunk>,
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

        let data_chunked = data.iter().fold(vec![], |mut acc, block| {
            match block {
                BlockType::FreeSpace() => {
                    if let Some(BlockTypeChunk::FreeSpace(len)) = acc.last_mut() {
                        *len += 1;
                    } else {
                        acc.push(BlockTypeChunk::FreeSpace(1));
                    }
                }
                BlockType::File(id) => {
                    if let Some(BlockTypeChunk::File(file_id, len)) = acc.last_mut() {
                        if *file_id == *id {
                            *len += 1;
                        } else {
                            acc.push(BlockTypeChunk::File(*id, 1));
                        }
                    } else {
                        acc.push(BlockTypeChunk::File(*id, 1));
                    }
                }
            }

            acc
        });

        Self {
            disk_map: data,
            disk_map_chunks: data_chunked,
        }
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

    fn stabilize_chunks(&mut self) -> Result<()> {
        let mut index = 0;

        while index < self.disk_map_chunks.len() {
            let chunk = self.disk_map_chunks[index];
            if let BlockTypeChunk::FreeSpace(len) = chunk {
                let mut free_len = len;

                // find a file chunk that can fit the free space, starting from the end
                let file = self
                    .disk_map_chunks
                    .iter()
                    .copied()
                    .enumerate()
                    .skip(index)
                    .rev()
                    .find(|(idx, block)| {
                        if let BlockTypeChunk::File(_, len) = block {
                            if *len <= free_len {
                                return true;
                            }
                        }

                        false
                    });

                if let Some((file_idx, BlockTypeChunk::File(id, len))) = file {
                    self.disk_map_chunks.swap(index, file_idx);

                    if len < free_len {
                        self.disk_map_chunks[file_idx] = BlockTypeChunk::FreeSpace(len);
                        self.disk_map_chunks
                            .insert(index + 1, BlockTypeChunk::FreeSpace(free_len - len));
                    }
                }
            }

            index += 1;
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

    fn expand_chunks(&self) -> Vec<BlockType> {
        self.disk_map_chunks
            .iter()
            .map(|block| match block {
                BlockTypeChunk::FreeSpace(len) => vec![BlockType::FreeSpace(); *len],
                BlockTypeChunk::File(id, len) => vec![BlockType::File(*id); *len],
            })
            .flatten()
            .collect_vec()
    }

    fn checksum_chunks(&self) -> usize {
        self.expand_chunks()
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
        println!("\n\n");
    }

    fn print_chunks(&self) {
        for block in self.expand_chunks() {
            match block {
                BlockType::FreeSpace() => print!("_"),
                BlockType::File(id) => print!("[{}]", id),
            }
        }
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

        fs.stabilize_chunks();
        let checksum = fs.checksum_chunks();

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
