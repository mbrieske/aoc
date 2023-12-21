use std::{collections::HashSet, fmt::Display};

#[allow(unused_imports)]
use itertools::Itertools;
use tracing::{instrument, Level};

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

    for _ in 0..steps {
        reachable = reachable
            .iter()
            .flat_map(|pos| {
                directions
                    .iter()
                    .filter_map(|&dir| walk(&grid, grid_size, pos, dir))
            })
            .collect();
    }
    reachable.len().try_into().unwrap()
}

fn walk(
    grid: &Vec<Vec<Terrain>>,
    grid_size: (isize, isize),
    position: &(isize, isize),
    dir: (isize, isize),
) -> Option<(isize, isize)> {
    let new = (position.0 + dir.0, position.1 + dir.1);
    if new.0 >= 0
        && new.0 < grid_size.0
        && new.1 >= 0
        && new.1 < grid_size.0
        && matches!(grid[new.1 as usize][new.0 as usize], Terrain::Garden)
    {
        Some(new)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tracing_init;
    // use rstest::rstest;

    #[test]
    fn example() {
        tracing_init();
        let example = "...........
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
        assert_eq!(solve(example, 6), 16);
    }

    // #[rstest]
    // #[case("asd", 2)]
    // #[case("fgh", 2)]
    // fn test_process_line(#[case] input: &str, #[case] expected: isize) {
    //     tracing_init();
    //     assert_eq!(solve(input), expected);
    // }
}
