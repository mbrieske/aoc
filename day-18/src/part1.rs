use grid::Grid;
use std::{
    cmp::{max, min},
    fmt::Display,
};

#[allow(dead_code)]
#[derive(Clone)]
enum Terrain {
    Trench,
    Ground,
}

impl Display for Terrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Terrain::Trench => write!(f, "#"),
            Terrain::Ground => write!(f, "."),
        }
    }
}
enum Direction {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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

pub fn solve(input: &str) -> usize {
    let directions: Vec<Direction> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let (dir_char, len) = (
                parts.next().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            );
            Direction::from((dir_char, len))
        })
        .collect();

    let (_, x_min, x_max, y_min, y_max) = directions.iter().fold(
        (Position { x: 0, y: 0 }, 0, 0, 0, 0),
        |(mut pos, x_min, x_max, y_min, y_max), direction| {
            match direction {
                Direction::Left(dist) => pos.x -= dist,
                Direction::Right(dist) => pos.x += dist,
                Direction::Up(dist) => pos.y -= dist,
                Direction::Down(dist) => pos.y += dist,
            };
            (
                pos,
                min(pos.x, x_min),
                max(pos.x, x_max),
                min(pos.y, y_min),
                max(pos.y, y_max),
            )
        },
    );

    let mut grid: Grid<Terrain> = Grid::from_vec(
        vec![Terrain::Ground; ((x_max + 1 - x_min) * (y_max + 1 - y_min)) as usize],
        (x_max + 1 - x_min) as usize,
    );

    let mut position = Position {
        x: -x_min,
        y: -y_min,
    };
    directions
        .into_iter()
        .for_each(|dir| mark_perimeter(&mut grid, &mut position, dir));

    let start_pos = find_inside(&grid);
    flood(&mut grid, start_pos);
    grid.iter()
        .filter(|&t| matches!(t, Terrain::Trench))
        .count()
}

fn find_inside(grid: &Grid<Terrain>) -> Position {
    for y in 0..grid.rows() {
        let mut is_perimeter = false;
        for (x, t) in grid.iter_row(y).enumerate() {
            match (is_perimeter, t) {
                (false, Terrain::Trench) => is_perimeter = true,
                (false, Terrain::Ground) => (),
                (true, Terrain::Trench) => break,
                (true, Terrain::Ground) => {
                    return Position {
                        x: x as isize,
                        y: y as isize,
                    }
                }
            }
        }
    }
    unreachable!()
}

fn mark_perimeter(grid: &mut Grid<Terrain>, pos: &mut Position, dir: Direction) {
    match dir {
        Direction::Up(len) => {
            for i in 1..=len {
                *grid.get_mut((pos.y - i) as usize, pos.x as usize).unwrap() = Terrain::Trench;
            }
            pos.y -= len;
        }
        Direction::Down(len) => {
            for i in 1..=len {
                *grid.get_mut((pos.y + i) as usize, pos.x as usize).unwrap() = Terrain::Trench;
            }
            pos.y += len;
        }
        Direction::Left(len) => {
            for i in 1..=len {
                *grid.get_mut(pos.y as usize, (pos.x - i) as usize).unwrap() = Terrain::Trench;
            }
            pos.x -= len;
        }
        Direction::Right(len) => {
            for i in 1..=len {
                *grid.get_mut(pos.y as usize, (pos.x + i) as usize).unwrap() = Terrain::Trench;
            }
            pos.x += len;
        }
    }
}

fn flood(grid: &mut Grid<Terrain>, start_pos: Position) {
    let mut queue = Vec::new();
    queue.push(start_pos);

    while let Some(pos) = queue.pop() {
        let (col, row) = (pos.x as usize, pos.y as usize);

        if let Some(Terrain::Ground) = grid.get(row, col) {
            *grid.get_mut(row, col).unwrap() = Terrain::Trench;
            for &(dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_pos = Position {
                    x: pos.x + dx,
                    y: pos.y + dy,
                };
                queue.push(new_pos);
            }
        }
    }
}

// fn print_grid(grid: &Grid<Terrain>) {
//     for row in grid.iter_rows() {
//         for t in row {
//             print!("{}", t);
//         }
//         println!();
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = "D 5 (#0dc571)
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
U 2 (#7a21e3)
R 6 (#70c710)";
        assert_eq!(solve(example), 62);
    }
}
