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
        let mut range = (0_u64, self.time);

        let first_winning = loop {
            let test = range.0 + (range.1 - range.0) / 2;
            let res = (self.is_winning(test - 1), self.is_winning(test));
            match res {
                (false, false) => range = (test, range.1),
                (false, true) => break (test),
                _ => range = (range.0, test),
            };
        };

        range = (0, self.time);

        let last_winning = loop {
            let test = range.0 + (range.1 - range.0) / 2;
            match (self.is_winning(test), self.is_winning(test + 1)) {
                (false, false) => range = (range.0, test),
                (true, false) => break (test),
                _ => range = (test, range.1),
            };
        };

        (last_winning + 1 - first_winning) as usize
    }

    fn is_winning(&self, time_pressed: u64) -> bool {
        let speed = &time_pressed;
        speed * (self.time - time_pressed) > self.record
    }
}

fn puzzle<R: BufRead>(reader: R) -> usize {
    let mut lines = reader.lines().map_while(Result::ok);

    let line1 = lines.next().unwrap();
    let time = line1[6..]
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let line2 = lines.next().unwrap();
    let record = line2[9..]
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let race = Race::new(time, record);

    race.calc_record_possibilities()
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

        assert_eq!(puzzle(BufReader::new(Cursor::new(example))), 71503);
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
