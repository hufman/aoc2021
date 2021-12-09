
#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10))
            .flatten()
            .map(|c| c as u8).collect()
    }).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(lines: &[Vec<u8>]) -> u32 {
    let mut points: Vec<Point> = Vec::new();
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let current = lines[y][x];
            let l = x == 0 || lines[y][x-1] > current;
            let u = y == 0 || lines[y-1][x] > current;
            let r = x == lines[y].len() - 1 || lines[y][x+1] > current;
            let d = y == lines.len() - 1 || lines[y+1][x] > current;
            if l && u && r && d {
                points.push(Point{x, y})
            }
        }
    }
    points.iter().map(|p| lines[p.y][p.x] as u32 + 1).sum()
}
