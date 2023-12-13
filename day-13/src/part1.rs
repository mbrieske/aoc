pub fn solve(input: &str) -> usize {
    input.split("\n\n").map(process_field).sum()
}

fn process_field(field: &str) -> usize {
    let mut field: Vec<Vec<char>> = field.lines().map(|line| line.chars().collect()).collect();

    let reflection_y = find_reflection_vertical(&field);

    if let Some(reflection_y) = reflection_y {
        reflection_y * 100
    } else if let Some(reflection_x) = {
        field = transpose(&field);
        find_reflection_vertical(&field)
    } {
        reflection_x
    } else {
        unreachable!()
    }
}

fn find_reflection_vertical(field: &Vec<Vec<char>>) -> Option<usize> {
    let size = field.len();
    (1..size).find(|i| {
        let (mut l, mut h) = (i - 1, *i);
        loop {
            if field[l] != field[h] {
                return false;
            }
            if l == 0 || h == size - 1 {
                return true;
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
        assert_eq!(solve(example), 405);
    }

    #[test]
    fn test_vertical() {
        let field = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(process_field(field), 400);
    }
}
