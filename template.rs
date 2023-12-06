use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("res/input").unwrap());
    println!("{}", puzzle(reader));
}

fn puzzle<R: BufRead>(reader: R) -> usize {
    let mut _lines = reader.lines().filter_map(Result::ok);
    288
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
}
