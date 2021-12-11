use aoc2021::day11::{input_generator, solve_part1};

#[test]
fn day11a_example() {
    let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    let data = input_generator(input);
    assert_eq!(1656, solve_part1(&data));
}