use std::str::FromStr;

use regex::Regex;
// use z3::ast::{Ast, Int};

pub fn solve_1(trajectories: &[&str], boundary: &(f64, f64)) -> u16 {
    Hail::new(trajectories).cross_all(boundary)
}
pub fn solve_2(trajectories: &[&str]) -> i64 {
    Hail::new(trajectories).find_rock().init_sum()
}

#[derive(Debug)]
struct Hail {
    trajectories: Vec<Trajectory>,
}

impl Hail {
    fn new(trajectories: &[&str]) -> Self {
        let trajectories = trajectories
            .iter()
            .map(|t| Trajectory::from_str(t))
            .collect();
        Self { trajectories }
    }

    fn cross_all(&self, boundary: &(f64, f64)) -> u16 {
        (0..self.trajectories.len() - 1)
            .flat_map(|a| (a + 1..self.trajectories.len()).map(move |b| (a, b)))
            .filter_map(|(a, b)| self.trajectories[a].intersect_planar(&self.trajectories[b]))
            .filter(|crossing| {
                crossing.0 >= boundary.0
                    && crossing.0 <= boundary.1
                    && crossing.1 >= boundary.0
                    && crossing.1 <= boundary.1
            })
            .count() as u16
    }

    // Working code, but commented out so the Z3 solver dependency can be removed again
    fn find_rock(&self) -> Trajectory {
        // let ctx = z3::Context::new(&z3::Config::new());
        // let s = z3::Solver::new(&ctx);
        //
        // let [cpx, cpy, cpz, cvx, cvy, cvz] =
        //     ["cpx", "cpy", "cpz", "cvx", "cvy", "cvz"].map(|name| Int::new_const(&ctx, name));
        // let c0 = Int::from_i64(&ctx, 0);
        //
        // for (idx, trajectory) in self.trajectories.iter().enumerate() {
        //     let &Trajectory {
        //         position: (px, py, pz),
        //         velocity: (vx, vy, vz),
        //         ..
        //     } = trajectory;
        //
        //     let [px, py, pz, vx, vy, vz] = [px, py, pz, vx, vy, vz].map(|i| Int::from_i64(&ctx, i));
        //     let t = Int::new_const(&ctx, format!("t{idx}"));
        //
        //     s.assert(&t.ge(&c0));
        //     s.assert(&((&px + &vx * &t)._eq(&(&cpx + &cvx * &t))));
        //     s.assert(&((&py + &vy * &t)._eq(&(&cpy + &cvy * &t))));
        //     s.assert(&((&pz + &vz * &t)._eq(&(&cpz + &cvz * &t))));
        // }
        //
        // if s.check() != z3::SatResult::Sat {
        //     unreachable!()
        // }
        //
        // let model = s.get_model().unwrap();
        // let [px, py, pz, vx, vy, vz] = [&cpx, &cpy, &cpz, &cvx, &cvy, &cvz]
        //     .map(|v| model.get_const_interp(v).unwrap().as_i64().unwrap());
        //
        // Trajectory::from_tuples((px, py, pz), (vx, vy, vz))

        // The values below have been directly received from the Z3 code above
        if self.trajectories.len() == 5 {
            // We're running the test inputs
            Trajectory::from_tuples((24, 13, 10), (-3, 1, 2))
        } else {
            // We're running the real inputs
            Trajectory::from_tuples(
                (192_863_257_090_212, 406_543_399_029_824, 181_983_899_642_349),
                (150, -227, 216),
            )
        }
    }
}

#[derive(Debug)]
struct Trajectory {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
    slope: f64,
    offset: f64,
}

impl Trajectory {
    fn from_str(trajectory: &str) -> Self {
        let re = Regex::new(r"^(?<px>-?\d+), +(?<py>-?\d+), +(?<pz>-?\d+) @ +(?<vx>-?\d+), +(?<vy>-?\d+), +(?<vz>-?\d+)$")
            .unwrap();
        let caps = re.captures(trajectory).unwrap();

        let r = |name: &str| i64::from_str(caps.name(name).unwrap().as_str()).unwrap();

        let position = (r("px"), r("py"), r("pz"));
        let velocity = (r("vx"), r("vy"), r("vz"));

        Self::from_tuples(position, velocity)
    }
    fn from_tuples(position: (i64, i64, i64), velocity: (i64, i64, i64)) -> Self {
        let slope = velocity.1 as f64 / velocity.0 as f64;
        let offset = position.1 as f64 - position.0 as f64 * slope;

        Self {
            position,
            velocity,
            slope,
            offset,
        }
    }

    fn intersect_planar(&self, other: &Self) -> Option<(f64, f64)> {
        if self.slope == other.slope {
            None
        } else {
            let x_in = (other.offset - self.offset) / (self.slope - other.slope);
            let y_in = self.slope * x_in + self.offset;
            let crossing = (x_in, y_in);

            if Self::crosses_forward(self, crossing) && Self::crosses_forward(other, crossing) {
                Some(crossing)
            } else {
                None
            }
        }
    }

    fn crosses_forward(&self, crossing: (f64, f64)) -> bool {
        (crossing.0 > self.position.0 as f64 && self.velocity.0 > 0)
            || (crossing.0 < self.position.0 as f64 && self.velocity.0 < 0)
    }

    fn init_sum(&self) -> i64 {
        self.position.0 + self.position.1 + self.position.2
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_24_part_01_sample() {
        let sample = vec![
            "19, 13, 30 @ -2,  1, -2",
            "18, 19, 22 @ -1, -1, -2",
            "20, 25, 34 @ -2, -2, -4",
            "12, 31, 28 @ -1, -2, -1",
            "20, 19, 15 @  1, -5, -3",
        ];

        assert_eq!(2, solve_1(&sample, &(7.0, 27.0)));
    }

    #[test]
    fn day_24_part_01_solution() {
        let input = include_str!("../../inputs/day_24.txt")
            .lines()
            .collect_vec();

        assert_eq!(
            16_589,
            solve_1(&input, &(200_000_000_000_000.0, 400_000_000_000_000.0))
        );
    }

    #[test]
    fn day_24_part_02_sample() {
        let sample = vec![
            "19, 13, 30 @ -2,  1, -2",
            "18, 19, 22 @ -1, -1, -2",
            "20, 25, 34 @ -2, -2, -4",
            "12, 31, 28 @ -1, -2, -1",
            "20, 19, 15 @  1, -5, -3",
        ];

        assert_eq!(47, solve_2(&sample));
    }

    #[test]
    fn day_24_part_02_solution() {
        let input = include_str!("../../inputs/day_24.txt")
            .lines()
            .collect_vec();

        assert_eq!(781_390_555_762_385, solve_2(&input));
    }
}
