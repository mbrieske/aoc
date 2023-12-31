#[derive(PartialEq, Eq)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

pub fn solve(input: &str) -> usize {
    input.split("\n\n").map(process_field).sum()
}

fn process_field(field: &str) -> usize {
    let mut field: Vec<Vec<char>> = field.lines().map(|line| line.chars().collect()).collect();

    let original_reflection = find_reflection(&field, None).unwrap();

    for y in 0..field.len() {
        for x in 0..field[0].len() {
            swap_field(&mut field, x, y);

            if let Some(reflection) = find_reflection(&field, Some(&original_reflection)) {
                if reflection != original_reflection {
                    return match reflection {
                        Reflection::Horizontal(reflection) => reflection,
                        Reflection::Vertical(reflection) => reflection * 100,
                    };
                }
                swap_field(&mut field, x, y);
            } else {
                swap_field(&mut field, x, y);
            }
        }
    }
    unreachable!();
}

fn swap_field(field: &mut [Vec<char>], x: usize, y: usize) {
    if field[y][x] == '#' {
        field[y][x] = '.';
    } else {
        field[y][x] = '#';
    }
}

fn find_reflection(field: &Vec<Vec<char>>, ignore: Option<&Reflection>) -> Option<Reflection> {
    if let Some(Reflection::Vertical(ignore_line)) = ignore {
        find_reflection_vertical(field, Some(*ignore_line))
    } else {
        find_reflection_vertical(field, None)
    }
    .map(Reflection::Vertical)
    .or_else(|| {
        if let Some(Reflection::Horizontal(ignore_line)) = ignore {
            find_reflection_vertical(&transpose(field), Some(*ignore_line))
        } else {
            find_reflection_vertical(&transpose(field), None)
        }
        .map(Reflection::Horizontal)
    })
}

fn find_reflection_vertical(field: &Vec<Vec<char>>, ignore: Option<usize>) -> Option<usize> {
    let size = field.len();
    (1..size).find_map(|i| {
        if let Some(ignore) = ignore {
            if ignore == i {
                return None;
            }
        }
        let (mut l, mut h) = (i - 1, i);
        loop {
            if field[l] != field[h] {
                return None;
            }
            if l == 0 || h == size - 1 {
                return Some(i);
            }
            l -= 1;
            h += 1;
        }
    })
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
        let example = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(solve(example), 400);
    }

    #[rstest]
    #[case(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        300
    )]
    #[case(
        "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        100
    )]
    #[case(
        ".#.##.#.#..#.
....#....#..#
.#..####..##.
..#.#########ccar
....#.....##.
.#####.#..##.
#..#...###..#
###..#.###..#
#.#..#.##....
...#.....####
..#...#......
..#...#.##..#
..#.##..#####
..#.##..#####
..#...#.##..#",
        11
    )]
    fn test_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process_field(input), expected);
    }
}
