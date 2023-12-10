use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("res/input").unwrap());
    println!("{}", puzzle(reader));
}

fn puzzle<R: BufRead>(reader: R) -> isize {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.split(' ')
                .map(str::parse::<isize>)
                .map_while(Result::ok)
                .collect::<Vec<isize>>()
        })
        .map(calculate_next)
        .sum()
}

fn calculate_next(mut history: Vec<isize>) -> isize {
    history.reverse();
    let mut tree = vec![history];
    while tree.last().unwrap().iter().any(|elem| *elem != 0) {
        tree.push(
            tree.last()
                .unwrap()
                .iter()
                .fold((Vec::new(), None), |(mut diffs, last), &elem| {
                    if let Some(last) = last {
                        diffs.push(elem - last);
                    }
                    (diffs, Some(elem))
                })
                .0,
        );
    }

    tree.last_mut().unwrap().push(0);
    let mut next: isize = 0;

    while let Some(diffs) = tree.pop() {
        next = *diffs.last().unwrap();
        if let Some(seq) = tree.last_mut() {
            seq.push(*seq.last().unwrap() + next);
        }
    }
    next
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn example() {
        let example = String::from(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );

        assert_eq!(puzzle(BufReader::new(Cursor::new(example))), 2);
    }

    #[test]
    fn example_1() {
        let testvec = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(calculate_next(testvec), -3)
    }
}
