use std::iter;

use aoc_runner_derive::{aoc, aoc_generator};

struct DiskMap {
    blocks: Vec<DiskBlock>,
}

#[derive(Debug, Clone)]
enum DiskBlock {
    Free,
    File(usize),
}

#[aoc_generator(day9)]
fn parse(input: &str) -> DiskMap {
    let blocks = input
        .trim_end()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .flat_map(|(i, v)| {
            if i % 2 == 0 {
                iter::repeat(DiskBlock::File(i / 2)).take(v as usize)
            } else {
                iter::repeat(DiskBlock::Free).take(v as usize)
            }
        })
        .collect();

    DiskMap {
        blocks,
    }
}

#[aoc(day9, part1)]
fn part1(disk_map: &DiskMap) -> String {
    let mut sum = 0;
    let mut i = 0;
    let mut j = disk_map.blocks.len();
    let mut disk_blocks = disk_map.blocks.iter();
    let mut disk_blocks_rev = disk_map.blocks.iter().rev();
    while i < j {
        match disk_blocks.next().unwrap() {
            DiskBlock::Free => {
                while i < j {
                    j -= 1;
                    match disk_blocks_rev.next().unwrap() {
                        DiskBlock::Free => (),
                        DiskBlock::File(file_id) => {
                            sum += i * file_id;
                            //println!("block {:03} file_id {:03}", i, file_id);
                            break;
                        },
                    }
                }  
            },
            DiskBlock::File(file_id) => {
                //println!("block {:03} file_id {:03}", i, file_id);
                sum += i * file_id;
            },
        }
        i += 1;
    }
    sum.to_string()
}

#[aoc(day9, part2)]
fn part2(input: &DiskMap) -> String {
    todo!()
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
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
