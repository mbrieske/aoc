use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

use grid::Grid;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
enum Symbol {
    Dot,
    Dash,
    Pipe,
    Forwardslash,
    Backslash,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn get_up(&self) -> Self {
        Position {
            x: self.x,
            y: self.y.wrapping_sub(1),
        }
    }

    fn get_down(&self) -> Self {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn get_left(&self) -> Self {
        Position {
            x: self.x.wrapping_sub(1),
            y: self.y,
        }
    }

    fn get_right(&self) -> Self {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }
}

impl From<char> for Symbol {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Self::Dot,
            '-' => Self::Dash,
            '|' => Self::Pipe,
            '/' => Self::Forwardslash,
            '\\' => Self::Backslash,
            _ => unreachable!(),
        }
    }
}

#[must_use]
pub fn solve(input: &str) -> usize {
    let grid = Grid::from_vec(
        input
            .chars()
            .filter(|&c| c != '\n')
            .map(Symbol::from)
            .collect::<Vec<Symbol>>(),
        input.lines().next().unwrap().len(),
    );

    let cols_max = (0..grid.cols())
        .map(|col| {
            max(
                luminate(
                    &grid,
                    &mut HashMap::new(),
                    Direction::Down,
                    Position { x: col, y: 0 },
                ),
                luminate(
                    &grid,
                    &mut HashMap::new(),
                    Direction::Up,
                    Position {
                        x: col,
                        y: grid.rows() - 1,
                    },
                ),
            )
        })
        .max()
        .unwrap();

    let rows_max = (0..grid.rows())
        .map(|row| {
            max(
                luminate(
                    &grid,
                    &mut HashMap::new(),
                    Direction::Right,
                    Position { x: 0, y: row },
                ),
                luminate(
                    &grid,
                    &mut HashMap::new(),
                    Direction::Left,
                    Position {
                        x: grid.cols() - 1,
                        y: row,
                    },
                ),
            )
        })
        .max()
        .unwrap();
    max(cols_max, rows_max)
}

fn luminate(
    grid: &Grid<Symbol>,
    seen: &mut HashMap<Position, HashSet<Direction>>,
    direction: Direction,
    position: Position,
) -> usize {
    let is_start = seen.is_empty();
    if let Some(&symbol) = grid.get(position.y, position.x) {
        match (direction, symbol) {
            (Direction::Right, Symbol::Dash | Symbol::Dot)
            | (Direction::Up, Symbol::Forwardslash)
            | (Direction::Down, Symbol::Backslash) => {
                if seen.entry(position).or_default().insert(Direction::Right) {
                    luminate(grid, seen, Direction::Right, position.get_right());
                }
            } // Right
            (Direction::Down, Symbol::Pipe | Symbol::Dot)
            | (Direction::Right, Symbol::Backslash)
            | (Direction::Left, Symbol::Forwardslash) => {
                if seen.entry(position).or_default().insert(Direction::Down) {
                    luminate(grid, seen, Direction::Down, position.get_down());
                }
            }
            (Direction::Left, Symbol::Dash | Symbol::Dot)
            | (Direction::Up, Symbol::Backslash)
            | (Direction::Down, Symbol::Forwardslash) => {
                if seen.entry(position).or_default().insert(Direction::Left) {
                    luminate(grid, seen, Direction::Left, position.get_left());
                }
            }
            (Direction::Up, Symbol::Pipe | Symbol::Dot)
            | (Direction::Right, Symbol::Forwardslash)
            | (Direction::Left, Symbol::Backslash) => {
                if seen.entry(position).or_default().insert(Direction::Up) {
                    luminate(grid, seen, Direction::Up, position.get_up());
                }
            }
            (Direction::Right | Direction::Left, Symbol::Pipe) => {
                if seen.entry(position).or_default().insert(Direction::Up) {
                    luminate(grid, seen, Direction::Up, position.get_up());
                }
                if seen.entry(position).or_default().insert(Direction::Down) {
                    luminate(grid, seen, Direction::Down, position.get_down());
                }
            } // Up and Down
            (Direction::Up | Direction::Down, Symbol::Dash) => {
                if seen.entry(position).or_default().insert(Direction::Left) {
                    luminate(grid, seen, Direction::Left, position.get_left());
                }
                if seen.entry(position).or_default().insert(Direction::Right) {
                    luminate(grid, seen, Direction::Right, position.get_right());
                }
            }
        }
    };
    if is_start {
        seen.keys().count()
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use rstest::rstest;

    #[test]
    fn example() {
        let example = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
        assert_eq!(solve(example), 51);
    }
}
