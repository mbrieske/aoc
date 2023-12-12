use day_12::part1::solve as solve_p1;
use day_12::part2::solve as solve_p2;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("res/input").unwrap();
    for line in input.lines() {
        assert!(solve_p1(line) == solve_p2(line))
    }
}
