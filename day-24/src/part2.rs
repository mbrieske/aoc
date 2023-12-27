use tracing::info;
use z3::ast::{Ast, Int};
use z3::*;

struct Hailstone {
    x: i64,
    y: i64,
    z: i64,
    dx: i64,
    dy: i64,
    dz: i64,
}

impl From<&str> for Hailstone {
    fn from(line: &str) -> Self {
        let (pos, speed) = line.split_once(" @ ").unwrap();
        if let [x, y, z] = pos
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i64>>()[..]
        {
            if let [dx, dy, dz] = speed
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i64>>()[..]
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

pub fn solve(input: &str) -> i64 {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let z = Int::new_const(&ctx, "z");
    let dx = Int::new_const(&ctx, "dx");
    let dy = Int::new_const(&ctx, "dy");
    let dz = Int::new_const(&ctx, "dz");

    let hailstones: Vec<_> = input.lines().map(Hailstone::from).collect();

    hailstones.iter().enumerate().for_each(|(i, hailstone)| {
        let xi = Int::from_i64(&ctx, hailstone.x);
        let yi = Int::from_i64(&ctx, hailstone.y);
        let zi = Int::from_i64(&ctx, hailstone.z);
        let dxi = Int::from_i64(&ctx, hailstone.dx);
        let dyi = Int::from_i64(&ctx, hailstone.dy);
        let dzi = Int::from_i64(&ctx, hailstone.dz);

        let t = Int::new_const(&ctx, i.to_string());
        solver.assert(&t.ge(&Int::from_i64(&ctx, 0)));

        solver.assert(&(&xi + &t * &dxi)._eq(&(&x + &t * &dx)));
        solver.assert(&(&yi + &t * &dyi)._eq(&(&y + &t * &dy)));
        solver.assert(&(&zi + &t * &dzi)._eq(&(&z + &t * &dz)));
    });

    match solver.check() {
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let xv = model.eval(&x, true).unwrap().as_i64().unwrap();
            let yv = model.eval(&y, true).unwrap().as_i64().unwrap();
            let zv = model.eval(&z, true).unwrap().as_i64().unwrap();
            let dxv = model.eval(&dx, true).unwrap().as_i64().unwrap();
            let dyv = model.eval(&dy, true).unwrap().as_i64().unwrap();
            let dzv = model.eval(&dz, true).unwrap().as_i64().unwrap();
            info!("x: {}", xv);
            info!("y: {}", yv);
            info!("z: {}", zv);
            info!("dx: {}", dxv);
            info!("dy: {}", dyv);
            info!("dz: {}", dzv);
            xv + yv + zv
        }
        _ => {
            unreachable!()
        }
    }
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
    #[case(EXAMPLE, 47)]
    fn test_example(#[case] input: &str, #[case] expected: i64) {
        crate::utils::tracing_init();
        assert_eq!(solve(input), expected);
    }
}
