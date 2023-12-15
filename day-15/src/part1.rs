pub fn solve(input: &str) -> usize {
    input
        .split(',')
        .map(|s| s.chars().fold(0_usize, |acc, ch| (acc + ch as usize) * 17) % 256)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(solve(example), 1320);
    }
}
