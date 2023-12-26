use grid::Grid;
use pathfinding::matrix::directions::DIRECTIONS_4;
use std::collections::{HashMap, HashSet};

type Pos = (isize, isize);
type Dir = (isize, isize);
type Trail = bool;

fn is_trail(field: char) -> Trail {
    match field {
        '.' | '^' | '>' | 'v' | '<' => true,
        '#' => false,
        _ => unreachable!(),
    }
}
struct Graph {
    vertices: HashSet<Pos>,
    edges: HashMap<Pos, HashSet<(Pos, usize)>>,
}

impl Graph {
    fn find_longest_path(&self, from: Pos, to: Pos) -> usize {
        let mut paths_to_target: Vec<Vec<&((isize, isize), usize)>> = Vec::new();

        // let visited: HashMap<Pos, usize> = HashMap::new();
        let first = self.edges.get(&from).unwrap().iter().next().unwrap();
        let mut paths: Vec<Vec<&((isize, isize), usize)>> = vec![vec![first]];

        while let Some(path) = paths.pop() {
            let next_edges = self.edges.get(&path.last().unwrap().0).unwrap();

            next_edges
                .iter()
                .filter(|next_edge| {
                    !path
                        .iter()
                        .map(|(pos, _)| pos)
                        .collect::<Vec<&(isize, isize)>>()
                        .contains(&&next_edge.0)
                })
                .for_each(|next_edge| {
                    let mut path = path.clone();
                    path.push(next_edge);
                    if next_edge.0 == to {
                        paths_to_target.push(path);
                    } else {
                        paths.push(path)
                    };
                });
        }

        paths_to_target
            .iter()
            .map(|path| path.iter().map(|(_, len)| len).sum())
            .max()
            .unwrap()
    }
}

pub fn solve(input: &str) -> usize {
    let v: Vec<Trail> = input
        .lines()
        .flat_map(|line| line.chars().map(is_trail))
        .collect();
    let grid: Grid<Trail> = Grid::from_vec(v, input.lines().next().unwrap().len());

    let startpos = (1, 0);
    let target = (grid.cols() as isize - 2, grid.rows() as isize - 1);

    let mut g = Graph {
        vertices: HashSet::from([startpos]),
        edges: HashMap::new(),
    };

    let mut queue: Vec<(Pos, Pos)> = vec![((0, 1), startpos)];
    while let Some((dir, pos)) = queue.pop() {
        if let Some((newpos, len, next_dirs)) = find_next(&grid, dir, pos, 0, target) {
            g.edges.entry(pos).or_default().insert((newpos, len));
            g.edges.entry(newpos).or_default().insert((pos, len));
            if g.vertices.insert(newpos) {
                next_dirs.iter().for_each(|&next_dir| {
                    queue.push((next_dir, newpos));
                })
            }
        }
    }

    g.find_longest_path(startpos, target)
}

// get next node + dist when following direction, plus directions going from that node
fn find_next(
    grid: &Grid<Trail>,
    dir: Dir,
    mut pos: Pos,
    len_acc: usize,
    target: Pos,
) -> Option<(Pos, usize, Vec<Dir>)> {
    pos.0 += dir.0;
    pos.1 += dir.1;
    if pos == target {
        return Some((pos, len_acc + 1, Vec::new()));
    }
    let next_dirs: Vec<Dir> = DIRECTIONS_4
        .iter()
        .filter(|next_dir| (next_dir.0 + dir.0, next_dir.1 + dir.1) != (0, 0))
        .map(|&next_dir| (next_dir, (pos.0 + next_dir.0, pos.1 + next_dir.1)))
        .filter_map(|(next_dir, next_pos)| {
            grid.get(next_pos.1, next_pos.0)
                .map(|trail| (next_dir, next_pos, trail))
        })
        .filter(|(_, _, trail)| matches!(trail, true))
        .map(|(next_dir, _, _)| next_dir)
        .collect();
    if next_dirs.is_empty() {
        None
    } else if next_dirs.len() == 1 {
        find_next(grid, next_dirs[0], pos, len_acc + 1, target)
    } else {
        Some((pos, len_acc + 1, next_dirs))
    }
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
    #[case(EXAMPLE, 154)]
    fn test_example(#[case] input: &str, #[case] expected: usize) {
        tracing_init();
        assert_eq!(solve(input), expected);
    }
}
