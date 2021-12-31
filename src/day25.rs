use std::fmt::{Display, Formatter};
// use itertools::Itertools;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    LEFT,
    UP,
    RIGHT,
    DOWN,
    EMPTY
}
impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::LEFT => write!(f, "<"),
            Direction::UP => write!(f, "^"),
            Direction::RIGHT => write!(f, ">"),
            Direction::DOWN => write!(f, "v"),
            _ => write!(f, "."),
        }
    }
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<Vec<Direction>> {
    input.lines()
        .map(|line| line.chars()
            .map(|c| {
                match c {
                    '<' => Direction::LEFT,
                    '^' => Direction::UP,
                    'v' => Direction::DOWN,
                    '>' => Direction::RIGHT,
                    _ => Direction::EMPTY,
                }
            })
            .collect::<Vec<Direction>>())
        .collect()
}

#[aoc(day25, part1)]
pub fn solve_part1(grid: &Vec<Vec<Direction>>) -> u32 {
    let mut solution: Vec<Vec<Direction>> = grid.into_iter()
        .map(|row| row.to_vec())
        .collect();
    find_stalemate(&mut solution)
}

/*
fn print_grid(grid: &Vec<Vec<Direction>>) {
    let output = grid.iter()
        .map(|line| {
            line.iter().join("")
        })
        .join("\n");
    println!("{}\n", output);
}
 */

fn find_stalemate(mut grid: &mut Vec<Vec<Direction>>) -> u32 {
    let mut steps = 0;
    let mut moved = true;
    while moved {
        moved = false;
        moved = move_step(&mut grid, Direction::RIGHT) || moved;
        moved = move_step(&mut grid, Direction::DOWN) || moved;
        steps += 1;
        // print_grid(grid);
    }
    steps
}

fn move_step(grid: &mut Vec<Vec<Direction>>, direction: Direction) -> bool {
    let mut moved = false;
    if direction == Direction::RIGHT {
        for y in 0..grid.len() {
            let movings: Vec<usize> = (0..grid[y].len())
                .filter(|&x| {
                    let new_x = (x+1) % grid[y].len();
                    grid[y][x] == Direction::RIGHT &&
                    grid[y][new_x] == Direction::EMPTY
                })
                .collect();
            moved |= !movings.is_empty();
            movings.into_iter()
                .for_each(|x| {
                    let new_x = (x+1) % grid[y].len();
                    grid[y][x] = Direction::EMPTY;
                    grid[y][new_x] = Direction::RIGHT;
                });
        }
    }
    if direction == Direction::DOWN {
        for x in 0..grid[0].len() {
            let movings: Vec<usize> = (0..grid.len())
                .filter(|&y| {
                    let new_y = (y+1) % grid.len();
                    grid[y][x] == Direction::DOWN &&
                    grid[new_y][x] == Direction::EMPTY
                })
                .collect();
            moved |= !movings.is_empty();
            movings.into_iter()
                .for_each(|y| {
                    let new_y = (y+1) % grid.len();
                    grid[y][x] = Direction::EMPTY;
                    grid[new_y][x] = Direction::DOWN;
                });
        }
    }
    moved
}