use std::{
    cmp::{max, min},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("res/input").unwrap();
    println!("{}", puzzle(input, 1_000_000));
}

fn puzzle(input: String, by: usize) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let empty_vertical_galaxies = get_empty_galaxies_vertically(&map);
    let transposed = transpose(&map);
    let empty_horizonal_galaxies = get_empty_galaxies_vertically(&transposed);

    let galaxy_positions = &map
        .iter()
        .flatten()
        .enumerate()
        .filter(|(_, &c)| c == '#')
        .map(|(i, _)| (i % map[0].len(), i / map[0].len()))
        .collect::<Vec<(usize, usize)>>();

    galaxy_positions
        .iter()
        .enumerate()
        .flat_map(|(i, pos_1)| {
            galaxy_positions[i + 1..].iter().map(|pos_2| {
                distance(
                    pos_1,
                    pos_2,
                    &empty_vertical_galaxies,
                    &empty_horizonal_galaxies,
                    by,
                )
            })
        })
        .sum()
}

fn distance(
    pos_1: &(usize, usize),
    pos_2: &(usize, usize),
    empty_vertical_galaxies: &[usize],
    empty_horizontal_galaxies: &[usize],
    by: usize,
) -> usize {
    let from_x = min(pos_1.0, pos_2.0);
    let from_y = min(pos_1.1, pos_2.1);
    let to_x = max(pos_1.0, pos_2.0);
    let to_y = max(pos_1.1, pos_2.1);

    to_x - from_x + to_y - from_y
        + count_in_range(empty_horizontal_galaxies, (from_x, to_x)) * (by - 1)
        + count_in_range(empty_vertical_galaxies, (from_y, to_y)) * (by - 1)
}

fn count_in_range(sorted_vec: &[usize], range: (usize, usize)) -> usize {
    let start_index = match sorted_vec.binary_search(&range.0) {
        Ok(index) => index,
        Err(index) => index,
    };

    let end_index = match sorted_vec.binary_search(&range.1) {
        Ok(index) => index + 1, // include the matching element
        Err(index) => index,
    };

    if start_index >= end_index {
        0
    } else {
        end_index - start_index
    }
}

fn get_empty_galaxies_vertically(map: &[Vec<char>]) -> Vec<usize> {
    map.iter()
        .enumerate()
        .filter(|(_, symbols)| !symbols.contains(&'#'))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>()
}

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_example_by_2() {
        let example = String::from(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );

        assert_eq!(puzzle(example, 2), 374);
    }

    #[test]
    fn expand_example_by_10() {
        let example = String::from(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );

        assert_eq!(puzzle(example, 10), 1030);
    }

    #[test]
    fn expand_example_by_100() {
        let example = String::from(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );

        assert_eq!(puzzle(example, 100), 8410);
    }
}
