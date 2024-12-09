use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

type DiskMap = Vec<DiskSegment>;

#[derive(Debug, Clone)]
enum DiskBlock {
    Free,
    File(usize),
}

#[derive(Debug, Clone)]
struct DiskSegment {
    block_size: usize,
    block: DiskBlock,
}

#[aoc_generator(day9)]
fn parse(input: &str) -> DiskMap {
    input
        .trim_end()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .filter(|(_, block_size)| *block_size != 0)
        .map(|(i, block_size)| DiskSegment {
            block_size,
            block: if i % 2 == 0 {
                DiskBlock::File(i / 2)
            } else {
                DiskBlock::Free
            },
        })
        .collect()
}

struct DiskReader<'a> {
    rev: bool,
    segments: &'a [DiskSegment],
    segment_idx: usize,
    segment_buffer: Option<DiskSegment>,
    block_idx: usize,
    last_block_idx: usize,
}

impl<'a> DiskReader<'a> {
    fn new(segments: &'a [DiskSegment]) -> Self {
        DiskReader {
            rev: false,
            segments,
            segment_idx: 0,
            segment_buffer: None,
            block_idx: 0,
            last_block_idx: 0,
        }
    }

    fn new_rev(segments: &'a [DiskSegment]) -> Self {
        let block_idx = segments.iter().map(|s| s.block_size).sum();
        DiskReader {
            rev: true,
            segments,
            segment_idx: segments.len() - 1,
            segment_buffer: None,
            block_idx,
            last_block_idx: block_idx,
        }
    }

    fn find_file_with_max_size(
        &mut self,
        max_size: usize,
        found_files: &mut HashSet<usize>,
    ) -> Option<(usize, usize)> {
        while let Some(segment) = self.next_segment() {
            match segment.block {
                DiskBlock::Free => continue,
                DiskBlock::File(id) => {
                    if segment.block_size > max_size || !found_files.insert(id) {
                        continue;
                    }
                    return Some((id, segment.block_size));
                }
            }
        }
        None
    }

    fn next_segment(&mut self) -> Option<DiskSegment> {
        let segment = match self.segment_buffer.take() {
            Some(b) => b,
            None => match self.segments.get(self.segment_idx) {
                None => return None,
                Some(s) => s.clone(),
            },
        };
        self.inc_or_dec_segment_idx();
        self.inc_or_dec_block_idx_by(segment.block_size);
        Some(segment)
    }

    fn next_block(&mut self) -> Option<DiskBlock> {
        let segment_buffer = match self.segment_buffer.as_mut() {
            Some(b) => b,
            None => match self.segments.get(self.segment_idx) {
                None => return None,
                Some(s) => {
                    self.inc_or_dec_segment_idx();
                    self.segment_buffer = Some(s.clone());
                    self.segment_buffer.as_mut().unwrap()
                }
            },
        };

        let block = segment_buffer.block.clone();
        segment_buffer.block_size -= 1;
        if segment_buffer.block_size == 0 {
            self.segment_buffer = None;
        }
        self.inc_or_dec_block_idx_by(1);

        Some(block)
    }

    fn inc_or_dec_segment_idx(&mut self) {
        self.segment_idx = if self.rev {
            self.segment_idx.wrapping_sub(1)
        } else {
            self.segment_idx.wrapping_add(1)
        };
    }

    fn inc_or_dec_block_idx_by(&mut self, v: usize) {
        self.last_block_idx = self.block_idx;
        self.block_idx = if self.rev {
            self.block_idx.wrapping_sub(v)
        } else {
            self.block_idx.wrapping_add(v)
        };
    }
}

#[aoc(day9, part1)]
fn part1(disk_map: &DiskMap) -> String {
    let mut reader = DiskReader::new(disk_map);
    let mut reader_rev = DiskReader::new_rev(disk_map);
    let mut sum = 0;
    while reader.block_idx < reader_rev.block_idx {
        match reader.next_block().unwrap() {
            DiskBlock::File(id) => sum += id * reader.last_block_idx,
            DiskBlock::Free => {
                while reader.block_idx < reader_rev.block_idx {
                    match reader_rev.next_block().unwrap() {
                        DiskBlock::File(id) => {
                            sum += id * reader.last_block_idx;
                            break;
                        }
                        DiskBlock::Free => (),
                    }
                }
            }
        }
    }

    sum.to_string()
}

#[aoc(day9, part2)]
fn part2(disk_map: &DiskMap) -> String {
    let mut reader = DiskReader::new(disk_map);
    let mut found_files = HashSet::new();
    let mut sum = 0;
    while let Some(segment) = reader.next_segment() {
        match segment.block {
            DiskBlock::File(id) => {
                if !found_files.insert(id) {
                    continue;
                }
                for k in reader.last_block_idx..reader.block_idx {
                    sum += id * k;
                }
            }
            DiskBlock::Free => {
                let mut reader_rev = DiskReader::new_rev(disk_map);
                let mut read_blocks = 0;
                while let Some((id, size)) = reader_rev
                    .find_file_with_max_size(segment.block_size - read_blocks, &mut found_files)
                {
                    let i = reader.last_block_idx + read_blocks;
                    read_blocks += size;
                    let j = reader.last_block_idx + read_blocks;
                    for k in i..j {
                        sum += id * k;
                    }
                }
            }
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/2024/day9.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "1928");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "2858");
    }
}
