use aoc2021::day25::{input_generator, solve_part1};

#[test]
fn day25a_example_1() {
    let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
    let grid = input_generator(input);
    assert_eq!(58, solve_part1(&grid));
}