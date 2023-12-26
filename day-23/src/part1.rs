use grid::Grid;
use pathfinding::matrix::directions::DIRECTIONS_4;
use tracing::{event, instrument, Level};

type Pos = (isize, isize);

#[derive(Debug)]
enum Trail {
    Path,
    Slope((isize, isize)),
    Forest,
}

impl TryFrom<char> for Trail {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Path),
            '^' => Ok(Self::Slope((0, -1))),
            '>' => Ok(Self::Slope((1, 0))),
            'v' => Ok(Self::Slope((0, 1))),
            '<' => Ok(Self::Slope((-1, 0))),
            '#' => Ok(Self::Forest),
            _ => Err(()),
        }
    }
}

impl Trail {
    fn can_go(&self, dir: Pos) -> bool {
        match self {
            Trail::Path => true,
            Trail::Slope(slope) => (dir.0 + slope.0, dir.1 + slope.1) != (0, 0),
            Trail::Forest => false,
        }
    }
}

pub fn solve(input: &str) -> usize {
    let v: Vec<(Trail, Option<usize>)> = input
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(Trail::try_from)
                .filter_map(Result::ok)
                .map(|trail| (trail, None))
        })
        .collect();
    let mut grid: Grid<(Trail, Option<usize>)> =
        Grid::from_vec(v, input.lines().next().unwrap().len());

    let startpos = (1, 0);
    let target = (grid.cols() as isize - 2, grid.rows() as isize - 1);

    let mut paths_to_target: Vec<Vec<Pos>> = Vec::new();

    // let paths: VecDeque<(Pos, HashSet<Pos>)> = VecDeque::from(vec![(startpos, HashSet::new())]);
    let mut paths: Vec<(Pos, Vec<Pos>)> = vec![(startpos, Vec::from([startpos]))];
    while let Some(path) = paths.pop() {
        if path.0 == target {
            event!(Level::INFO, "{}", path.1.len() - 1);
            paths_to_target.push(path.1);
        } else {
            let newpaths = advance(path.0, &mut grid, path.1);
            let newpositions: Vec<Pos> = newpaths.iter().map(|(pos, _)| *pos).collect();
            paths.retain(|(pos, _)| !newpositions.contains(pos));
            paths.extend(newpaths)
        }
    }

    // progressbar_init(total_iterations);
    paths_to_target
        .iter()
        .max_by(|path, other| path.len().cmp(&other.len()))
        .unwrap()
        .len()
        - 1
}

#[instrument (level = Level::DEBUG, skip(grid))]
fn advance(
    pos: Pos,
    grid: &mut Grid<(Trail, Option<usize>)>,
    prev: Vec<Pos>,
) -> Vec<(Pos, Vec<Pos>)> {
    let can_go = match grid.get(pos.1, pos.0).unwrap() {
        (Trail::Path, _) => DIRECTIONS_4
            .iter()
            .map(|dir| (dir, (pos.0 + dir.0, pos.1 + dir.1)))
            .filter(|(_, newpos)| !prev.contains(newpos))
            .filter_map(|(&dir, newpos)| {
                grid.get(newpos.1, newpos.0)
                    .map(|trail| (dir, newpos, trail))
            })
            .filter(|&(dir, _, (trail, longest))| {
                if let Some(longest) = longest {
                    if longest > &prev.len() {
                        false
                    } else {
                        trail.can_go(dir)
                    }
                } else {
                    trail.can_go(dir)
                }
            })
            .map(|(_, newpos, _)| newpos)
            .collect(),
        (Trail::Slope(slope_dir), _) => vec![(pos.0 + slope_dir.0, pos.1 + slope_dir.1)],
        (Trail::Forest, _) => unreachable!(),
    };
    can_go
        .iter()
        .map(|&pos| {
            let mut prev = prev.clone();
            prev.push(pos);
            grid.get_mut(pos.1, pos.0).unwrap().1 = Some(prev.len());
            (pos, prev)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tracing_init;
    use rstest::rstest;

    static EXAMPLE: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[rstest]
    #[case(EXAMPLE, 94)]
    fn test_example(#[case] input: &str, #[case] expected: usize) {
        tracing_init();
        assert_eq!(solve(input), expected);
    }
}
