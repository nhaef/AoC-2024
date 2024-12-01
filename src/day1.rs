use aoc_runner_derive::{aoc, aoc_generator};

type Input = (Vec<u32>, Vec<u32>);

#[aoc_generator(day1)]
fn parse(input: &str) -> Input {
    let (mut left_list, mut right_list) = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("   ").expect("missing delimiter");
            let left = left.parse::<u32>().expect("not a number");
            let right = right.parse::<u32>().expect("not a number");
            (left, right)
        })
        .collect::<Input>();

    left_list.sort();
    right_list.sort();

    (left_list, right_list)
}

#[aoc(day1, part1)]
fn part1(input: &Input) -> String {
    let (left_list, right_list) = input;

    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum::<u32>()
        .to_string()
}

#[aoc(day1, part2)]
fn part2(input: &Input) -> String {
    let (left_list, right_list) = input;

    let mut right_list = right_list.iter().peekable();
    let mut last_similarity_score: Option<(u32, u32)> = None;

    left_list
        .iter()
        .map(|left| {
            // early return if similarity score has been calculated in the previous iteration
            if let Some((last_left, last_score)) = last_similarity_score {
                if *left == last_left {
                    return last_score;
                }
            }
           
            // calculate similarity score by counting all occurrences of left in right_list
            let mut similarity_score = 0;
            while let Some(right) = right_list.peek() {
                match left.cmp(right) {
                    std::cmp::Ordering::Less => break,
                    std::cmp::Ordering::Equal => similarity_score += left,
                    std::cmp::Ordering::Greater => (),
                }
                right_list.next();
            }
            last_similarity_score = Some((*left, similarity_score));
            similarity_score
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/2024/day1.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "11");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "31");
    }
}
