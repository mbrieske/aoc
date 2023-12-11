use std::fmt;
use std::fs::read_to_string;

#[derive(Debug)]
enum AocError {
    OutOfBoundsError,
    NotConnectedError,
    SError,
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
    marked: Vec<Vec<char>>,
}

impl<'a> fmt::Debug for Map<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for row in &self.marked {
            for &ch in row {
                write!(f, "{} ", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<'a> Map<'a> {
    fn new(field: Vec<&'a str>) -> Self {
        let marked = vec![vec!['.'; field[0].len()]; field.len()];
        Self { field, marked }
    }

    fn size(&self) -> Position {
        Position {
            x: self.field[0].len(),
            y: self.field.len(),
        }
    }

    fn mark(&mut self, pos: &Position, symbol: char) {
        let symbol = match symbol {
            'F' => '┌',
            'J' => '┘',
            'L' => '└',
            '7' => '┐',
            '-' => '─',
            '|' => '│',
            'S' => return,
            _ => symbol,
        };
        self.marked[pos.y][pos.x] = symbol;
    }

    fn get(&self, pos: &Position) -> Result<char, AocError> {
        if let Some(line) = self.field.get(pos.y) {
            if let Some(c) = line.as_bytes().get(pos.x) {
                Ok(*c as char)
            } else {
                Err(AocError::OutOfBoundsError)
            }
        } else {
            Err(AocError::OutOfBoundsError)
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
            S => return Err(AocError::SError),
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

            _ => return Err(AocError::NotConnectedError),
        };

        Ok((next_pos, next_dir))
    }

    fn calc_inner_outer(&mut self) -> usize {
        self.marked.iter_mut().for_each(|line| {
            let mut inner = false;
            let mut last_half: Option<char> = None;

            line.iter_mut().for_each(|c| {
                *c = match c {
                    '.' => {
                        if inner {
                            'x'
                        } else {
                            '.'
                        }
                    }
                    '│' => {
                        inner = !inner;
                        *c
                    }
                    '─' => *c,
                    '┌' => {
                        last_half = Some(*c);
                        *c
                    }
                    '└' => {
                        last_half = Some(*c);
                        *c
                    }
                    '┘' => {
                        if let Some(last) = last_half {
                            match last {
                                '└' => {
                                    last_half = None;
                                    *c
                                }
                                '┌' => {
                                    inner = !inner;
                                    last_half = None;
                                    *c
                                }
                                _ => unreachable!(),
                            }
                        } else {
                            unreachable!()
                        }
                    }
                    '┐' => {
                        if let Some(last) = last_half {
                            match last {
                                '┌' => {
                                    last_half = None;
                                    *c
                                }
                                '└' => {
                                    inner = !inner;
                                    last_half = None;
                                    *c
                                }
                                _ => unreachable!(),
                            }
                        } else {
                            unreachable!()
                        }
                    }

                    _ => unreachable!(),
                };
            });
        });
        self.marked
            .iter()
            .flatten()
            .filter(|&symbol| symbol == &'x')
            .count()
    }
}

fn main() {
    let input = read_to_string("res/input").unwrap();
    println!("{}", puzzle(input, Direction::Down, 'F'));
}

fn puzzle(input: String, start_dir: Direction, start_char: char) -> usize {
    let mut map = Map::new(input.lines().collect::<Vec<&str>>());
    let start_index = input.find('S').unwrap();

    let start_pos = Position {
        x: start_index % (map.size().x + 1),
        y: start_index / (map.size().y + 1),
    };
    map.mark(&start_pos, start_char);

    let mut pos = start_pos;
    let mut dir = start_dir; // Known from P1

    while dir != Direction::S {
        (pos, dir) = map.go(&pos, &dir).unwrap();
        map.mark(&pos, map.get(&pos).unwrap());
    }

    map.calc_inner_outer()
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

        assert_eq!(puzzle(example, Direction::Right, '┌'), 1);
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

        assert_eq!(puzzle(example, Direction::Right, '┌'), 1);
    }

    #[test]
    fn example_complex_p2_1() {
        let example = String::from(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        );

        assert_eq!(puzzle(example, Direction::Right, '┌'), 4);
    }

    #[test]
    fn example_complex_p2_2() {
        let example = String::from(
            "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........",
        );

        assert_eq!(puzzle(example, Direction::Right, '┌'), 4);
    }

    #[test]
    fn example_complex_p2_bigger() {
        let example = String::from(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        );

        assert_eq!(puzzle(example, Direction::Down, '┐'), 10);
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
        let map = Map::new(field);

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
