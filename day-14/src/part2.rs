#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use cached::proc_macro::cached;

pub fn solve(input: &str, cycles: usize) -> usize {
    let mut field: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    rotate_minus90(&mut field);

    let mut seen: Vec<Vec<Vec<char>>> = Vec::new();
    for i in 0..cycles {
        field = cycle(field);
        if let Some(first_seen_index) = seen.iter().position(|v| *v == field) {
            let period = i - first_seen_index;
            let target = (cycles - 1 - first_seen_index) % period + first_seen_index;
            field = seen[target].clone();
            break;
        } else {
            seen.push(field.clone())
        };
        dbg!(i);
    }

    rotate_90(&mut field);

    calculate_val(field)
}

fn calculate_val(field: Vec<Vec<char>>) -> usize {
    let field_len = field.len();
    field
        .iter()
        .enumerate()
        .map(|(i, line)| (field_len - i) * line.iter().filter(|&&c| c == 'O').count())
        .sum()
}

fn get_field_str(field: &Vec<Vec<char>>, rotated: bool) -> String {
    let mut printfield = field.clone();
    if rotated {
        rotate_90(&mut printfield);
    };

    printfield
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn roll_field_left(field: &mut Vec<Vec<char>>) {
    field.iter_mut().for_each(roll_stones_left);
}

#[cached]
fn cycle(mut field: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for _ in 0..4 {
        roll_field_left(&mut field);
        rotate_90(&mut field);
    }
    field
}

fn roll_stones_left(line: &mut Vec<char>) {
    *line = line
        .split(|&c| c == '#')
        .map(|substr| {
            let mut substr = Vec::from(substr);
            substr.sort_by(|c1, c2| c2.cmp(c1));
            substr.push('#');
            substr
        })
        .flatten()
        .collect::<Vec<char>>();
    line.pop();
}

fn rotate_90(v: &mut Vec<Vec<char>>) {
    *v = transpose(v);
    v.iter_mut().for_each(|row| row.reverse());
}

fn rotate_minus90(v: &mut Vec<Vec<char>>) {
    v.iter_mut().for_each(|row| row.reverse());
    *v = transpose(v);
}

fn transpose<T>(v: &[Vec<T>]) -> Vec<Vec<T>>
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
    use rstest::rstest;

    #[test]
    fn example() {
        let example = "O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....";
        assert_eq!(solve(example, 20), 64); // 1_000_000_000
    }

    #[test]
    fn example_3c() {
        let example = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(solve(example, 3), 69);
    }

    #[test]
    fn test_cycle() {
        let example = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let expected = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";
        let mut field: Vec<Vec<char>> =
            example.lines().map(|line| line.chars().collect()).collect();
        rotate_minus90(&mut field);
        field = cycle(field);
        assert_eq!(get_field_str(&field, true), expected);
    }

    #[test]
    fn tst_calc_val() {
        let field_str = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";
        let field: Vec<Vec<char>> = field_str
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        assert_eq!(calculate_val(field), 69);
    }

    #[rstest]
    #[case("..OO", "OO..")]
    #[case("...OO.", "OO....")]
    #[case("...OO.#.O.", "OO....#O..")]
    #[case("#...OO.", "#OO....")]
    fn test_process_line(#[case] input: &str, #[case] expected: &str) {
        let mut input = input.chars().collect::<Vec<char>>();
        let expected = expected.chars().collect::<Vec<char>>();
        roll_stones_left(&mut input);
        assert_eq!(input, expected);
    }
}
