use std::collections::{HashMap, HashSet};

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

    let mut seen: HashMap<Position, HashSet<Direction>> = HashMap::new();

    luminate(&grid, &mut seen, Direction::Right, Position { x: 0, y: 0 });
    seen.keys().count()
}

fn luminate(
    grid: &Grid<Symbol>,
    seen: &mut HashMap<Position, HashSet<Direction>>,
    direction: Direction,
    position: Position,
) {
    if let Some(&symbol) = grid.get(position.y, position.x) {
        match (direction, symbol) {
            (Direction::Right, Symbol::Dash | Symbol::Dot)
            | (Direction::Up, Symbol::Forwardslash)
            | (Direction::Down, Symbol::Backslash) => {
                if seen.entry(position).or_default().insert(Direction::Right) {
                    luminate(grid, seen, Direction::Right, position.get_right())
                }
            } // Right
            (Direction::Down, Symbol::Pipe | Symbol::Dot)
            | (Direction::Right, Symbol::Backslash)
            | (Direction::Left, Symbol::Forwardslash) => {
                if seen.entry(position).or_default().insert(Direction::Down) {
                    luminate(grid, seen, Direction::Down, position.get_down())
                }
            }
            (Direction::Left, Symbol::Dash | Symbol::Dot)
            | (Direction::Up, Symbol::Backslash)
            | (Direction::Down, Symbol::Forwardslash) => {
                if seen.entry(position).or_default().insert(Direction::Left) {
                    luminate(grid, seen, Direction::Left, position.get_left())
                }
            }
            (Direction::Up, Symbol::Pipe | Symbol::Dot)
            | (Direction::Right, Symbol::Forwardslash)
            | (Direction::Left, Symbol::Backslash) => {
                if seen.entry(position).or_default().insert(Direction::Up) {
                    luminate(grid, seen, Direction::Up, position.get_up())
                }
            }
            (Direction::Right | Direction::Left, Symbol::Pipe) => {
                if seen.entry(position).or_default().insert(Direction::Up) {
                    luminate(grid, seen, Direction::Up, position.get_up())
                }
                if seen.entry(position).or_default().insert(Direction::Down) {
                    luminate(grid, seen, Direction::Down, position.get_down())
                }
            } // Up and Down
            (Direction::Up | Direction::Down, Symbol::Dash) => {
                if seen.entry(position).or_default().insert(Direction::Left) {
                    luminate(grid, seen, Direction::Left, position.get_left())
                }
                if seen.entry(position).or_default().insert(Direction::Right) {
                    luminate(grid, seen, Direction::Right, position.get_right())
                }
            }
        }
    };
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
        assert_eq!(solve(example), 46);
    }

    // #[rstest]
    // #[case("asd", 2)]
    // #[case("fgh", 2)]
    // fn test_process_line(#[case] input: &str, #[case] expected: usize) {
    //     assert_eq!(process_line(input), expected);
    // }
}
