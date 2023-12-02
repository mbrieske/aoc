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

    let result = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|record| calc_power(record.split_once(':').unwrap().1))
        .sum::<u32>();

    println!("{result}");
}

fn calc_power(record: &str) -> u32 {
    let fewest: Set = record
        .split([',', ';'])
        .map(|reveal| {
            let (num, color) = reveal[1..].split_once(' ').unwrap();
            let num = num.parse::<u32>().unwrap();
            (color, num)
        })
        .fold(Set::default(), |mut acc, (color, num)| {
            match color {
                "red" => acc.red = max(acc.red, num),
                "green" => acc.green = max(acc.green, num),
                "blue" => acc.blue = max(acc.blue, num),
                &_ => unreachable!(),
            }
            acc
        });
    fewest.red * fewest.green * fewest.blue
}

#[test]
fn power() {
    assert_eq!(
        calc_power(" 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
        48
    )
}
