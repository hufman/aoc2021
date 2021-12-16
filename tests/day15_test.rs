use aoc2021::day15::{input_generator, solve_part1, solve_part2};

#[test]
fn day15a_example() {
    let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    let grid = input_generator(input);
    assert_eq!(40, solve_part1(&grid))
}

#[test]
fn day15b_example() {
    let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    let grid = input_generator(input);
    assert_eq!(315, solve_part2(&grid))
}