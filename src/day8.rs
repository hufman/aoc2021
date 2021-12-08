
#[derive(Debug)]
pub struct Scramble {
    pub signals: Vec<String>,
    pub digits: Vec<String>,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Scramble> {

    input.lines()
        .map(|l| {
            let fields: Vec<&str> = l.split("|").collect();
            let signals = fields[0].split_whitespace().map(String::from).collect();
            let digits = fields[1].split_whitespace().map(String::from).collect();
            Scramble{signals, digits}
        })
        .collect()
}


#[aoc(day8, part1)]
pub fn solve_part1(scrambles: &[Scramble]) -> u32 {
    scrambles.into_iter()
        .map(|s| s.digits.iter()
            .filter(|d| is_part1_digit(d))
            .count() as u32)
        .sum()
}

fn is_part1_digit(digit: &str) -> bool {
    let size = digit.len();
    size == 2 || size == 3 || size == 4 || size == 7
}