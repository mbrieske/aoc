use day_17::part2::solve;
// use day_xx::part2::solve;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("res/input").unwrap();

    println!("{}", solve(&input));
}
