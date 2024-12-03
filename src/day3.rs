use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day3, part1)]
fn part1(input: &str) -> String {
    fn try_eval(program: &str, offset: usize) -> Option<u64> {
        enum State {
            Init,
            MulCharM,
            MulCharU,
            MulCharL,
            MulNumberA,
            MulNumberB,
        }

        let mut state = State::Init;
        let mut reg_number_start = 0;
        let mut reg_number_a = 0;

        for (i, c) in program.char_indices().skip(offset) {
            match state {
                State::Init if c == 'm' => {
                    state = State::MulCharM;
                }
                State::MulCharM if c == 'u' => {
                    state = State::MulCharU;
                }
                State::MulCharU if c == 'l' => {
                    state = State::MulCharL;
                }
                State::MulCharL if c == '(' => {
                    state = State::MulNumberA;
                    reg_number_start = i + 1;
                }
                State::MulNumberA => {
                    if c == ',' {
                        if let Ok(number_a) = program[reg_number_start..i].parse::<u64>() {
                            state = State::MulNumberB;
                            reg_number_start = i + 1;
                            reg_number_a = number_a;
                        } else {
                            break;
                        }
                    }
                }
                State::MulNumberB => {
                    if c == ')' {
                        return if let Ok(number_b) = program[reg_number_start..i].parse::<u64>() {
                            Some(reg_number_a * number_b)
                        } else {
                            None
                        };
                    }
                }
                _ => break,
            }
        }

        None
    }

    input
        .char_indices()
        .filter_map(|(i, _)| try_eval(input, i))
        .sum::<u64>()
        .to_string()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> String {
    enum Token {
        Do,
        DoNot,
        Mul(u64),
    }

    fn try_eval(program: &str, offset: usize) -> Option<Token> {
        enum State {
            DoCharD,
            DoCharO,
            DontCharN,
            DontCharApostrophe,
            Init,
            MulCharM,
            MulCharU,
            MulCharL,
            MulNumberA,
            MulNumberB,
        }

        let mut state = State::Init;
        let mut reg_number_start = 0;
        let mut reg_number_a = 0;

        for (i, c) in program.char_indices().skip(offset) {
            match state {
                State::Init if c == 'm' => {
                    state = State::MulCharM;
                }
                State::MulCharM if c == 'u' => {
                    state = State::MulCharU;
                }
                State::MulCharU if c == 'l' => {
                    state = State::MulCharL;
                }
                State::MulCharL if c == '(' => {
                    state = State::MulNumberA;
                    reg_number_start = i + 1;
                }
                State::MulNumberA => {
                    if c == ',' {
                        if let Ok(number_a) = program[reg_number_start..i].parse::<u64>() {
                            state = State::MulNumberB;
                            reg_number_start = i + 1;
                            reg_number_a = number_a;
                        } else {
                            break;
                        }
                    }
                }
                State::MulNumberB => {
                    if c == ')' {
                        return if let Ok(number_b) = program[reg_number_start..i].parse::<u64>() {
                            Some(Token::Mul(reg_number_a * number_b))
                        } else {
                            None
                        };
                    }
                }
                State::Init if c == 'd' => {
                    state = State::DoCharD;
                }
                State::DoCharD if c == 'o' => {
                    state = State::DoCharO;
                }
                State::DoCharO => {
                    if c == 'n' {
                        state = State::DontCharN;
                    } else {
                        return Some(Token::Do);
                    }
                }
                State::DontCharN if c == '\'' => {
                    state = State::DontCharApostrophe;
                }
                State::DontCharApostrophe if c == 't' => {
                    return Some(Token::DoNot);
                }
                _ => break,
            }
        }

        None
    }

    let mut sum = 0;
    let mut enabled = true;
    for (i, _) in input.char_indices() {
        match try_eval(input, i) {
            Some(Token::Do) => enabled = true,
            Some(Token::DoNot) => enabled = false,
            Some(Token::Mul(v)) => {
                if enabled {
                    sum += v
                }
            }
            None => (),
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/2024/day3.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "161");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "48");
    }
}
