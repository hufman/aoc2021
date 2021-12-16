use crate::day15::GridNodeStatus::{PENDING, UNVISITED};
use crate::helpers::Point;

#[derive(Clone, PartialEq, Debug)]
enum GridNodeStatus {
    UNVISITED,
    PENDING(u32),
    VISITED(u32),
}
#[derive(Clone, Debug)]
struct GridNode {
    status: GridNodeStatus,
    point: Point,
}
#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|line| line.chars()
            .map(|c| c.to_digit(10))
            .flatten()
            .map(|c| c as u8)
            .collect::<Vec<u8>>())
        .collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(grid: &Vec<Vec<u8>>) -> u32 {
    let start = Point{x:0, y:0};
    let y = grid.len() - 1;
    let end = Point{x: grid[y].len() - 1, y: y};
    crawl_grid(grid, &start, &end)
}

pub fn crawl_grid(grid: &Vec<Vec<u8>>, start: &Point, end: &Point) -> u32 {
    let mut visit_status = Vec::new();
    for y in 0..grid.len() {
        visit_status.push(vec![GridNodeStatus::UNVISITED; grid[y].len()])
    }
    let mut pending: Vec<GridNode> = Vec::new();

    pending.extend(
        crawl_around(grid, start).into_iter()
        .map(|p| GridNode{point: p, status: GridNodeStatus::PENDING(grid[p.y][p.x] as u32)})
    );

    while !pending.is_empty() {
        pending.sort_by_key(|n| if let GridNodeStatus::PENDING(weight) = n.status {u32::MAX - weight} else {panic!("unknown node in pending queue")});
        let next_node = pending.pop().unwrap();
        let next_distance = if let GridNodeStatus::PENDING(weight) = next_node.status {weight} else {panic!("unknown node in pending queue")};
        visit_status[next_node.point.y][next_node.point.x] = GridNodeStatus::VISITED(next_distance);

        if next_node.point == *end {
            pending.clear()
        } else {
            let new_pending: Vec<GridNode> = crawl_around(grid, &next_node.point).into_iter()
                .filter(|p| visit_status[p.y][p.x] == UNVISITED)
                .map(|p| GridNode{point: p, status: GridNodeStatus::PENDING(next_distance + grid[p.y][p.x] as u32)})
                .collect();
            new_pending.iter().for_each(|n| visit_status[n.point.y][n.point.x] = n.status.clone());
            pending.extend(new_pending);

        }
    }

    if let GridNodeStatus::VISITED(weight) = visit_status[end.y][end.x] {weight} else {panic!("Could not find solution")}
}

fn crawl_around(grid: &Vec<Vec<u8>>, point: &Point) -> Vec<Point> {
    let mut neighbors = Vec::new();
    if point.y > 0 {
        neighbors.push(Point{x: point.x, y: point.y-1})
    }
    if point.x > 0 {
        neighbors.push(Point{x: point.x-1, y: point.y})
    }
    if point.y < grid.len() - 1 {
        neighbors.push(Point{x: point.x, y: point.y+1})
    }
    if point.x < grid[point.y].len() - 1 {
        neighbors.push(Point{x: point.x+1, y: point.y})
    }
    neighbors.sort_by_key(|p| grid[p.y][p.x]);
    neighbors
}