
#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<u16> {
    input.lines()
        .map(|l| l.split_whitespace().nth(4))
        .flatten()
        .map(|l| l.parse::<u16>())
        .flatten()
        .collect()
}

struct DeterministicDice {
    last_roll: u16,
    total_rolls: u32,
}
impl Iterator for DeterministicDice {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.last_roll + 1;
        if next > 100 {
            next = 1
        }
        self.last_roll = next;
        self.total_rolls += 1;
        Some(next)
    }
}

struct PlayboardPosition {
    position: u16,
    score: u32,
}
impl PlayboardPosition {
    fn add(&mut self, spaces: u16) {
        self.position += spaces;
        while self.position > 10 {
            self.position -= 10
        }
        self.score += self.position as u32
    }
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &[u16]) -> u32 {
    let mut dice = DeterministicDice{last_roll: 0, total_rolls: 0};
    dice_game(dice, input)
}

fn dice_game(mut dice: DeterministicDice, starting_positions: &[u16]) -> u32 {
    let mut positions: Vec<PlayboardPosition> = starting_positions.iter()
        .map(|p| PlayboardPosition{position: *p, score: 0})
        .collect();
    let mut current_player = 0usize;
    while !positions.iter().any(|p| p.score >= 1000) {
        let spaces = (0..3).fold(0u16, |acc, _| acc + dice.next().unwrap());
        positions[current_player].add(spaces);
        current_player += 1;
        if current_player >= positions.len() {
            current_player = 0;
        }
    }
    let loser = positions.iter()
        .filter(|p| p.score < 1000).nth(0).unwrap();
    loser.score * dice.total_rolls
}