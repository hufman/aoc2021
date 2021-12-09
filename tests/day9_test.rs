use aoc2021::day9::{input_generator, solve_part1};

#[test]
fn day4a_example() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
    let data = input_generator(input);
    assert_eq!(15, solve_part1(data.as_slice()));
}
