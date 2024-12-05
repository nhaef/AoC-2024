//! The following solutions make use of the assumption that
//! for any two pages a and b in any update, there
//! must be a rule (a, b) or (b, a) in the ordering rules.
//! This assumption is not guaranteed by the AoC problem statement,
//! but it is true for the input data.

use aoc_runner_derive::{aoc, aoc_generator};

type PageNumber = usize;
type PageOrderingRule = (PageNumber, PageNumber);
struct Input {
    ordering_rules: Vec<PageOrderingRule>,
    updates: Vec<Vec<PageNumber>>,
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    let mut input = input.lines();

    let ordering_rules = input
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (before, after) = line.split_once('|').unwrap();
            let before = before.parse().unwrap();
            let after = after.parse().unwrap();
            (before, after)
        })
        .collect();

    let updates = input
        .map(|line| line.split(',').map(|page| page.parse().unwrap()).collect())
        .collect();

    Input {
        ordering_rules,
        updates,
    }
}

fn is_update_ok(update: &[PageNumber], ordering_rules: &[PageOrderingRule]) -> bool {
    update
        .windows(2)
        .all(|w| ordering_rules.contains(&(w[0], w[1])))
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> String {
    input
        .updates
        .iter()
        .filter(|update| is_update_ok(update, &input.ordering_rules))
        .map(|update| update[update.len() / 2])
        .sum::<usize>()
        .to_string()
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> String {
    input
        .updates
        .iter()
        .filter(|update| !is_update_ok(update, &input.ordering_rules))
        .map(|update| {
            update.iter().fold(Vec::new(), |mut fixed_update, page| {
                let insert_at = if fixed_update.is_empty() {
                    0
                } else {
                    fixed_update
                        .iter()
                        .position(|p| !input.ordering_rules.contains(&(*p, *page)))
                        .unwrap_or(fixed_update.len())
                };
                fixed_update.insert(insert_at, *page);
                fixed_update
            })
        })
        .map(|update| update[update.len() / 2])
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/2024/day5.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "143");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "123");
    }
}
