use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut calibration_numbers: Vec<u8> = vec![];

    let file = File::open("res/input").unwrap();
    let mut lines = io::BufReader::new(file).lines();

    while let Some(Ok(line)) = lines.next() {
        calibration_numbers.push(get_calibration_number(line));
    }

    let sum: u64 = calibration_numbers.iter().map(|x| *x as u64).sum();

    println!("{:?}", calibration_numbers);
    println!("{sum}");
}

fn get_calibration_number(line: String) -> u8 {
    let mut calibration_string = String::new();

    for ch in line.chars() {
        if ch.is_numeric() {
            calibration_string.push(ch);
            break;
        }
    }

    for ch in line.chars().rev() {
        if ch.is_numeric() {
            calibration_string.push(ch);
            break;
        }
    }

    calibration_string.parse::<u8>().unwrap()
}
