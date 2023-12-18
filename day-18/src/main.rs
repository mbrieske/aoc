use day_18::part2::solve;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("res/input").unwrap();

    println!("{}", solve(&input));
}
