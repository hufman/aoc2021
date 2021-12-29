use std::collections::HashMap;
use std::ops::{Add, Mul};

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

#[derive(Copy, Clone, Debug)]
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
    let dice = DeterministicDice{last_roll: 0, total_rolls: 0};
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

struct DiceWinners {
    left: u64,
    right: u64,
}
impl Add for DiceWinners {
    type Output = DiceWinners;

    fn add(self, rhs: Self) -> Self::Output {
        DiceWinners{left: self.left + rhs.left, right: self.right + rhs.right}
    }
}
impl Mul<u16> for DiceWinners {
    type Output = DiceWinners;

    fn mul(self, rhs: u16) -> Self::Output {
        DiceWinners{left: self.left * rhs as u64, right: self.right * rhs as u64}
    }
}
impl DiceWinners {
    fn max(self) -> u64 {
        if self.left > self.right {
            self.left
        } else {
            self.right
        }
    }
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &[u16]) -> u64 {
    let positions: Vec<PlayboardPosition> = input.iter()
        .map(|p| PlayboardPosition{position: *p, score: 0})
        .collect();
    let winners = simulate_universal_winners(&[positions[0], positions[1]], 0);
    winners.max()
}

fn simulate_universal_winners(positions: &[PlayboardPosition; 2], next_player: usize) -> DiceWinners {
    if positions[0].score >= 21 {
        return DiceWinners{left:1, right:0}
    }
    if positions[1].score >= 21 {
        return DiceWinners{left:0, right:1}
    }
    let other_player = 1 - next_player;
    let rolls: Vec<u16> = (1..=3).map(|a| {
        (1..=3).map(|b| {
            (1..=3).map(|c| {
                a+b+c
            }).collect::<Vec<u16>>()
        }).flatten().collect::<Vec<u16>>()
    }).flatten().collect::<Vec<u16>>();
    let roll_counts: HashMap<u16, u16> = rolls.into_iter()
        .fold(HashMap::new(), |mut acc, moves| {
            *acc.entry(moves).or_default() += 1; acc
        });
    let scores: Vec<DiceWinners> = roll_counts.iter().map(|(roll, count)| {
        let mut new_score = positions[next_player];
        new_score.add(*roll);
        // println!("New score:{:?}", new_score);
        if next_player == 0 {
            simulate_universal_winners(&[new_score, positions[other_player]], other_player) * *count
        } else {
            simulate_universal_winners(&[positions[other_player], new_score], other_player) * *count
        }
    }).collect();
    scores.into_iter().fold(DiceWinners{left:0, right:0}, |acc, n| {
        acc + n
    })
}