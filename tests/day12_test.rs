use aoc2021::day12::{input_generator, solve_part1, solve_part2};

#[test]
fn day12a_tiny() {
    let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    let puzzle = input_generator(input);
    assert_eq!(6, puzzle.nodes.len());
    assert_eq!(10, solve_part1(&puzzle))
}

#[test]
fn day12b_tiny() {
    let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    let puzzle = input_generator(input);
    assert_eq!(36, solve_part2(&puzzle))
}