enum Direction {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

impl Direction {
    fn len(&self) -> isize {
        match self {
            Direction::Up(len) => *len,
            Direction::Down(len) => *len,
            Direction::Right(len) => *len,
            Direction::Left(len) => *len,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Vertice {
    x: isize,
    y: isize,
}

impl From<&str> for Direction {
    fn from(input: &str) -> Self {
        let relevant = input.split_whitespace().nth(2).unwrap();
        let dist = isize::from_str_radix(&relevant[2..7], 16).unwrap();
        match relevant.chars().nth(7).unwrap() {
            '0' => Direction::Right(dist),
            '1' => Direction::Down(dist),
            '2' => Direction::Left(dist),
            '3' => Direction::Up(dist),
            _ => unreachable!(),
        }
    }
}

pub fn solve(input: &str) -> isize {
    let directions: Vec<Direction> = input.lines().map(Direction::from).collect();

    let vertices = directions
        .iter()
        .fold(vec![Vertice { x: 0, y: 0 }], |mut acc, direction| {
            use Direction::*;
            match direction {
                Up(dist) => acc.push(Vertice {
                    x: acc.last().unwrap().x,
                    y: acc.last().unwrap().y - dist,
                }),
                Down(dist) => acc.push(Vertice {
                    x: acc.last().unwrap().x,
                    y: acc.last().unwrap().y + dist,
                }),
                Left(dist) => acc.push(Vertice {
                    x: acc.last().unwrap().x - dist,
                    y: acc.last().unwrap().y,
                }),
                Right(dist) => acc.push(Vertice {
                    x: acc.last().unwrap().x + dist,
                    y: acc.last().unwrap().y,
                }),
            }
            acc
        });

    let a = vertices.windows(2).fold(0, |acc, vertices| {
        acc + vertices[0].x * vertices[1].y - vertices[1].x * vertices[0].y
    }) / 2;

    let perimeter = directions.iter().fold(0, |acc, dir| acc + dir.len()) / 2 + 1;
    a + perimeter
}

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
        assert_eq!(solve(example), 952408144115);
    }
}
