#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines()
        .map(|line| line.to_string())
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(lines: &[String]) -> u32 {
    // let lines: &[&str] = input.lines().collect();
    let gamma_str: String = lines.first()
        .unwrap()
        .chars()
        .enumerate()
        .map(|(i, _)| if count_bits(lines, i) < lines.len()/2 { '0' } else { '1' })
        .collect();
    let gamma = u32::from_str_radix(gamma_str.as_str(), 2).unwrap();
    let epsilon_str: String = gamma_str.chars()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect();
    let epsilon = u32::from_str_radix(epsilon_str.as_str(), 2).unwrap();
    gamma * epsilon
}

fn count_bits(lines: &[String], bit: usize) -> usize {
    lines.iter()
        .filter(|l| l.chars().nth(bit) == Some('1'))
        .count()
}