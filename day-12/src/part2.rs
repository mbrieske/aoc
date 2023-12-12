// use rayon::prelude::*;
// use std::sync::atomic::{AtomicUsize, Ordering};

// static COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn solve(input: &str, repeat: usize) -> usize {
    // rayon::ThreadPoolBuilder::new()
    //     .num_threads(1)
    //     .build_global()
    //     .unwrap();
    input.lines().map(|line| process_line(line, repeat)).sum()
    // input.par_lines().map(|line| process_line(line, 5)).sum()
}

fn process_line(line: &str, repeat: usize) -> usize {
    // dbg!(&COUNT);
    // COUNT.fetch_add(1, Ordering::SeqCst);
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

    fit(&record, groups)
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

            let mut sum: usize = 0;

            for placement_index in possible_placements {
                let rec_remaining = if placement_index > 1 {
                    &record[..placement_index - 1]
                } else {
                    ""
                };
                if record[placement_index + g..].contains('#') {
                    continue; // not a valid placement
                } else if groups.len() == 0 {
                    if !rec_remaining.contains('#') {
                        sum += 1;
                    } else {
                        continue;
                    }
                } else if placement_index > 0 {
                    sum += fit(rec_remaining, groups.clone())
                } else {
                    continue;
                }
            }

            sum

            // possible_placements
            //     .iter()
            //     .map(|&placement_index| {
            //         let rec_remaining = if placement_index > 1 {
            //             &record[..placement_index - 1]
            //         } else {
            //             ""
            //         };
            //         if record[placement_index + g..].contains('#') {
            //             0
            //         } else if groups.len() == 0 {
            //             if !rec_remaining.contains('#') {
            //                 1
            //             } else {
            //                 0
            //             }
            //         } else if placement_index > 0 {
            //             fit(rec_remaining, groups.clone())
            //         } else {
            //             0
            //         }
            //     })
            //     .sum()
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
