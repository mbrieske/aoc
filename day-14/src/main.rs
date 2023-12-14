// use day_14::part1::solve;
use day_14::part2::solve;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("res/input").unwrap();

    println!("{}", solve(&input, 1_000_000_000)); //1_000_000_000
}
