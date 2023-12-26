use day_23::{part2::solve, utils::tracing_init};
use std::fs::read_to_string;
use tracing::{event, Level};

fn main() {
    tracing_init();
    let input = read_to_string("res/input").unwrap();

    let result = solve(&input);
    event!(Level::INFO, "{}", result);
}
