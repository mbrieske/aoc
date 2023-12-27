use indicatif::ProgressIterator;
use itertools::Itertools;
use z3::ast::{Ast, Real};
use z3::*;

use crate::utils::progressbar_init;

#[derive(Debug, Clone)]
struct Hailstone<'a> {
    x: &'a str,
    y: &'a str,
    dx: &'a str,
    dy: &'a str,
}

impl<'a> From<&'a str> for Hailstone<'a> {
    fn from(line: &'a str) -> Self {
        let (pos, speed) = line.split_once(" @ ").unwrap();
        if let [x, y, _] = pos.split(", ").collect::<Vec<&str>>()[..] {
            if let [dx, dy, _] = speed.split(", ").collect::<Vec<&str>>()[..] {
                Self { x, y, dx, dy }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }
}

pub fn solve(input: &str, bounds: (i64, i64)) -> usize {
    let cfg = Config::new();

    let hailstone_combinations: Vec<_> = input
        .lines()
        .map(Hailstone::from)
        .tuple_combinations()
        .collect();

    let progress_bar = progressbar_init(hailstone_combinations.len() as u64);

    hailstone_combinations
        .iter()
        .progress_with(progress_bar)
        .filter(|(hailstone, other)| {
            let ctx = Context::new(&cfg);
            let solver = Solver::new(&ctx);

            let x1 = Real::from_real_str(&ctx, hailstone.x, "1").unwrap();
            let y1 = Real::from_real_str(&ctx, hailstone.y, "1").unwrap();
            let dx1 = Real::from_real_str(&ctx, hailstone.dx, "1").unwrap();
            let dy1 = Real::from_real_str(&ctx, hailstone.dy, "1").unwrap();

            let x2 = Real::from_real_str(&ctx, other.x, "1").unwrap();
            let y2 = Real::from_real_str(&ctx, other.y, "1").unwrap();
            let dx2 = Real::from_real_str(&ctx, other.dx, "1").unwrap();
            let dy2 = Real::from_real_str(&ctx, other.dy, "1").unwrap();

            let lower_bound = Real::from_real_str(&ctx, &bounds.0.to_string(), "1").unwrap();
            let upper_bound = Real::from_real_str(&ctx, &bounds.1.to_string(), "1").unwrap();

            let t = Real::new_const(&ctx, "t");
            let s = Real::new_const(&ctx, "s");

            let eq1 = &(&x1 + &t * &dx1)._eq(&(&x2 + &s * &dx2));
            let eq2 = &(&y1 + &t * &dy1)._eq(&(&y2 + &s * &dy2));

            solver.assert(eq1);
            solver.assert(eq2);
            solver.assert(&t.ge(&Real::from_real(&ctx, 0, 1)));
            solver.assert(&s.ge(&Real::from_real(&ctx, 0, 1)));
            solver.assert(&(&x1 + &t * &dx1).ge(&lower_bound));
            solver.assert(&(&x2 + &s * &dx2).ge(&lower_bound));
            solver.assert(&(&y1 + &t * &dy1).ge(&lower_bound));
            solver.assert(&(&y2 + &s * &dy2).ge(&lower_bound));
            solver.assert(&(&x1 + &t * &dx1).le(&upper_bound));
            solver.assert(&(&x2 + &s * &dx2).le(&upper_bound));
            solver.assert(&(&y1 + &t * &dy1).le(&upper_bound));
            solver.assert(&(&y2 + &s * &dy2).le(&upper_bound));

            match solver.check() {
                z3::SatResult::Sat => true,
                _ => false,
            }
        })
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::rstest;

    static EXAMPLE: &str = "19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3";

    #[rstest]
    #[case(EXAMPLE, (7, 27), 2)]
    fn test_example(#[case] input: &str, #[case] bounds: (i64, i64), #[case] expected: usize) {
        crate::utils::tracing_init();
        assert_eq!(solve(input, bounds), expected);
    }
}
