use std::cmp::Ordering;
use std::str::FromStr;

use itertools::Itertools;
use rustc_hash::FxHashMap;

type Label = usize;

const GROUND_L: Label = usize::MAX;
const GROUND_H: LabeledHeight = LabeledHeight {
    label: GROUND_L,
    z: 0,
};

pub fn solve_1(snapshot: &[&str]) -> u32 {
    Stack::new(snapshot).settle().count_disintegrate()
}

#[derive(Debug)]
struct Stack {
    bricks: Vec<Brick>,
    planar_points: FxHashMap<PlaneCoord, LabeledHeight>,
    leans_upon: FxHashMap<Label, Vec<Label>>,
    leaned_upon_by: FxHashMap<Label, Vec<Label>>,
}

impl Stack {
    fn new(snapshot: &[&str]) -> Self {
        let mut bricks = snapshot
            .iter()
            .enumerate()
            .map(|(l, &s)| {
                let coords = s
                    .split('~')
                    .map(|p| {
                        p.split(',')
                            .map(|n| u16::from_str(n).unwrap())
                            .collect_vec()
                    })
                    .collect_vec();

                Brick {
                    label: l,
                    x: (coords[0][0], coords[1][0]),
                    y: (coords[0][1], coords[1][1]),
                    z: (coords[0][2], coords[1][2]),
                }
            })
            .collect_vec();
        bricks.sort();

        Self {
            bricks,
            planar_points: Default::default(),
            leans_upon: Default::default(),
            leaned_upon_by: Default::default(),
        }
    }

    fn settle(&mut self) -> &Self {
        for brick in &mut self.bricks {
            // For every x,y, find the drop distance of Z
            let z_drop = Self::x_y_iter(brick)
                .map(|p| brick.z.0 - self.planar_points.get(&p).map(|lh| lh.z).unwrap_or(0) - 1)
                .min()
                .unwrap();
            brick.z.0 -= z_drop;
            brick.z.1 -= z_drop;

            // For every x,y, find whether we lean on another brick (has z value new_z-1),
            // and put them in a Vec after running "unique" for de-duplication, if we lean across multiple cubes
            let leaning_upon = Self::x_y_iter(brick)
                .map(|p| self.planar_points.get(&p).unwrap_or(&GROUND_H))
                .filter(|&lh| lh.z == brick.z.0 - 1)
                .map(|lh| lh.label)
                .unique()
                .collect_vec();

            // Update the map leaned_upon_by for every brick in the collection above, adding this one
            leaning_upon
                .iter()
                .for_each(|&l| self.leaned_upon_by.entry(l).or_default().push(brick.label));

            // Update the map leans_upon for this brick with the collection above
            self.leans_upon.insert(brick.label, leaning_upon);

            // Update the planar_points map for every x-y with brick.z.1
            let lh = LabeledHeight {
                label: brick.label,
                z: brick.z.1,
            };
            Self::x_y_iter(brick).for_each(|p| {
                self.planar_points.insert(p, lh);
            })
        }

        self
    }

    fn count_disintegrate(&self) -> u32 {
        // For every brick, count all that either:
        // - have no bricks leaning on them
        // - have only bricks leaning on them that themselves lean on at least 2 bricks
        self.bricks
            .iter()
            .map(|b| b.label)
            .filter(|b| match self.leaned_upon_by.get(b) {
                None => true,
                Some(leaned_on) => leaned_on.iter().all(|l| self.leans_upon[l].len() >= 2),
            })
            .count() as u32
    }

    fn x_y_iter(brick: &Brick) -> impl Iterator<Item = PlaneCoord> + '_ {
        (brick.x.0..=brick.x.1)
            .flat_map(|x| (brick.y.0..=brick.y.1).map(move |y| PlaneCoord { x, y }))
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Brick {
    label: Label,
    x: (u16, u16),
    y: (u16, u16),
    z: (u16, u16),
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.z.0.cmp(&other.z.0).then(self.label.cmp(&other.label))
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct PlaneCoord {
    x: u16,
    y: u16,
}

#[derive(Debug, Copy, Clone)]
struct LabeledHeight {
    label: Label,
    z: u16,
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_22_part_01_sample() {
        let sample = vec![
            "1,0,1~1,2,1",
            "0,0,2~2,0,2",
            "0,2,3~2,2,3",
            "0,0,4~0,2,4",
            "2,0,5~2,2,5",
            "0,1,6~2,1,6",
            "1,1,8~1,1,9",
        ];

        assert_eq!(5, solve_1(&sample));
    }

    #[test]
    fn day_22_part_01_solution() {
        let input = include_str!("../../inputs/day_22.txt")
            .lines()
            .collect_vec();

        assert_eq!(389, solve_1(&input));
    }
}
