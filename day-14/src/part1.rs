#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

pub fn solve(input: &str) -> usize {
    let mut field: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let field_len = field.len();
    field = transpose(&field);
    field.iter_mut().for_each(process_line);
    field = transpose(&field);
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

fn process_line(line: &mut Vec<char>) {
    *line = line
        .split(|&c| c == '#')
        .map(|substr| {
            let mut substr = Vec::from(substr);
            substr.sort();
            substr.reverse();
            substr.push('#');
            substr
        })
        .flatten()
        .collect::<Vec<char>>();
    line.pop();
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
        assert_eq!(solve(example), 136);
    }

    #[rstest]
    #[case("..OO", "OO..")]
    #[case("...OO.", "OO....")]
    #[case("...OO.#.O.", "OO....#O..")]
    #[case("#...OO.", "#OO....")]
    fn test_process_line(#[case] input: &str, #[case] expected: &str) {
        let mut input = input.chars().collect::<Vec<char>>();
        let expected = expected.chars().collect::<Vec<char>>();
        process_line(&mut input);
        assert_eq!(input, expected);
    }
}
