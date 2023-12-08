use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("res/input").unwrap();
    println!("{}", puzzle(input));
}

fn puzzle(input: String) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();

    let map = lines
        .skip(1)
        .map(|line| line.split_once(" = ").unwrap())
        .map(|(from, to)| (from, to[1..to.len() - 1].split_once(", ").unwrap()))
        .collect::<HashMap<&str, (&str, &str)>>();

    let mut current = "AAA";

    instructions
        .chars()
        .cycle()
        .enumerate()
        .find_map(|(step, instruction)| {
            let next = map.get(current).unwrap();
            current = match instruction {
                'L' => next.0,
                'R' => next.1,
                _ => unreachable!(),
            };
            if current == "ZZZ" {
                Some(step + 1)
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let example = String::from(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );

        assert_eq!(puzzle(example), 2);
    }

    #[test]
    fn example_2() {
        let example = String::from(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );

        assert_eq!(puzzle(example), 6);
    }
}
