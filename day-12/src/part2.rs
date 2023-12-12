pub fn solve(input: &str) -> usize {
    input.lines().map(process_line).sum()
}

fn process_line(line: &str) -> usize {
    let (record, groups) = line.split_once(' ').unwrap();
    let groups: Vec<usize> = groups.split(',').map(|g| g.parse().unwrap()).collect();

    fit(record, groups)
}

fn fit(record: &str, mut groups: Vec<usize>) -> usize {
    if let Some(g) = groups.pop() {
        if record.len() < g {
            0
        } else {
            let mut possible_placements = Vec::new();
            for i in 0..record.len() - g + 1 {
                if record[i..i + g].contains('.') {
                    continue;
                } else {
                    if (i == 0 || record.as_bytes()[i - 1] != '#' as u8)
                        && (i + g == record.len() || record.as_bytes()[i + g] != '#' as u8)
                    {
                        possible_placements.push(i)
                    }
                }
            }
            println!(
                "Possible placements for {} in {}: {:?}",
                g, record, possible_placements
            );
            possible_placements
                .iter()
                .map(|&placement_index| {
                    if groups.len() == 0 {
                        1
                    } else if placement_index > 0 {
                        let rec_remaining = &record[..placement_index - 1];
                        fit(rec_remaining, groups.clone())
                    } else {
                        0
                    }
                })
                .sum()
        }
    } else if !record.contains('#') {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(solve(example), 21);
    }

    #[test]
    fn test_line_1() {
        let line = "???.### 1,1,3";
        assert_eq!(process_line(line), 1);
    }

    #[test]
    fn test_line_2() {
        let line = ".??..??...?##. 1,1,3";
        assert_eq!(process_line(line), 4);
    }

    #[test]
    fn test_line_3() {
        let line = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(process_line(line), 1);
    }

    #[test]
    fn test_line_4() {
        let line = "????.#...#... 4,1,1";
        assert_eq!(process_line(line), 1);
    }

    #[test]
    fn test_line_5() {
        let line = "????.######..#####. 1,6,5";
        assert_eq!(process_line(line), 4);
    }

    #[test]
    fn test_line_6() {
        let line = "?###???????? 3,2,1";
        assert_eq!(process_line(line), 10);
    }

    #[test]
    fn test_fit() {
        let record = ".?#?????###???. 1,6,1";
        let groups = vec![1, 6, 1];
        assert_eq!(process_line(record), 3);
    }
}
