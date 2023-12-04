use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let file = File::open("res/input").unwrap();
    println!("{}", process_lines(BufReader::new(file).lines()));
}

fn process_lines<I>(lines: I) -> usize
where
    I: Iterator<Item = io::Result<String>>,
{
    let card_values: Vec<usize> = lines.filter_map(Result::ok).map(process_line).collect();
    let mut owned_cards = vec![1; card_values.len()];

    card_values.iter().enumerate().for_each(|(id, card_value)| {
        let num_current_card = *owned_cards.get(id).unwrap();
        for i in id + 1..id + 1 + card_value {
            let next_card = owned_cards.get_mut(i).unwrap();
            *next_card += num_current_card;
        }
    });

    owned_cards.iter().sum()
}

fn process_line(line: String) -> usize {
    let (winning_str, mine) = line.split_once(':').unwrap().1.split_once('|').unwrap();
    let winning: HashSet<&str> = winning_str.split_whitespace().collect();
    mine.split_whitespace()
        .filter(|num| winning.contains(num))
        .count()
}

#[test]
fn example() {
    let example = String::from(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    );

    let res = process_lines(example.lines().map(|line| Ok(line.to_string())));
    assert_eq!(res, 30);
}

#[test]
fn linetest() {
    let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    assert_eq!(process_line(String::from(line)), 4)
}
