use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};

fn main() {
    let reader = BufReader::new(File::open("res/input").unwrap());
    println!("{}", puzzle(reader));
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Range {
    pub from: usize,
    pub len: usize,
}

impl Range {
    fn new(from: usize, len: usize) -> Range {
        Range { from, len }
    }

    fn end(&self) -> usize {
        self.from + self.len - 1
    }
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

    fn translate_ranges(&self, mut from_ranges: Vec<Range>) -> Vec<Range> {
        let mut to_ranges: Vec<Range> = Vec::new();

        for (src, (_, len)) in &self.map {
            let mut i = 0;
            while i < from_ranges.len() {
                let from_range = from_ranges.remove(i);
                // Overlap if:
                // a_start is less than or equal to b_end AND
                // b_start is less than or equal to a_end.
                if from_range.from <= *src + *len - 1 && from_range.end() > *src {
                    let overlap_start = max(from_range.from, *src);
                    let overlap_end = min(from_range.end(), *src + *len - 1);

                    // Push translated overlapped Range to to_ranges
                    to_ranges.push(Range::new(
                        self.get(overlap_start).unwrap(),
                        overlap_end - overlap_start + 1,
                    ));

                    // Create new ranges from not translated range and push to back of from_ranges
                    if from_range.from < overlap_start {
                        from_ranges.insert(
                            0,
                            Range::new(from_range.from, overlap_start - from_range.from),
                        );
                        i += 1;
                    }
                    if from_range.end() > overlap_end {
                        from_ranges.insert(
                            0,
                            Range::new(overlap_end + 1, from_range.end() - overlap_end),
                        );
                        i += 1;
                    }
                } else {
                    from_ranges.insert(0, from_range);
                    i += 1;
                }
            }
        }
        to_ranges.extend(from_ranges.iter());
        to_ranges
    }
}

fn puzzle<R: BufRead>(reader: R) -> usize {
    let mut lines = reader
        .lines()
        .filter_map(Result::ok)
        .chain(std::iter::once("".to_string()));

    let mut seed_ranges: Vec<Range> = lines.next().unwrap()[7..]
        .split(' ')
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
        .collect::<Vec<usize>>()
        .chunks(2)
        .fold(Vec::new(), |mut all, new_range| {
            all.push(Range::new(new_range[0], new_range[1]));
            all
        });

    lines.next();

    let mut map: RangeMap = RangeMap::new();

    for line in lines {
        if line.is_empty() {
            seed_ranges = map.translate_ranges(seed_ranges);
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
    seed_ranges
        .iter()
        .map(|range| range.from)
        .min()
        .unwrap()
        .to_owned()
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

    assert_eq!(puzzle(BufReader::new(Cursor::new(example))), 46);
}

#[test]
fn map_get_ranges() {
    let mut map = RangeMap::new();
    map.insert(50, 60, 20);

    let test_range = vec![Range::new(55, 10)];
    let expected = vec![Range::new(65, 10)];
    assert_eq!(map.translate_ranges(test_range), expected);
}

#[test]
fn map_get_ranges_overlapping() {
    let mut map = RangeMap::new();
    map.insert(50, 60, 20);

    let test_range = vec![Range::new(55, 20)];
    let expected = vec![Range::new(65, 15), Range::new(70, 5)];
    assert_eq!(map.translate_ranges(test_range), expected);
}

#[test]
fn map_get_ranges_overlapping_both() {
    let mut map = RangeMap::new();
    map.insert(50, 60, 1);

    let test_range = vec![Range::new(45, 10)];
    let expected = vec![Range::new(60, 1), Range::new(51, 4), Range::new(45, 5)];
    assert_eq!(map.translate_ranges(test_range), expected);
}

#[test]
fn map_get_ranges_same() {
    let mut map = RangeMap::new();
    map.insert(50, 60, 20);

    let test_range = vec![Range::new(50, 20)];
    let expected = vec![Range::new(60, 20)];
    assert_eq!(map.translate_ranges(test_range), expected);
}

#[test]
fn map_get_ranges_one_off() {
    let mut map = RangeMap::new();
    map.insert(50, 60, 20);

    let test_range = vec![Range::new(49, 22)];
    let expected = vec![Range::new(60, 20), Range::new(70, 1), Range::new(49, 1)];
    assert_eq!(map.translate_ranges(test_range), expected);
}
