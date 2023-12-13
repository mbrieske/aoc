pub fn solve(input: &str) -> usize {
    input.lines().map(process_line).sum()
}

fn process_line(_line: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn example() {
        let example = "";
        assert_eq!(solve(example), 0);
    }

    #[rstest]
    #[case("asd", 2)]
    #[case("fgh", 2)]
    fn test_process_line(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process_line(input), expected);
    }
}
