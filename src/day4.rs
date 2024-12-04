use aoc_runner_derive::{aoc, aoc_generator};

fn search_word(
    input: &[Vec<char>],
    word: &[char],
    mut position: (usize, usize),
    direction: (isize, isize),
) -> bool {
    word.iter().all(
        |word_c| match input.get(position.0).and_then(|line| line.get(position.1)) {
            Some(input_c) if word_c == input_c => {
                let dy = (position.0 as isize) + direction.0;
                let dx = (position.1 as isize) + direction.1;
                position = (dy as usize, dx as usize);
                true
            }
            _ => false,
        },
    )
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Vec<char>]) -> String {
    const WORD: [char; 4] = ['X', 'M', 'A', 'S'];
    input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().flat_map(move |(x, _)| {
                [
                    search_word(input, &WORD, (y, x), (-1, -1)),
                    search_word(input, &WORD, (y, x), (-1, 1)),
                    search_word(input, &WORD, (y, x), (-1, 0)),
                    search_word(input, &WORD, (y, x), (1, -1)),
                    search_word(input, &WORD, (y, x), (1, 1)),
                    search_word(input, &WORD, (y, x), (1, 0)),
                    search_word(input, &WORD, (y, x), (0, 1)),
                    search_word(input, &WORD, (y, x), (0, -1)),
                ]
            })
        })
        .filter(|b| *b)
        .count()
        .to_string()
}

#[aoc(day4, part2)]
fn part2(input: &[Vec<char>]) -> String {
    const WORD: [char; 3] = ['M', 'A', 'S'];
    input
        .iter()
        .enumerate()
        .skip(2)
        .flat_map(|(y, line)| {
            line.iter().enumerate().skip(2).map(move |(x, _)| {
                (search_word(input, &WORD, (y, x - 2), (-1, 1))
                    || search_word(input, &WORD, (y - 2, x), (1, -1)))
                    && (search_word(input, &WORD, (y, x), (-1, -1))
                        || search_word(input, &WORD, (y - 2, x - 2), (1, 1)))
            })
        })
        .filter(|b| *b)
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/2024/day4.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "18");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "9");
    }
}
