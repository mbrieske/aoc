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

    let mut current = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .copied()
        .collect::<Vec<&str>>();

    instructions
        .chars()
        .cycle()
        .enumerate()
        .find_map(|(step, instruction)| {
            current = current
                .iter()
                .map(|node| match instruction {
                    'L' => map.get(node).unwrap().0,
                    'R' => map.get(node).unwrap().1,
                    _ => unreachable!(),
                })
                .collect();

            if current.iter().all(|node| node.ends_with('Z')) {
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
    fn example() {
        let example = String::from(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );

        assert_eq!(puzzle(example), 6);
    }
}
