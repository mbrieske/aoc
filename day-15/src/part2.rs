use indexmap::IndexMap;

#[must_use]
pub fn solve(input: &str) -> usize {
    let mut boxes: Vec<IndexMap<&str, u8>> = vec![IndexMap::new(); 256];

    input
        .split(',')
        .map(|substr| (substr, calc_hash(substr.split(['-', '=']).next().unwrap())))
        .for_each(|(s, box_i)| {
            if s.contains('=') {
                let (label, mirror) = s.split_once('=').unwrap();
                let mirror: u8 = mirror.parse().unwrap();
                boxes[box_i].insert(label, mirror);
            } else {
                let _ = boxes[box_i].shift_remove(&s[0..s.len() - 1]);
            }
        });

    boxes
        .iter()
        .zip(1..)
        .map(|(mbox, ibox)| {
            mbox.iter()
                .zip(1..)
                .map(&|((_, &focal_length), i)| i * focal_length as usize)
                .sum::<usize>()
        } * ibox)
        .sum()
}

fn calc_hash(s: &str) -> usize {
    s.chars().fold(0, |acc, ch| (acc + ch as usize) * 17) % 256
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(solve(example), 145);
    }
}
