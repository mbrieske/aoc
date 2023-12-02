use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut calibration_numbers: Vec<u8> = vec![];

    let file = File::open("res/input")?;
    let mut lines = io::BufReader::new(file).lines();

    while let Some(Ok(line)) = lines.next() {
        calibration_numbers.push(get_calibration_number(line));
    }

    let sum: u64 = calibration_numbers.iter().map(|x| *x as u64).sum();

    println!("{:?}", calibration_numbers);
    println!("{sum}");

    Ok(())
}

fn get_calibration_number(line: String) -> u8 {
    let mut calibration_string = String::new();

    for (i, ch) in line.char_indices() {
        if ch.is_numeric() {
            calibration_string.push(ch);
            break;
        } else if let Some(ch) = spelled_number_at_start(&line[i..]) {
            calibration_string.push(ch);
            break;
        }
    }

    for (i, ch) in line.char_indices().rev() {
        if ch.is_numeric() {
            calibration_string.push(ch);
            break;
        } else if let Some(ch) = spelled_number_at_end(&line[..i + 1]) {
            calibration_string.push(ch);
            break;
        }
    }

    calibration_string.parse::<u8>().unwrap()
}

fn spelled_number_at_start(string: &str) -> Option<char> {
    if string.starts_with("zero") {
        Some('0')
    } else if string.starts_with("one") {
        Some('1')
    } else if string.starts_with("two") {
        Some('2')
    } else if string.starts_with("three") {
        Some('3')
    } else if string.starts_with("four") {
        Some('4')
    } else if string.starts_with("five") {
        Some('5')
    } else if string.starts_with("six") {
        Some('6')
    } else if string.starts_with("seven") {
        Some('7')
    } else if string.starts_with("eight") {
        Some('8')
    } else if string.starts_with("nine") {
        Some('9')
    } else {
        None
    }
}

fn spelled_number_at_end(string: &str) -> Option<char> {
    if string.ends_with("zero") {
        Some('0')
    } else if string.ends_with("one") {
        Some('1')
    } else if string.ends_with("two") {
        Some('2')
    } else if string.ends_with("three") {
        Some('3')
    } else if string.ends_with("four") {
        Some('4')
    } else if string.ends_with("five") {
        Some('5')
    } else if string.ends_with("six") {
        Some('6')
    } else if string.ends_with("seven") {
        Some('7')
    } else if string.ends_with("eight") {
        Some('8')
    } else if string.ends_with("nine") {
        Some('9')
    } else {
        None
    }
}

#[test]
fn test() {
    assert_eq!(spelled_number_at_start("sevenabc"), Some('7'));
    assert_eq!(spelled_number_at_end("abcseven"), Some('7'));
}
