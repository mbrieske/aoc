use cached::proc_macro::cached;

pub fn solve(input: &str, repeat: usize) -> usize {
    input.lines().map(|line| process_line(line, repeat)).sum()
}

fn process_line(line: &str, repeat: usize) -> usize {
    let (record, groups) = line.split_once(' ').unwrap();

    let record = (0..repeat)
        .map(|_| record.to_string())
        .collect::<Vec<String>>()
        .join("?");

    let groups: Vec<usize> = groups
        .split(',')
        .map(|g| g.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .repeat(repeat);

    fit(record, groups)
}

#[cached]
fn fit(record: String, groups: Vec<usize>) -> usize {
    if let Some((gi, &g)) = groups.iter().enumerate().max_by_key(|&(_, g)| g) {
        if record.len() < g {
            0
        } else {
            let (groups_left, groups_right) = groups.split_at(gi);
            let groups_right = &groups_right[1..];

            let mut possible_placements = Vec::new();
            for i in 0..record.len() - g + 1 {
                if record[i..i + g].contains('.')
                    || (record[..i].len() + 1
                        < groups_left.iter().sum::<usize>() + groups_left.len())
                    || record[i + g..].len() + 1
                        < groups_right.iter().sum::<usize>() + groups_right.len()
                {
                    continue;
                } else {
                    if (i == 0 || record.as_bytes()[i - 1] != '#' as u8)
                        && (i + g == record.len() || record.as_bytes()[i + g] != '#' as u8)
                    {
                        possible_placements.push(i)
                    }
                }
            }

            possible_placements
                .iter()
                .map(|&placement_index| {
                    let (mut rec_left, mut rec_right) = record.split_at(placement_index);
                    let options_left = {
                        rec_left = if rec_left.len() <= 1 {
                            ""
                        } else {
                            &rec_left[..rec_left.len() - 1]
                        };

                        if groups_left.len() == 0 {
                            if rec_left.contains('#') {
                                0
                            } else {
                                1
                            }
                        } else {
                            fit(rec_left.to_string(), groups_left.to_vec())
                        }
                    };

                    if options_left > 0 {
                        let options_right = {
                            rec_right = if rec_right.len() <= 1 + g {
                                ""
                            } else {
                                &rec_right[1 + g..]
                            };

                            if groups_right.len() == 0 {
                                if rec_right.contains('#') {
                                    0
                                } else {
                                    1
                                }
                            } else {
                                fit(rec_right.to_string(), groups_right.to_vec())
                            }
                        };
                        options_left * options_right
                    } else {
                        0
                    }
                })
                .sum()
        }
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

        assert_eq!(solve(example, 1), 21);
    }

    #[test]
    fn test_line_1() {
        let line = "???.### 1,1,3";
        assert_eq!(process_line(line, 1), 1);
    }

    #[test]
    fn test_line_2() {
        let line = ".??..??...?##. 1,1,3";
        assert_eq!(process_line(line, 1), 4);
    }

    #[test]
    fn test_line_3() {
        let line = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(process_line(line, 1), 1);
    }

    #[test]
    fn test_line_4() {
        let line = "????.#...#... 4,1,1";
        assert_eq!(process_line(line, 1), 1);
    }

    #[test]
    fn test_line_5() {
        let line = "????.######..#####. 1,6,5";
        assert_eq!(process_line(line, 1), 4);
    }

    #[test]
    fn test_line_6() {
        let line = "?###???????? 3,2,1";
        assert_eq!(process_line(line, 1), 10);
    }

    #[test]
    fn test_fit() {
        let record = ".?#?????###???. 1,6,1";
        assert_eq!(process_line(record, 1), 3);
    }

    #[test]
    fn test_fit_easy() {
        let record = "??? 1";
        assert_eq!(process_line(record, 1), 3);
    }

    #[test]
    fn test_line() {
        let line = "????.?#????#?? 2,1,1,3";
        assert_eq!(process_line(line, 1), 12);
    }

    #[test]
    fn test_line2() {
        let line = "??.#? 2";
        assert_eq!(process_line(line, 1), 1);
    }

    #[test]
    fn example_p2() {
        let example = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(solve(example, 5), 525152);
    }
}
