use std::{collections::HashSet, hash::Hash};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug)]
enum Tile {
    Empty,
    Start,
    Obstruction,
}

#[derive(Clone, Debug)]
struct Map(Vec<Vec<Tile>>);

impl Map {
    fn get(&self, position: &(usize, usize)) -> Option<&Tile> {
        self.0.get(position.0).and_then(|row| row.get(position.1))
    }
    fn set(&mut self, position: &(usize, usize), tile: Tile) {
        self.0[position.0][position.1] = tile;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Direction {
    fn turn_right(&mut self) {
        *self = match self {
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
            Self::Up => Self::Right,
        }
    }

    fn apply(&self, position: &(usize, usize)) -> (usize, usize) {
        match self {
            Direction::Down => (position.0.overflowing_add(1).0, position.1),
            Direction::Left => (position.0, position.1.overflowing_sub(1).0),
            Direction::Right => (position.0, position.1.overflowing_add(1).0),
            Direction::Up => (position.0.overflowing_sub(1).0, position.1),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Guard {
    position: (usize, usize),
    facing: Direction,
}

impl Guard {
    fn next_position(&self) -> (usize, usize) {
        self.facing.apply(&self.position)
    }
    fn tick(&mut self, map: &Map) -> bool {
        let next_position = self.next_position();
        let tile = match map.get(&next_position) {
            None => return false,
            Some(t) => t,
        };
        match tile {
            Tile::Empty | Tile::Start => self.position = next_position,
            Tile::Obstruction => self.facing.turn_right(),
        }
        true
    }
    fn is_loop(&mut self, map: &Map) -> bool {
        let mut states = HashSet::new();
        while self.tick(map) {
            if states.contains(self) {
                return true;
            }
            states.insert(self.clone());
        }
        false
    }
}

type Input = (Map, Guard);

#[aoc_generator(day6)]
fn parse(input: &str) -> Input {
    let mut guard = None;
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Obstruction,
                    '^' => {
                        guard = Some(Guard {
                            position: (y, x),
                            facing: Direction::Up,
                        });
                        Tile::Start
                    }
                    c => panic!("unexpected character '{}'", c),
                })
                .collect()
        })
        .collect();

    (Map(map), guard.unwrap())
}

#[aoc(day6, part1)]
fn part1((map, guard): &Input) -> String {
    let mut guard = guard.clone();
    let mut visited_tiles = HashSet::new();

    loop {
        visited_tiles.insert(guard.position);
        if !guard.tick(map) {
            break;
        }
    }

    visited_tiles.len().to_string()
}

#[aoc(day6, part2)]
fn part2((map, guard_start): &Input) -> String {
    let mut guard = guard_start.clone();
    let mut new_obstructions = HashSet::new();

    while guard.tick(map) {
        let next_position = guard.next_position();

        match map.get(&next_position) {
            // break early if next_position is out of bounds
            None => break,
            // we must not place a new obstruction on the guard's start position
            Some(Tile::Start) => continue,
            // we must not place a new obstruction on an existing one
            Some(Tile::Obstruction) => continue,
            _ => (),
        }
        if new_obstructions.contains(&next_position) {
            // we already confirmed that this new obstruction would cause a loop
            continue;
        }

        // simulate new obstruction on next_position
        let mut map = map.clone();
        map.set(&next_position, Tile::Obstruction);
        if guard_start.clone().is_loop(&map) {
            new_obstructions.insert(next_position);
        }
    }

    new_obstructions.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/2024/day6.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "41");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "6");
    }
}
