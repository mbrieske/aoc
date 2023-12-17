use grid::Grid;
use pathfinding::prelude::astar;

pub fn solve(input: &str) -> usize {
    let grid = Grid::from_vec(
        input
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>(),
        input.lines().next().unwrap().len(),
    );

    let goal = Pos {
        x: grid.cols() - 1,
        y: grid.rows() - 1,
        last: (Direction::None, 0),
    };

    let result = astar(
        &Pos {
            x: 0,
            y: 0,
            last: (Direction::None, 0),
        },
        |p| {
            p.successors(grid.cols() - 1, grid.rows() - 1)
                .into_iter()
                .map(|other| (other, other.cost(&grid)))
        },
        |p| goal.x - p.x + goal.y - p.y,
        |p| p.x == goal.x && p.y == goal.y && p.last.1 >= 4,
    );

    dbg!(&result);
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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: usize,
    y: usize,
    last: (Direction, usize),
}

impl Pos {
    fn cost(&self, grid: &Grid<usize>) -> usize {
        *grid.get(self.y, self.x).unwrap()
    }

    fn successors(&self, max_x: usize, max_y: usize) -> Vec<Pos> {
        let &Pos {
            x,
            y,
            last: (last_dir, last_n_steps),
        } = self;

        let mut successors = Vec::new();

        if last_n_steps < 4 && last_dir != Direction::None {
            if last_dir == Direction::Right && x < max_x {
                successors.push(Pos {
                    x: x + 1,
                    y,
                    last: (Direction::Right, last_n_steps + 1),
                });
            } else if last_dir == Direction::Left && x > 0 {
                successors.push(Pos {
                    x: x - 1,
                    y,
                    last: (Direction::Left, last_n_steps + 1),
                });
            } else if last_dir == Direction::Up && y > 0 {
                successors.push(Pos {
                    x,
                    y: y - 1,
                    last: (Direction::Up, last_n_steps + 1),
                });
            } else if last_dir == Direction::Down && y < max_y {
                successors.push(Pos {
                    x,
                    y: y + 1,
                    last: (Direction::Down, last_n_steps + 1),
                });
            }
        } else {
            if x < max_x
                && last_dir != Direction::Left
                && !(last_dir == Direction::Right && last_n_steps == 10)
            {
                let x = x + 1;
                let n_steps = if last_dir == Direction::Right {
                    last_n_steps + 1
                } else {
                    1
                };
                successors.push(Pos {
                    x,
                    y,
                    last: (Direction::Right, n_steps),
                });
            }

            if x > 0
                && last_dir != Direction::Right
                && !(last_dir == Direction::Left && last_n_steps == 10)
            {
                let x = x - 1;
                let n_steps = if last_dir == Direction::Left {
                    last_n_steps + 1
                } else {
                    1
                };
                successors.push(Pos {
                    x,
                    y,
                    last: (Direction::Left, n_steps),
                });
            }

            if y < max_y
                && last_dir != Direction::Up
                && !(last_dir == Direction::Down && last_n_steps == 10)
            {
                let y = y + 1;
                let n_steps = if last_dir == Direction::Down {
                    last_n_steps + 1
                } else {
                    1
                };
                successors.push(Pos {
                    x,
                    y,
                    last: (Direction::Down, n_steps),
                });
            }

            if y > 0
                && last_dir != Direction::Down
                && !(last_dir == Direction::Up && last_n_steps == 10)
            {
                let y = y - 1;
                let n_steps = if last_dir == Direction::Up {
                    last_n_steps + 1
                } else {
                    1
                };
                successors.push(Pos {
                    x,
                    y,
                    last: (Direction::Up, n_steps),
                });
            }
        }
        successors
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
