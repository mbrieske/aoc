use core::num;
use std::cmp::min;
use std::collections::HashMap;
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
        let valid = match self.line_index {
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
        };
        valid
    }

    fn get_sum(&self, input: &str) -> usize {
        input
            .lines()
            .skip(self.line_index)
            .take(1)
            .collect::<Vec<&str>>()[0][self.pos..self.pos + self.len]
            .parse()
            .unwrap()
    }
}

fn main() {
    let input = read_to_string("res/input").unwrap();
    let mut numbers_map: HashMap<usize, Number> = HashMap::new();
    input.lines().zip(0..).for_each(|(line, line_index)| {
        find_numbers(line, line_index)
            .into_iter()
            .for_each(|number| {
                numbers_map.insert(line.len() * line_index + number.pos, number);
            })
    });

    let mut keys = numbers_map.keys().collect::<Vec<&usize>>();
    keys.sort();

    let gear_values: usize = input
        .lines()
        .zip(0 as usize..)
        .map(|(line, line_index)| {
            line.char_indices()
                .filter(|(_, ch)| *ch == '*')
                .map(|(pos, _)| {
                    let lines = input.lines().skip(line_index.saturating_sub(1));
                    let relevant_lines = if line_index == 0 {
                        lines.take(2)
                    } else {
                        lines.take(3)
                    };
                    let adjacent_parts: Vec<&Number> = relevant_lines
                        .zip(if line_index == 0 {
                            0..
                        } else {
                            line_index - 1..
                        })
                        .map(|(line, line_index)| {
                            // Mid
                            let mut adjacent_parts: Vec<&Number> = Vec::new();
                            let pos_global = line.len() * line_index + pos;
                            if keys.contains(&&pos_global) {
                                adjacent_parts.push(numbers_map.get(&pos_global).unwrap());
                            }
                            // if mid contains first digit of part number, both left and right cannot be start of part numbers
                            else {
                                // check right side only if not at the end of line
                                if pos_global % line.len() <= line.len() - 2 {
                                    if keys.contains(&&(pos_global + 1)) {
                                        adjacent_parts
                                            .push(numbers_map.get(&&(pos_global + 1)).unwrap());
                                    }
                                }
                                // check if left is part number
                                if pos_global % line.len() > 1 {
                                    if line.chars().collect::<Vec<char>>()[pos - 1].is_numeric() {
                                        let index = keys.binary_search(&&(pos_global - 1));
                                        if let Ok(_) = index {
                                            // left is start of part number
                                            adjacent_parts
                                                .push(numbers_map.get(&&(pos_global - 1)).unwrap());
                                        } else {
                                            let index = index.err().unwrap() - 1;
                                            let key = keys.get(index).unwrap();
                                            let number = numbers_map.get(key).unwrap();
                                            adjacent_parts.push(number);
                                        }
                                    }
                                }
                            }
                            adjacent_parts
                        })
                        .flatten()
                        .filter(|part| part.is_valid(&input))
                        .collect();
                    if adjacent_parts.len() == 2 {
                        adjacent_parts.get(0).unwrap().get_sum(&input)
                            * adjacent_parts.get(1).unwrap().get_sum(&input)
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum();
    println!("{gear_values}");
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
