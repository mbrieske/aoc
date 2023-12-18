use std::fmt::Display;

use grid::Grid;

#[derive(Clone)]
enum Terrain {
    Trench { color: Vec<u8> },
    Ground,
}

impl Display for Terrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Terrain::Trench { color } => write!(f, "{}", '#'),
            Terrain::Ground => write!(f, "{}", '.'),
        }
    }
}

enum Direction {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

struct Position {
    x: isize,
    y: isize,
}

impl From<(&str, isize)> for Direction {
    fn from((dir_char, len): (&str, isize)) -> Self {
        match dir_char {
            "U" => Self::Up(len),
            "R" => Self::Right(len),
            "D" => Self::Down(len),
            "L" => Self::Left(len),
            _ => unreachable!(),
        }
    }
}

impl Default for Terrain {
    fn default() -> Self {
        Self::Ground
    }
}

pub fn solve(input: &str) -> isize {
    let mut grid: Grid<Terrain> = Grid::from_vec(vec![Terrain::Trench { color: vec![0; 3] }; 1], 1);
    let mut position = Position { x: 0, y: 0 };

    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        let (dir_char, len, color) = (
            parts.next().unwrap(),
            parts.next().unwrap().parse().unwrap(),
            hex::decode(&parts.next().unwrap()[2..8]).unwrap(),
        );
        let dir = Direction::from((dir_char, len));
        extend_grid(&mut grid, &mut position, dir, color)
    });
    print_grid(&grid);
    todo!()
}

fn extend_grid(grid: &mut Grid<Terrain>, pos: &mut Position, dir: Direction, color: Vec<u8>) {
    match dir {
        Direction::Up(len) => {
            pos.y -= len;
            while pos.y < 0 {
                grid.insert_row(0, vec![Terrain::default(); grid.cols()]);
                pos.y += 1;
            }
            for i in 0..len {
                *grid.get_mut((pos.y + i) as usize, pos.x as usize).unwrap() = Terrain::Trench {
                    color: color.clone(),
                };
            }
        }
        Direction::Down(len) => {
            while (pos.y + len) as usize >= grid.rows() {
                grid.push_row(vec![Terrain::default(); grid.cols()]);
            }
            for i in 1..=len {
                *grid.get_mut((pos.y + i) as usize, pos.x as usize).unwrap() = Terrain::Trench {
                    color: color.clone(),
                };
            }
            pos.y += len;
        }
        Direction::Left(len) => {
            pos.x -= len;
            while pos.x < 0 {
                grid.insert_col(0, vec![Terrain::default(); grid.rows()]);
                pos.x += 1;
            }
            for i in 0..len {
                *grid.get_mut(pos.y as usize, (pos.x + i) as usize).unwrap() = Terrain::Trench {
                    color: color.clone(),
                };
            }
        }
        Direction::Right(len) => {
            while (pos.x + len) as usize >= grid.cols() {
                grid.push_col(vec![Terrain::default(); grid.rows()]);
            }
            for i in 1..=len {
                *grid.get_mut(pos.y as usize, (pos.x + i) as usize).unwrap() = Terrain::Trench {
                    color: color.clone(),
                };
            }
            pos.x += len;
        }
    }
}

fn print_grid(grid: &Grid<Terrain>) {
    for row in grid.iter_rows() {
        for t in row {
            print!("{}", t);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use rstest::rstest;

    #[test]
    fn example() {
        let example = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(solve(example), 62);
    }

    // #[rstest]
    // #[case("asd", 2)]
    // #[case("fgh", 2)]
    // fn test_process_line(#[case] input: &str, #[case] expected: isize) {
    //     assert_eq!(process_line(input), expected);
    // }
}
