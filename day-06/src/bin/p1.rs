use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("res/input").unwrap());
    println!("{}", puzzle(reader));
}

struct Race {
    pub time: u64,
    pub record: u64,
}

impl Race {
    fn new(time: u64, record: u64) -> Race {
        Race { time, record }
    }

    fn calc_record_possibilities(&self) -> usize {
        (0..self.time)
            .filter(|time_pressed| self.is_winning(*time_pressed))
            .count()
    }

    fn is_winning(&self, time_pressed: u64) -> bool {
        let speed = &time_pressed;
        speed * (self.time - time_pressed) > self.record
    }
}

fn puzzle<R: BufRead>(reader: R) -> usize {
    let mut lines = reader.lines().filter_map(Result::ok);

    let line1 = lines.next().unwrap();
    let times = line1[6..]
        .split_whitespace()
        .map(str::parse::<u64>)
        .filter_map(Result::ok);

    let line2 = lines.next().unwrap();
    let records = line2[9..]
        .split_whitespace()
        .map(str::parse::<u64>)
        .filter_map(Result::ok);

    let solution = times
        .zip(records)
        .map(|(time, record)| Race::new(time, record))
        .map(|race| race.calc_record_possibilities())
        .fold(1, |acc, item| acc * item);

    solution
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn example() {
        let example = String::from(
            "Time:      7  15   30
Distance:  9  40  200",
        );

        assert_eq!(puzzle(BufReader::new(Cursor::new(example))), 288);
    }

    #[test]
    fn possible_records_1() {
        let race = Race::new(7, 9);
        assert_eq!(race.calc_record_possibilities(), 4);
    }

    #[test]
    fn possible_records_2() {
        let race = Race::new(15, 40);
        assert_eq!(race.calc_record_possibilities(), 8);
    }

    #[test]
    fn possible_records_3() {
        let race = Race::new(30, 200);
        assert_eq!(race.calc_record_possibilities(), 9);
    }
}
