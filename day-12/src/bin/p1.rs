use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("res/input").unwrap());
    println!("{}", puzzle(reader));
}

fn puzzle<R: BufRead>(reader: R) -> usize {
    reader.lines().map_while(Result::ok).map(process_line).sum()
}

fn process_line(line: String) -> usize {
    let (record, groups) = line.split_once(' ').unwrap();
    let unknowns = &record
        .char_indices()
        .filter(|&(_, c)| c == '?')
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    let groups: Vec<usize> = groups.split(',').map(|g| g.parse().unwrap()).collect();
    let mut record = record.as_bytes().to_owned();
    let n = unknowns.len();
    let num_total_combinations = 1_usize << n; // 2^n

    let mut working: usize = 0;

    for combination in 0..num_total_combinations {
        for i in 0..n {
            if combination & (1 << i) != 0 {
                record[unknowns[i]] = '.' as u8;
            } else {
                record[unknowns[i]] = '#' as u8;
            }
        }
        if get_groups(&record) == groups {
            working += 1;
        }
    }
    working
}

fn get_groups(record: &Vec<u8>) -> Vec<usize> {
    record
        .into_iter()
        .fold(vec![0; 1], |mut acc: Vec<usize>, c| {
            if *c == '#' as u8 {
                *acc.last_mut().unwrap() += 1;
            } else {
                acc.push(0);
            }
            acc
        })
        .into_iter()
        .filter(|&n| n > 0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn example() {
        let example = String::from(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );

        assert_eq!(puzzle(BufReader::new(Cursor::new(example))), 21);
    }

    #[test]
    fn test_get_groups() {
        let line = "####.#...#... 4,1,1";
        assert_eq!(get_groups(&line.as_bytes().to_owned()), vec![4, 1, 1]);
    }

    #[test]
    fn test_line() {
        let line = "????.######..#####. 1,6,5".to_string();
        assert_eq!(process_line(line), 4);
    }
}
