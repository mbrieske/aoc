use std::fs::read_to_string;

fn main() {
    let input = read_to_string("res/input").unwrap();
    println!("{}", puzzle(input));
}

fn puzzle(input: String) -> usize {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    map = transpose(map);
    map = expand_vertically(map);
    map = transpose(map);
    map = expand_vertically(map);

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
        .map(|(i, pos_1)| {
            galaxy_positions[i + 1..]
                .iter()
                .map(|pos_2| pos_1.0.abs_diff(pos_2.0) + pos_1.1.abs_diff(pos_2.1))
        })
        .flatten()
        .sum()
}

fn expand_vertically(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let empty_galaxies = map
        .iter()
        .enumerate()
        .filter(|(_, symbols)| !symbols.contains(&'#'))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    empty_galaxies
        .into_iter()
        .enumerate()
        .for_each(|(i, original_i)| {
            map.insert(i + original_i, map.get(original_i + i).unwrap().clone());
        });
    map
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
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
    fn example() {
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

        assert_eq!(puzzle(example), 374);
    }
}
