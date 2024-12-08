use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position(isize, isize);

#[derive(Debug, Clone)]
struct Map {
    antennas_by_frequency: HashMap<char, Vec<Position>>,
    map_bounds: Position,
}

impl Map {
    fn is_in_bounds(&self, p: &Position) -> bool {
        !(p.0 < 0 || p.1 < 0 || p.0 > self.map_bounds.0 || p.1 > self.map_bounds.1)
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Map {
    let mut map_bounds = Position(0, 0);
    let mut antennas_by_frequency = HashMap::<char, Vec<_>>::new();
    for (y, line) in input.lines().enumerate() {
        let y = y as isize;
        map_bounds.0 = y;
        for (x, c) in line.chars().enumerate() {
            let x = x as isize;
            map_bounds.1 = x;
            if c != '.' {
                let antenna_position = Position(y, x);
                if let Some(antennas) = antennas_by_frequency.get_mut(&c) {
                    antennas.push(antenna_position);
                } else {
                    let antennas = vec![antenna_position];
                    antennas_by_frequency.insert(c, antennas);
                }
            }
        }
    }
    Map {
        antennas_by_frequency,
        map_bounds,
    }
}

#[aoc(day8, part1)]
fn part1(input: &Map) -> String {
    let mut antinodes = HashSet::<Position>::new();
    for antennas in input.antennas_by_frequency.values() {
        for i in (0..antennas.len()).rev() {
            for j in 0..i {
                let a = &antennas[i];
                let b = &antennas[j];
                let d = (b.0 - a.0, b.1 - a.1);
                let mut try_insert_antinode = |position: Position| {
                    if input.is_in_bounds(&position) {
                        antinodes.insert(position);
                    }
                };
                try_insert_antinode(Position(a.0 - d.0, a.1 - d.1));
                try_insert_antinode(Position(b.0 + d.0, b.1 + d.1));
            }
        }
    }
    antinodes.len().to_string()
}

#[aoc(day8, part2)]
fn part2(input: &Map) -> String {
    let mut antinodes = HashSet::<Position>::new();
    for antennas in input.antennas_by_frequency.values() {
        for i in (0..antennas.len()).rev() {
            for j in 0..i {
                let a = &antennas[i];
                let b = &antennas[j];
                let d = (b.0 - a.0, b.1 - a.1);
                let mut insert_all_antinodes = |position: &Position, direction: isize| {
                    let mut k = 0;
                    loop {
                        let antinode = Position(position.0 + k * d.0, position.1 + k * d.1);
                        if !input.is_in_bounds(&antinode) {
                            break;
                        }
                        antinodes.insert(antinode);
                        k += direction;
                    }
                };
                insert_all_antinodes(a, -1);
                insert_all_antinodes(b, 1);
            }
        }
    }
    antinodes.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/2024/day8.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "14");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "34");
    }
}
