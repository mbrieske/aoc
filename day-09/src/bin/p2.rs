use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("res/input").unwrap());
    println!("{}", puzzle(reader));
}

fn puzzle<R: BufRead>(reader: R) -> usize {
    let mut lines = reader.lines().map_while(Result::ok);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn example() {
        let example = String::from("");

        assert_eq!(puzzle(BufReader::new(Cursor::new(example))), 0);
    }
}
