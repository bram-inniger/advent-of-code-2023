use std::cmp::Ordering;
use std::collections::VecDeque;
use std::str::FromStr;

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

type Label = usize;

const GROUND_L: Label = usize::MAX;
const GROUND_H: LabeledHeight = LabeledHeight {
    label: GROUND_L,
    z: 0,
};

pub fn solve_1(snapshot: &[&str]) -> u32 {
    Stack::new(snapshot).count_disintegratable()
}

pub fn solve_2(snapshot: &[&str]) -> u32 {
    let stack = Stack::new(snapshot);

    stack
        .bricks
        .iter()
        .map(|b| b.label)
        .map(|b| stack.clone().chain_reaction(b))
        .sum()
}

#[derive(Debug, Clone)]
struct Stack {
    bricks: Vec<Brick>,
    planar_points: FxHashMap<PlaneCoord, LabeledHeight>,
    leans_upon: FxHashMap<Label, FxHashSet<Label>>,
    leaned_upon_by: FxHashMap<Label, FxHashSet<Label>>,
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

        let mut stack = Self {
            bricks,
            planar_points: Default::default(),
            leans_upon: Default::default(),
            leaned_upon_by: Default::default(),
        };

        stack.settle();
        stack
    }

    fn settle(&mut self) {
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
            let leaning_upon: FxHashSet<_> = Self::x_y_iter(brick)
                .map(|p| self.planar_points.get(&p).unwrap_or(&GROUND_H))
                .filter(|&lh| lh.z == brick.z.0 - 1)
                .map(|lh| lh.label)
                .collect();

            // Update the map leaned_upon_by for every brick in the collection above, adding this one
            leaning_upon.iter().for_each(|&l| {
                self.leaned_upon_by
                    .entry(l)
                    .or_default()
                    .insert(brick.label);
            });

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
    }

    fn count_disintegratable(&self) -> u32 {
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

    fn chain_reaction(mut self, init_brick: Label) -> u32 {
        let mut falling = 0;
        let mut to_remove: VecDeque<Label> = VecDeque::new();
        to_remove.push_back(init_brick);

        while let Some(brick) = to_remove.pop_front() {
            // When removing a brick, check all bricks that were leaning on it
            if let Some(leaned_upon) = self.leaned_upon_by.get_mut(&brick) {
                for leaning in leaned_upon.iter() {
                    // For every brick leaning on it, discard the removed brick
                    let leaning_on = self.leans_upon.get_mut(leaning).unwrap();
                    leaning_on.remove(&brick);

                    // If we now find that the newly inspected brick is leaning on nothing else,
                    // it starts falling, and it too should be removed,
                    // repeating the procedure until no more bricks are falling
                    if leaning_on.is_empty() {
                        to_remove.push_front(*leaning);
                        falling += 1;
                    }
                }
            }
        }

        falling
    }

    fn x_y_iter(brick: &Brick) -> impl Iterator<Item = PlaneCoord> + '_ {
        (brick.x.0..=brick.x.1)
            .flat_map(|x| (brick.y.0..=brick.y.1).map(move |y| PlaneCoord { x, y }))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
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

    #[test]
    fn day_22_part_02_sample() {
        let sample = vec![
            "1,0,1~1,2,1",
            "0,0,2~2,0,2",
            "0,2,3~2,2,3",
            "0,0,4~0,2,4",
            "2,0,5~2,2,5",
            "0,1,6~2,1,6",
            "1,1,8~1,1,9",
        ];

        assert_eq!(7, solve_2(&sample));
    }

    #[test]
    fn day_22_part_02_solution() {
        let input = include_str!("../../inputs/day_22.txt")
            .lines()
            .collect_vec();

        assert_eq!(70_609, solve_2(&input));
    }
}
