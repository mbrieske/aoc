use day_21::{part2::solve, utils::tracing_init};
use std::fs::read_to_string;

fn main() {
    tracing_init();
    let input = read_to_string("res/input").unwrap();

    println!("{}", solve(&input, 26501365));
}
