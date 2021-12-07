
#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<u16> {
    input.split(",")
        .map(|f| u16::from_str_radix(f, 10))
        .flatten()
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(positions: &[u16]) -> u64 {
    let mut best_score: Option<u64> = None;
    for position in 0..2000 {
        let score = total_fuel_1(positions, position as u16);
        if best_score.is_none() ||
            best_score.unwrap() > score {
            best_score = Some(score)
        }
    }
    best_score.unwrap()
}

fn total_fuel_1(positions: &[u16], destination: u16) -> u64 {
    positions.iter()
        .map(|p| (destination as i16 - p.clone() as i16).abs() as u64)
        .sum()
}


#[aoc(day7, part2)]
pub fn solve_part2(positions: &[u16]) -> u64 {
    let mut best_score: Option<u64> = None;
    for position in 0..2000 {
        let score = total_fuel_2(positions, position as u16);
        if best_score.is_none() ||
            best_score.unwrap() > score {
            best_score = Some(score)
        }
    }
    best_score.unwrap()
}

fn total_fuel_2(positions: &[u16], destination: u16) -> u64 {
    positions.iter()
        .map(|p| (destination as i16 - p.clone() as i16).abs() as u64)
        .map(|p| p * (p+1) / 2)
        .sum()
}