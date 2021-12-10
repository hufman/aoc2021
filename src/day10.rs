
#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines()
        .map(|s| s.to_string())
        .collect()
}

pub enum ParseStatus {
    Opened,
    Closed(ParseResult),
    Unknown,
}
pub enum ParseResult {
    Corrupted(char, char),
    Incomplete(char),
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
    ParseResult::Ok
}

fn confirm_pop(stack: &mut Vec<char>, ending: char) -> ParseResult {
    if stack.len() == 0 {
        ParseResult::Incomplete(ending)
    } else {
        let popped = stack.pop().unwrap();
        let expected = match popped {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => popped
        };
        if expected == ending {
            ParseResult::Ok
        } else {
            ParseResult::Corrupted(expected, ending)
        }
    }
}

fn score(char: char) -> u32 {
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
        .map(|c| score(c))
        .sum()
}