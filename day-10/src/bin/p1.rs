use std::fs::read_to_string;

#[derive(Debug)]
enum AocError {
    OutOfBounds,
    NotConnected,
    S,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    S,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

struct Map<'a> {
    field: Vec<&'a str>,
}

impl<'a> Map<'a> {
    fn size(&self) -> Position {
        Position {
            x: self.field[0].len(),
            y: self.field.len(),
        }
    }

    fn get(&self, pos: &Position) -> Result<char, AocError> {
        if let Some(line) = self.field.get(pos.y) {
            if let Some(c) = line.as_bytes().get(pos.x) {
                Ok(*c as char)
            } else {
                Err(AocError::OutOfBounds)
            }
        } else {
            Err(AocError::OutOfBounds)
        }
    }

    fn go(
        &self,
        from: &Position,
        direction: &Direction,
    ) -> Result<(Position, Direction), AocError> {
        use Direction::*;
        let next_pos = match direction {
            Up => Position {
                x: from.x,
                y: from.y.wrapping_sub(1),
            },
            Down => Position {
                x: from.x,
                y: from.y + 1,
            },
            Left => Position {
                x: from.x.wrapping_sub(1),
                y: from.y,
            },
            Right => Position {
                x: from.x + 1,
                y: from.y,
            },
            S => return Err(AocError::S),
        };

        let next_dir = match (direction, self.get(&next_pos)?) {
            (_, 'S') => S,

            (Up, '7') => Left,
            (Up, '|') => Up,
            (Up, 'F') => Right,

            (Right, 'J') => Up,
            (Right, '-') => Right,
            (Right, '7') => Down,

            (Down, 'L') => Right,
            (Down, '|') => Down,
            (Down, 'J') => Left,

            (Left, 'F') => Down,
            (Left, '-') => Left,
            (Left, 'L') => Up,

            _ => return Err(AocError::NotConnected),
        };

        Ok((next_pos, next_dir))
    }
}

fn main() {
    let input = read_to_string("res/input").unwrap();
    println!("{}", puzzle(input));
}

fn puzzle(input: String) -> usize {
    let map = Map {
        field: input.lines().collect::<Vec<&str>>(),
    };
    let start_index = input.find('S').unwrap();

    let start_pos = Position {
        x: start_index % (map.size().x + 1),
        y: start_index / (map.size().y + 1),
    };

    [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .iter()
    .map(|start_dir| {
        let mut pos = start_pos;
        let mut dir = *start_dir;

        let mut steps = 0;

        while dir != Direction::S {
            steps += 1;
            (pos, dir) = map.go(&pos, &dir)?;
        }

        Ok::<usize, AocError>(steps / 2)
    })
    .filter_map(Result::ok)
    .max()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = String::from(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );

        assert_eq!(puzzle(example), 4);
    }

    #[test]
    fn example_complex() {
        let example = String::from(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );

        assert_eq!(puzzle(example), 8);
    }

    #[test]
    fn goto_test() {
        let field = ".....
.S-7.
.|.|.
.L-J.
....."
            .lines()
            .collect::<Vec<&str>>();
        let map = Map { field };

        assert_eq!(
            map.go(&Position { x: 1, y: 1 }, &Direction::Right).unwrap(),
            (Position { x: 2, y: 1 }, Direction::Right)
        );

        assert_eq!(
            map.go(&Position { x: 0, y: 1 }, &Direction::Right).unwrap(),
            (Position { x: 1, y: 1 }, Direction::S)
        );
    }
}
