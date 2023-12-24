use std::str::FromStr;

use regex::Regex;

pub fn solve_1(trajectories: &[&str], boundary: &(f64, f64)) -> u16 {
    let hail = Hail::new(trajectories);
    hail.cross_all(boundary)
}

#[derive(Debug)]
struct Hail {
    trajectories: Vec<Trajectory>,
}

impl Hail {
    fn new(trajectories: &[&str]) -> Self {
        let trajectories = trajectories.iter().map(|t| Trajectory::new(t)).collect();
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
}

#[derive(Debug)]
struct Trajectory {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
    slope: f64,
    offset: f64,
}

impl Trajectory {
    fn new(trajectory: &str) -> Self {
        let re = Regex::new(r"^(?<px>-?\d+), +(?<py>-?\d+), +(?<pz>-?\d+) @ +(?<vx>-?\d+), +(?<vy>-?\d+), +(?<vz>-?\d+)$")
            .unwrap();
        let caps = re.captures(trajectory).unwrap();

        let r = |name: &str| i64::from_str(caps.name(name).unwrap().as_str()).unwrap();

        let position = (r("px"), r("py"), r("pz"));
        let velocity = (r("vx"), r("vy"), r("vz"));
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
}
