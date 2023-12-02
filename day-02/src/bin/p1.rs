use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("res/input").unwrap();
    let re = Regex::new(r"Game (\d+): (.*)").unwrap();

    let result = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let captures = re.captures(&line).unwrap();
            let id = captures.get(1).unwrap().as_str();
            let record = captures.get(2).unwrap().as_str();
            let impossible = record.split("; ").any(|set| {
                set.split(", ").any(|reveal| {
                    let (num, color) = reveal.split_once(' ').unwrap();
                    let num = num.parse::<u8>().unwrap();
                    match color {
                        "red" => num > 12,
                        "green" => num > 13,
                        "blue" => num > 14,
                        &_ => unreachable!(),
                    }
                })
            });
            if !impossible {
                id.parse::<u32>().unwrap()
            } else {
                0
            }
        })
        .sum::<u32>();

    println!("{result}");
}
