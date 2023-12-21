use std::{collections::HashSet, fmt::Display};

#[allow(unused_imports)]
use itertools::Itertools;
use tracing::{event, instrument, Level};

pub fn solve(input: &str, steps: isize) -> isize {
    let grid: Vec<Vec<Terrain>> = input
        .lines()
        .map(|line| line.chars().map(Terrain::from).collect())
        .collect();
    let start = input
        .chars()
        .filter(|&c| c != '\n')
        .position(|c| c == 'S')
        .unwrap() as isize;
    let grid_size = (
        input.lines().next().unwrap().len() as isize,
        input.lines().count() as isize,
    );
    let start_pos = (start % grid_size.0, (start / grid_size.0));

    let mut reachable: HashSet<(isize, isize)> = HashSet::from([start_pos]);
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let offset = steps % grid_size.0;
    let x = steps / grid_size.0;
    event!(Level::INFO, "Solve for x = {}, quadratic fit values:", x);

    for i in 1..=2 * grid_size.0 + offset {
        reachable = reachable
            .iter()
            .flat_map(|pos| {
                directions
                    .iter()
                    .filter_map(|&dir| walk(&grid, grid_size, pos, dir))
            })
            .collect();
        if i % grid_size.0 == offset && i != 2 * grid_size.0 + offset {
            event!(Level::INFO, "{}", reachable.len());
        }
    }

    reachable.len().try_into().unwrap()
}

fn walk(
    grid: &Vec<Vec<Terrain>>,
    grid_size: (isize, isize),
    position: &(isize, isize),
    dir: (isize, isize),
) -> Option<(isize, isize)> {
    let new_wrapped = wrap_add(*position, dir, grid_size);
    if matches!(
        grid[new_wrapped.1 as usize][new_wrapped.0 as usize],
        Terrain::Garden
    ) {
        Some((position.0 + dir.0, position.1 + dir.1))
    } else {
        None
    }
}
enum Terrain {
    Garden,
    Rock,
}

impl From<char> for Terrain {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Rock,
            '.' | 'S' => Self::Garden,

            _ => unreachable!(),
        }
    }
}

impl Display for Terrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Terrain::Garden => write!(f, "."),
            Terrain::Rock => write!(f, "#"),
        }
    }
}

#[instrument(level = Level::DEBUG)]
fn process_line(_line: &str) -> isize {
    2
}

fn wrap_add(a: (isize, isize), b: (isize, isize), range: (isize, isize)) -> (isize, isize) {
    let mut new = (((a.0 + b.0) % range.0), ((a.1 + b.1) % range.1));

    while new.0 < 0 {
        new.0 += range.0
    }
    while new.1 < 0 {
        new.1 += range.1
    }

    new
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tracing_init;
    use rstest::rstest;

    #[test]
    fn wrapping() {
        assert_eq!(wrap_add((9, 2), (2, 0), (10, 10)), (1, 2));
        assert_eq!(wrap_add((2, 2), (-3, 0), (10, 10)), (9, 2));
    }

    static EXAMPLE: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[rstest]
    #[case(6, 16)]
    #[case(10, 50)]
    #[case(50, 1594)]
    #[case(100, 6536)]
    fn test_example(#[case] steps: isize, #[case] expected: isize) {
        tracing_init();
        assert_eq!(solve(EXAMPLE, steps), expected);
    }
}
