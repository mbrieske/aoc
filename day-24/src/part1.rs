use itertools::Itertools;
use tracing::debug;
use z3::ast::{Ast, Real};
use z3::*;

#[derive(Debug, Clone)]
struct Hailstone {
    x: i32,
    y: i32,
    z: i32,
    dx: i32,
    dy: i32,
    dz: i32,
}

impl From<&str> for Hailstone {
    fn from(line: &str) -> Self {
        let (pos, speed) = line.split_once(" @ ").unwrap();
        if let [x, y, z] = pos
            .split(", ")
            .map(|s: &str| s.parse().unwrap())
            .collect::<Vec<i32>>()[..]
        {
            if let [dx, dy, dz] = speed
                .split(", ")
                .map(|s: &str| s.parse().unwrap())
                .collect::<Vec<i32>>()[..]
            {
                Self {
                    x,
                    y,
                    z,
                    dx,
                    dy,
                    dz,
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }
}

pub fn solve(input: &str, bounds: (i32, i32)) -> usize {
    // progressbar_init(total_iterations);
    // .progress_with(progress_bar)
    let cfg = Config::new();

    input
        .lines()
        .map(Hailstone::from)
        .tuple_combinations()
        .filter(|(hailstone, other)| {
            let ctx = Context::new(&cfg);
            let solver = Solver::new(&ctx);

            let x1 = Real::from_real(&ctx, hailstone.x, 1);
            let y1 = Real::from_real(&ctx, hailstone.y, 1);
            let dx1 = Real::from_real(&ctx, hailstone.dx, 1);
            let dy1 = Real::from_real(&ctx, hailstone.dy, 1);

            let x2 = Real::from_real(&ctx, other.x, 1);
            let y2 = Real::from_real(&ctx, other.y, 1);
            let dx2 = Real::from_real(&ctx, other.dx, 1);
            let dy2 = Real::from_real(&ctx, other.dy, 1);

            let lower_bound = Real::from_real(&ctx, bounds.0, 1);
            let upper_bound = Real::from_real(&ctx, bounds.1, 1);

            // Variables representing the parameterization of the lines
            let t = Real::new_const(&ctx, "t");
            let s = Real::new_const(&ctx, "s");

            // Equations for the intersection of the vectors
            let eq1 = Real::add(&ctx, &[&x1, &Real::mul(&ctx, &[&t, &dx1])])
                ._eq(&Real::add(&ctx, &[&x2, &Real::mul(&ctx, &[&s, &dx2])]));
            let eq2 = Real::add(&ctx, &[&y1, &Real::mul(&ctx, &[&t, &dy1])])
                ._eq(&Real::add(&ctx, &[&y2, &Real::mul(&ctx, &[&s, &dy2])]));

            // Add constraints to the solver
            solver.assert(&eq1);
            solver.assert(&eq2);
            solver.assert(&t.ge(&Real::from_real(&ctx, 0, 1)));
            solver.assert(&s.ge(&Real::from_real(&ctx, 0, 1)));
            solver.assert(&Real::add(&ctx, &[&x1, &Real::mul(&ctx, &[&t, &dx1])]).ge(&lower_bound));
            solver.assert(&Real::add(&ctx, &[&x2, &Real::mul(&ctx, &[&s, &dx2])]).ge(&lower_bound));
            solver.assert(&Real::add(&ctx, &[&y1, &Real::mul(&ctx, &[&t, &dy1])]).ge(&lower_bound));
            solver.assert(&Real::add(&ctx, &[&y2, &Real::mul(&ctx, &[&s, &dy2])]).ge(&lower_bound));
            solver.assert(&Real::add(&ctx, &[&x1, &Real::mul(&ctx, &[&t, &dx1])]).le(&upper_bound));
            solver.assert(&Real::add(&ctx, &[&x2, &Real::mul(&ctx, &[&s, &dx2])]).le(&upper_bound));
            solver.assert(&Real::add(&ctx, &[&y1, &Real::mul(&ctx, &[&t, &dy1])]).le(&upper_bound));
            solver.assert(&Real::add(&ctx, &[&y2, &Real::mul(&ctx, &[&s, &dy2])]).le(&upper_bound));

            match solver.check() {
                z3::SatResult::Sat => {
                    let model = solver.get_model().unwrap();

                    let intersection_x = model
                        .eval(
                            &Real::add(&ctx, &[&x1, &Real::mul(&ctx, &[&t, &dx1])]),
                            true,
                        )
                        .unwrap()
                        .as_real()
                        .unwrap();
                    let intersection_y = model
                        .eval(
                            &Real::add(&ctx, &[&y1, &Real::mul(&ctx, &[&t, &dy1])]),
                            true,
                        )
                        .unwrap()
                        .as_real()
                        .unwrap();

                    let t = model.eval(&t, true).unwrap().as_real().unwrap();
                    let s = model.eval(&s, true).unwrap().as_real().unwrap();

                    debug!(
                        "{:?} and {:?} intersect at x={}, y={}, t={}, s={}",
                        hailstone,
                        other,
                        intersection_x.0 as f32 / intersection_x.1 as f32,
                        intersection_y.0 as f32 / intersection_y.1 as f32,
                        t.0 as f32 / t.1 as f32,
                        s.0 as f32 / s.1 as f32
                    );

                    true
                }

                _ => {
                    debug!(
                        "{:?} and {:?} don't intersect in the future",
                        hailstone, other
                    );
                    false
                }
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
