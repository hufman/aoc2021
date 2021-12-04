use std::borrow::Borrow;
use std::fmt;
use std::fmt::{Formatter, Write};

#[derive(Debug)]
pub struct BingoBoard {
    rows: Vec<Vec<u8>>
}
#[derive(Debug)]
pub struct Bingo {
    input: Vec<u8>,
    boards: Vec<BingoBoard>,
}

// mutable structs cloned from input
pub struct BingoSquare {
    num: u8,
    marked: bool,
}
impl fmt::Debug for BingoSquare {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.num.to_string().as_str());
        if self.marked {
            f.write_str("*")
        } else {
            f.write_str(" ")
        }
    }
}
#[derive(Debug)]
pub struct BingoBoardSolution {
    rows: Vec<Vec<BingoSquare>>
}
impl From<&BingoBoard> for BingoBoardSolution {
    fn from(input: &BingoBoard) -> Self {
        let rows: Vec<Vec<BingoSquare>> = input.rows.iter()
            .map(|row| row.iter().map(|n| BingoSquare{num: *n, marked: false})
                .collect())
            .collect();
        BingoBoardSolution {rows}
    }
}

impl BingoBoardSolution {
    fn is_bingo(&self) -> bool {
        // row checks
        let bingo_horizontal = self.rows.iter().any(|row|
            row.iter().all(|c| c.marked)
        );
        // column checks
        let bingo_vertical = (0..4).any(|col_index|
            self.rows.iter().all(|row|
                row[col_index].marked
            )
        );
        // slash checks
        let bingo_backslash = (0..4).all(|i|
            self.rows[i][i].marked
        );
        let bingo_slash = (0..4).all(|i|
            self.rows[i][4-i].marked
        );
        bingo_horizontal || bingo_vertical || bingo_backslash || bingo_slash
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Bingo {
    let mut sections = input.split("\n\n");
    let input: Vec<u8> = sections.nth(0).unwrap()
        .split(',')
        .map(|n| u8::from_str_radix(n, 10))
        .flatten()
        .collect();
    let boards: Vec<BingoBoard> = sections
        .map(|b| parse_board(b))
        .collect();

    Bingo {input, boards}
}

fn parse_board(input: &str) -> BingoBoard {
    let rows = input.lines()
        .map(|l| l.split_whitespace())
        .map(|row| row.map(|n| u8::from_str_radix(n, 10))
            .flatten()
            .collect())
        .collect();
    BingoBoard {rows}
}


#[aoc(day4, part1)]
pub fn solve_part1(bingo: &Bingo) -> u32 {
    let mut boards: Vec<BingoBoardSolution> = bingo.boards.iter()
        .map(|b| BingoBoardSolution::from(b))
        .collect();
    for n in bingo.input.as_slice() {
        boards.iter_mut().for_each(|b|
            mark_cell(b, *n)
        );
        for board in boards.iter() {
            if board.is_bingo() {
                return score_board(board.borrow()) * (*n as u32);
            }
        }
    }
    0
}

#[aoc(day4, part2)]
pub fn solve_part2(bingo: &Bingo) -> u32 {
    let mut boards: Vec<BingoBoardSolution> = bingo.boards.iter()
        .map(|b| BingoBoardSolution::from(b))
        .collect();
    for n in bingo.input.as_slice() {
        // print!("{} ->", n);
        boards.iter_mut().for_each(|b|
            mark_cell(b, *n)
        );
        // println!("{:?}", boards);
        if boards.len() == 1 && boards[0].is_bingo() {
            return score_board(boards[0].borrow()) * (*n as u32);
        }
        boards = boards.into_iter().filter(|b| !b.is_bingo()).collect();
    }
    0
}

fn mark_cell(board: &mut BingoBoardSolution, num: u8) {
    board.rows.iter_mut()
        .for_each(|row| row.iter_mut().for_each(|cell|
            if cell.num == num { cell.marked = true }
        ))
}

fn score_board(board: &BingoBoardSolution) -> u32 {
    board.rows.iter()
        .map(|row|
            row.iter()
                .filter(|c| !c.marked)
                .map(|c| c.num as u32)
                .sum::<u32>()
        ).sum::<u32>()
}