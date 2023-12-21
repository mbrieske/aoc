use crate::utils::*;
#[allow(unused_imports)]
use itertools::Itertools;
use tracing::{instrument, Level};

static DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn solve(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let grid_size = (grid.get(0).unwrap().len(), grid.len());

    // progressbar_init(total_iterations);

    input
        .lines()
        // .progress_with(progress_bar)
        .map(process_line)
        .sum()
}

#[instrument (level = Level::DEBUG)]
fn process_line(line: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    static EXAMPLE: &str = "";

    #[rstest]
    #[case(EXAMPLE, 0)]
    fn test_example(#[case] input: &str, #[case] expected: usize) {
        tracing_init();
        assert_eq!(solve(input), expected);
    }
}
