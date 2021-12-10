
#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines()
        .map(|s| s.to_string())
        .collect()
}

#[derive(Debug, PartialEq)]
pub enum ParseStatus {
    Opened,
    Closed(ParseResult),
    Unknown,
}
#[derive(Debug, PartialEq)]
pub enum ParseResult {
    Corrupted(char, char),
    Incomplete(Vec<char>),
    Ok,
}
pub fn check_line(input: &str) -> ParseResult {
    let mut stack: Vec<char> = Vec::new();
    for i in input.chars() {
        let status = match i {
            '(' => { stack.push(i); ParseStatus::Opened },
            '[' => { stack.push(i); ParseStatus::Opened },
            '{' => { stack.push(i); ParseStatus::Opened },
            '<' => { stack.push(i); ParseStatus::Opened },
            ')' => ParseStatus::Closed(confirm_pop(&mut stack, i)),
            ']' => ParseStatus::Closed(confirm_pop(&mut stack, i)),
            '}' => ParseStatus::Closed(confirm_pop(&mut stack, i)),
            '>' => ParseStatus::Closed(confirm_pop(&mut stack, i)),
            _ => ParseStatus::Unknown,
        };
        match status {
            ParseStatus::Closed(result) => {
                if matches!(result, ParseResult::Incomplete(_) | ParseResult::Corrupted(_, _)) {
                    return result
                }
            }
            _ => ()
        }
    }
    if stack.len() > 0 {
        ParseResult::Incomplete(stack.into_iter().rev().map(|c| get_ending(c)).collect())
    }  else {
        ParseResult::Ok
    }
}

fn get_ending(opening: char) -> char {
    match opening {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => opening
    }
}

fn confirm_pop(stack: &mut Vec<char>, ending: char) -> ParseResult {
    if stack.len() == 0 {
        ParseResult::Incomplete(Vec::new())
    } else {
        let popped = stack.pop().unwrap();
        let expected = get_ending(popped);
        if expected == ending {
            ParseResult::Ok
        } else {
            ParseResult::Corrupted(expected, ending)
        }
    }
}

fn score_part1(char: char) -> u32 {
    match char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}

#[aoc(day10, part1)]
pub fn solve_part1(lines: &[String]) -> u32 {
    lines.iter()
        .map(|l| check_line(l))
        .filter_map(|l| match l {
            ParseResult::Corrupted(_, ending) => Some(ending),
            _ => None
        })
        .map(|c| score_part1(c))
        .sum()
}

fn score_part2(char: char) -> u64 {
    match char {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0
    }
}
#[aoc(day10, part2)]
pub fn solve_part2(lines: &[String]) -> u64 {
    let mut scores: Vec<u64> = lines.iter()
        .map(|l| check_line(l))
        .filter_map(|l| match l {
            ParseResult::Incomplete(missing) => Some(missing),
            _ => None
        })
        .map(|missing| {
            missing.into_iter().fold(0, |a, c| a*5 + score_part2(c))
        })
        .collect();
    scores.sort();
    scores[scores.len()/2]
}