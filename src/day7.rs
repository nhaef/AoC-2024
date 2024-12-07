use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
struct Equation {
    test_value: usize,
    numbers: Vec<usize>,
}

impl Equation {
    fn is_possibly_true_p1(&self) -> bool {
        self.is_possibly_true_recursive_p1(self.numbers[0], 1)
    }
    fn is_possibly_true_recursive_p1(&self, current_value: usize, i: usize) -> bool {
        if current_value > self.test_value {
            false
        } else if i == self.numbers.len() {
            current_value == self.test_value
        } else {
            self.is_possibly_true_recursive_p1(current_value + self.numbers[i], i + 1)
                || self.is_possibly_true_recursive_p1(current_value * self.numbers[i], i + 1)
        }
    }

    fn is_possibly_true_p2(&self) -> bool {
        self.is_possibly_true_recursive_p2(self.numbers[0], 1)
    }
    fn is_possibly_true_recursive_p2(&self, current_value: usize, i: usize) -> bool {
        if current_value > self.test_value {
            false
        } else if i == self.numbers.len() {
            current_value == self.test_value
        } else {
            self.is_possibly_true_recursive_p2(current_value + self.numbers[i], i + 1)
                || self.is_possibly_true_recursive_p2(current_value * self.numbers[i], i + 1)
                || self.is_possibly_true_recursive_p2(
                    concat_usize(current_value, self.numbers[i]),
                    i + 1,
                )
        }
    }
}

fn concat_usize(a: usize, b: usize) -> usize {
    let mut factor = 10;
    while factor <= b {
        factor *= 10;
    }
    a * factor + b
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (test_value, numbers) = line.split_once(':').unwrap();
            let test_value = test_value.parse().unwrap();
            let numbers = numbers
                .trim()
                .split(' ')
                .map(|v| v.parse().unwrap())
                .collect();
            Equation {
                test_value,
                numbers,
            }
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Equation]) -> String {
    input
        .iter()
        .filter(|equation| equation.is_possibly_true_p1())
        .map(|equation| equation.test_value)
        .sum::<usize>()
        .to_string()
}

#[aoc(day7, part2)]
fn part2(input: &[Equation]) -> String {
    input
        .iter()
        .filter(|equation| equation.is_possibly_true_p2())
        .map(|equation| equation.test_value)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/2024/day7.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "3749");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "11387");
    }
}
