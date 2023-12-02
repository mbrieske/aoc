use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("res/input").unwrap();

    let result = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|record| {
            let id = get_game_id(&record);
            let impossible = record
                .split_once(':')
                .unwrap()
                .1
                .split([',', ';'])
                .any(|reveal| {
                    let (num, color) = reveal[1..].split_once(' ').unwrap();
                    let num = num.parse::<u8>().unwrap();
                    match color {
                        "red" => num > 12,
                        "green" => num > 13,
                        "blue" => num > 14,
                        &_ => unreachable!(),
                    }
                });
            if !impossible {
                id
            } else {
                0
            }
        })
        .sum::<u32>();

    println!("{result}");
}

fn get_game_id(record: &str) -> u32 {
    let re = Regex::new(r"Game (\d+)").unwrap();
    re.captures(&record)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

#[test]
fn test_id() {
    assert_eq!(get_game_id("Game 86: 8 blue, 9 green"), 86);
}
