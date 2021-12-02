pub enum Direction {
    FORWARD,
    DOWN,
    UP,
}
pub struct Instruction {
    direction: Direction,
    distance: u32
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|line| parse_instruction(line))
        .flatten()      // filter out None
        .collect()
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() == 2 {
        let direction = match parts[0] {
            "forward" => Some(Direction::FORWARD),
            "down" => Some(Direction::DOWN),
            "up" => Some(Direction::UP),
            _ => None
        };
        let distance = parts[1].parse();
        if direction.is_some() && distance.is_ok() {
            Some(Instruction{direction: direction.unwrap(), distance: distance.unwrap()})
        } else {
            None
        }
    } else {
        None
    }
}

#[aoc(day2, part1)]
pub fn solve_part1(instructions: &[Instruction]) -> u32 {
    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;

    for instruction in instructions.iter() {
        match instruction.direction {
            Direction::UP => depth -= instruction.distance,
            Direction::DOWN => depth += instruction.distance,
            Direction::FORWARD => horizontal += instruction.distance
        }
    }
    horizontal * depth
}

#[aoc(day2, part2)]
pub fn solve_part2(instructions: &[Instruction]) -> u32 {
    let mut horizontal: u32 = 0;
    let mut aim: u32 = 0;
    let mut depth: u32 = 0;

    for instruction in instructions.iter() {
        match instruction.direction {
            Direction::UP => aim -= instruction.distance,
            Direction::DOWN => aim += instruction.distance,
            Direction::FORWARD => {
                horizontal += instruction.distance;
                depth += instruction.distance * aim
            }
        }
    }
    horizontal * depth
}