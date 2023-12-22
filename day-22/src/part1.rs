use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use itertools::Itertools;
use tracing::{event, instrument, Level};

pub fn solve(input: &str) -> usize {
    // progressbar_init(total_iterations);

    let mut bricks = input
        .lines()
        // .progress_with(progress_bar)
        .map(Brick::from)
        .collect::<Vec<Brick>>();

    while gravitate(&mut bricks) > 0 {}

    let blocking: HashMap<usize, HashSet<usize>> = bricks
        .iter()
        .map(|brick| {
            bricks
                .iter()
                .enumerate()
                .filter(|&(_, other)| brick != other && other.is_blocking(brick))
                .map(|(id, _)| id)
                .collect()
        })
        .enumerate()
        .collect();

    let important: HashSet<&usize> = blocking
        .iter()
        .filter(|&(_, supporting)| supporting.len() == 1)
        .flat_map(&|(_, supporting)| supporting)
        .collect();

    bricks
        .iter()
        .enumerate()
        .filter(|(id, _)| !important.contains(id))
        .count()
}

#[derive(Debug, PartialEq, Eq)]
struct Brick {
    x: (u16, u16),
    y: (u16, u16),
    z: (u16, u16),
    falling: bool,
}

impl From<&str> for Brick {
    fn from(s: &str) -> Self {
        let mut coords = s
            .split('~')
            .map(|coord| coord.split(',').map(|x| x.parse::<u16>().unwrap()));

        let mut coords = coords.next().unwrap().zip(coords.next().unwrap());

        let x = coords.next().unwrap();
        let y = coords.next().unwrap();
        let z = coords.next().unwrap();

        Brick {
            x,
            y,
            z,
            falling: !(z.0 == 1 || z.1 == 1),
        }
    }
}

impl Brick {
    fn is_blocking(&self, other: &Brick) -> bool {
        if self.x.1 < other.x.0 || self.x.0 > other.x.1 {
            return false;
        }
        if self.y.1 < other.y.0 || self.y.0 > other.y.1 {
            return false;
        }
        if self.z.1 + 1 < other.z.0 || self.z.0 + 1 > other.z.1 {
            return false;
        }
        true
    }
}

#[instrument (level = Level::DEBUG, skip(bricks))]
fn gravitate(bricks: &mut Vec<Brick>) -> usize {
    let mut can_fall: Vec<usize> = Vec::new();
    let mut landet: Vec<usize> = Vec::new();

    bricks
        .iter()
        .enumerate()
        .filter(|(_, brick)| brick.falling)
        .for_each(|(id, brick)| {
            if let Some(other) = bricks
                .iter()
                .filter(|&other| brick != other)
                .find(|other| other.is_blocking(brick))
            {
                event!(
                    Level::DEBUG,
                    "Brick {id} {:?} blocked by {id} {:?}",
                    brick,
                    other
                );
                if !other.falling {
                    landet.push(id)
                }
            } else {
                event!(Level::DEBUG, "Brick {id} {:?} can fall", brick);
                can_fall.push(id);
            }
        });

    can_fall.iter().for_each(|&id| {
        let brick = &mut bricks[id];
        brick.z.0 -= 1;
        brick.z.1 -= 1;
        if brick.z.0 == 1 || brick.z.1 == 1 {
            brick.falling = false;
            event!(Level::DEBUG, "Brick {id} {:?} landet on ground", brick);
        };
    });

    landet.iter().for_each(|&id| {
        let brick = &mut bricks[id];
        brick.falling = false;
        event!(Level::DEBUG, "Brick {id} {:?} landet on other", brick);
    });

    can_fall.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tracing_init;
    use rstest::rstest;

    static EXAMPLE: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[rstest]
    #[case(EXAMPLE, 5)]
    fn test_example(#[case] input: &str, #[case] expected: usize) {
        tracing_init();
        assert_eq!(solve(input), expected);
    }
}
