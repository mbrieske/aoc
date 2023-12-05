use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};

fn main() {
    let reader = BufReader::new(File::open("res/input").unwrap());
    println!("{}", puzzle(reader));
}

struct RangeMap {
    map: HashMap<usize, (usize, usize)>,
}

impl RangeMap {
    fn new() -> RangeMap {
        RangeMap {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, src: usize, dest: usize, range: usize) {
        self.map.insert(src, (dest, range));
    }

    fn clear(&mut self) {
        self.map.clear();
    }

    fn get(&self, src: usize) -> Option<usize> {
        let res = self
            .map
            .iter()
            .find(|(src_start, (_, range))| **src_start <= src && src < *src_start + range);
        if let Some((src_start, (dest_start, _))) = res {
            Some(src + dest_start - src_start)
        } else {
            None
        }
    }
}

fn puzzle<R: BufRead>(reader: R) -> usize {
    let mut lines = reader
        .lines()
        .filter_map(Result::ok)
        .chain(std::iter::once("".to_string()));

    let mut seeds: Vec<usize> = lines
        .next()
        .map(|line| {
            line[7..]
                .split(' ')
                .map(str::parse::<usize>)
                .filter_map(Result::ok)
                .collect()
        })
        .unwrap();

    lines.next();

    let mut map: RangeMap = RangeMap::new();

    for line in lines {
        if line.is_empty() {
            for src in seeds.iter_mut() {
                if let Some(dest) = map.get(*src) {
                    *src = dest;
                }
            }
        } else if line.ends_with("map:") {
            map.clear();
        } else {
            let [dest, source, range]: [usize; 3] = line
                .split(' ')
                .map(str::parse::<usize>)
                .filter_map(Result::ok)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            // let extend = (source..source + range).zip(dest..dest + range);
            // map.extend(extend);
            map.insert(source, dest, range);
        }
    }
    seeds.iter().min().unwrap().to_owned()
}

#[test]
fn example() {
    let example = String::from(
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
    );

    assert_eq!(puzzle(BufReader::new(Cursor::new(example))), 35);
}
