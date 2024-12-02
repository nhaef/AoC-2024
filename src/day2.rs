use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<Report>;
type Report = Vec<i64>;
enum Levels {
    Increasing,
    Decreasing,
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.split(' ').map(|v| v.parse().unwrap()).collect())
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &Input) -> String {
    fn is_safe(report: &Report, levels: Levels) -> bool {
        let range = match levels {
            Levels::Increasing => 1..=3,
            Levels::Decreasing => -3..=-1,
        };

        report
            .iter()
            .skip(1)
            .zip(report.iter())
            .all(|(k, j)| range.contains(&(k - j)))
    }

    input
        .iter()
        .filter(|report| is_safe(report, Levels::Increasing) || is_safe(report, Levels::Decreasing))
        .count()
        .to_string()
}

#[aoc(day2, part2)]
fn part2(input: &Input) -> String {
    fn is_safe(report: &Report, levels: &Levels, detected_bad_level: bool) -> bool {
        let range = match levels {
            Levels::Increasing => 1..=3,
            Levels::Decreasing => -3..=-1,
        };

        let bad_level = report
            .iter()
            .skip(1)
            .zip(report.iter())
            .position(|(k, j)| !range.contains(&(k - j)));

        if bad_level.is_some() && detected_bad_level {
            false
        } else if let Some(i) = bad_level {
            let report_without_level_at = |i: usize| {
                let mut report = report.clone();
                report.remove(i);
                report
            };

            let report_a = report_without_level_at(i + 1);
            let report_b = report_without_level_at(i);

            is_safe(&report_a, levels, true) || is_safe(&report_b, levels, true)
        } else {
            true
        }
    }

    input
        .iter()
        .filter(|report| {
            is_safe(report, &Levels::Increasing, false)
                || is_safe(report, &Levels::Decreasing, false)
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/2024/day2.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "2");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "4");
    }
}
