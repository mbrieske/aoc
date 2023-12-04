use std::cmp::min;
use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
struct Number {
    pub line_index: usize,
    pub pos: usize,
    pub len: usize,
}

impl Number {
    fn new(line_index: usize, pos: usize) -> Number {
        Number {
            line_index,
            pos,
            len: 1,
        }
    }

    fn get_end(&self) -> usize {
        self.pos + self.len
    }

    fn increase(&mut self) {
        self.len += 1;
    }

    fn is_valid(&self, input: &str) -> bool {
        match self.line_index {
            0 => input.lines().take(2).any(|line| {
                line[self.pos.saturating_sub(1)..min(self.pos + self.len + 1, line.len())]
                    .chars()
                    .any(|ch| !ch.is_numeric() && !(ch == '.'))
            }),
            _ => input.lines().skip(self.line_index - 1).take(3).any(|line| {
                line[self.pos.saturating_sub(1)..min(self.pos + self.len + 1, line.len())]
                    .chars()
                    .any(|ch| !ch.is_numeric() && !(ch == '.'))
            }),
        }
    }

    fn get_valid_sum(&self, input: &str) -> usize {
        if self.is_valid(input) {
            input
                .lines()
                .skip(self.line_index)
                .take(1)
                .collect::<Vec<&str>>()[0][self.pos..self.pos + self.len]
                .parse()
                .unwrap()
        } else {
            0
        }
    }
}

fn main() {
    let input = read_to_string("res/input").unwrap();
    let result: usize = input
        .lines()
        .zip(0..)
        .map(|(line, line_index)| sum_valid_numbers(&input, line_index, line))
        .sum();
    println!("{result}");
}

fn sum_valid_numbers(input: &str, line_index: usize, line: &str) -> usize {
    find_numbers(line, line_index)
        .into_iter()
        .map(|number| number.get_valid_sum(input))
        .sum()
}

fn find_numbers(line: &str, line_index: usize) -> Vec<Number> {
    line.char_indices()
        .filter(|(_, ch)| ch.is_alphanumeric())
        .fold(Vec::new(), |mut acc, (pos, _)| {
            match acc.last_mut() {
                Some(last) if last.get_end() == pos => {
                    last.increase();
                }
                _ => acc.push(Number::new(line_index, pos)),
            }
            acc
        })
}

#[test]
fn test_find_numbers() {
    assert_eq!(
        find_numbers("...%.564..", 0),
        vec![Number {
            line_index: 0,
            pos: 5,
            len: 3
        }]
    );
}

#[test]
fn test_find_numbers_start_and_end() {
    assert_eq!(
        find_numbers("54%.564", 0),
        vec![
            Number {
                line_index: 0,
                pos: 0,
                len: 2
            },
            Number {
                line_index: 0,
                pos: 4,
                len: 3
            }
        ]
    );
}
