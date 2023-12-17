use grid::Grid;
use pathfinding::prelude::astar;

pub fn solve(input: &str) -> isize {
    let grid: Grid<isize> = Grid::from_vec(
        input
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| c.to_digit(10).unwrap() as isize)
            .collect(),
        input.lines().next().unwrap().len(),
    );

    let goal = Pos {
        x: grid.cols() as isize - 1,
        y: grid.rows() as isize - 1,
        last_dir: Direction::None,
        last_n_steps: 0,
    };

    let result = astar(
        &Pos {
            x: 0,
            y: 0,
            last_dir: Direction::None,
            last_n_steps: 0,
        },
        |p| {
            p.successors(grid.cols() as isize - 1, grid.rows() as isize - 1)
                .into_iter()
                .map(|other| (other, other.cost(&grid)))
        },
        |p| goal.x - p.x + goal.y - p.y,
        |p| p.x == goal.x && p.y == goal.y && p.last_n_steps >= 4,
    );

    result.unwrap().1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    None,
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::None => Direction::None,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: isize,
    y: isize,
    last_dir: Direction,
    last_n_steps: usize,
}

impl Pos {
    fn cost(&self, grid: &Grid<isize>) -> isize {
        *grid.get(self.y as usize, self.x as usize).unwrap()
    }

    fn successors(&self, max_x: isize, max_y: isize) -> Vec<Pos> {
        [
            (Direction::Up, 0, -1),
            (Direction::Down, 0, 1),
            (Direction::Left, -1, 0),
            (Direction::Right, 1, 0),
        ]
        .iter()
        .map(|&(direction, dx, dy)| Pos {
            x: self.x + dx,
            y: self.y + dy,
            last_dir: direction,
            last_n_steps: if direction == self.last_dir {
                self.last_n_steps + 1
            } else {
                1
            },
        })
        .filter(|pos| {
            pos.x >= 0
                && pos.x <= max_x
                && pos.y >= 0
                && pos.y <= max_y
                && pos.last_n_steps <= 10
                && pos.last_dir != self.last_dir.opposite()
                && !(self.last_dir != Direction::None
                    && self.last_n_steps < 4
                    && self.last_dir != pos.last_dir)
        })
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let example = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(solve(example), 94);
    }

    #[test]
    fn example_2() {
        let example = "111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(solve(example), 71);
    }
}
