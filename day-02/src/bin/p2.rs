use regex::Regex;
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let file = File::open("res/input").unwrap();
    let re = Regex::new(r"Game \d+: (.*)").unwrap();

    let result = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let captures = re.captures(&line).unwrap();
            let record = captures.get(1).unwrap().as_str();
            calc_power(record)
        })
        .sum::<u32>();

    println!("{result}");
}

fn calc_power(record: &str) -> u32 {
    let fewest: Set = record
        .split("; ")
        .map(|setstr| {
            let mut set = Set::default();
            setstr.split(", ").for_each(|reveal| {
                let (num, color) = reveal.split_once(' ').unwrap();
                let num = num.parse::<u32>().unwrap();
                match color {
                    "red" => set.red = num,
                    "green" => set.green = num,
                    "blue" => set.blue = num,
                    &_ => unreachable!(),
                }
            });
            set
        })
        .fold(Set::default(), |mut acc, item| {
            acc.red = max(acc.red, item.red);
            acc.green = max(acc.green, item.green);
            acc.blue = max(acc.blue, item.blue);
            acc
        });
    fewest.red * fewest.green * fewest.blue
}

#[test]
fn power() {
    assert_eq!(
        calc_power("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
        48
    )
}
