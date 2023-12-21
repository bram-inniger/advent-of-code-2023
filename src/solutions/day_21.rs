use std::collections::VecDeque;
use std::ops::Not;

use rustc_hash::{FxHashMap, FxHashSet};

pub fn solve_1(garden: &[&str], steps: u8) -> usize {
    Garden::new(garden)
        .reachable(steps)
        .iter()
        .filter(|(_, &d)| d % 2 == 0)
        .count()
}

#[derive(Debug)]
struct Garden {
    plots: FxHashSet<(i16, i16)>,
    start: (i16, i16),
}

impl Garden {
    fn new(plots: &[&str]) -> Garden {
        let plots: FxHashMap<_, _> = plots
            .iter()
            .enumerate()
            .flat_map(|(y, &s)| {
                s.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as i16, y as i16), c))
            })
            .filter(|(_, c)| *c == '.' || *c == 'S')
            .collect();

        let start = *plots.iter().find(|(_, &c)| c == 'S').unwrap().0;
        let plots = plots.into_keys().collect();

        Garden { plots, start }
    }

    fn reachable(&self, steps: u8) -> FxHashMap<(i16, i16), u8> {
        let mut to_visit: VecDeque<((i16, i16), u8)> = VecDeque::new();
        let mut seen: FxHashMap<(i16, i16), u8> = FxHashMap::default();

        to_visit.push_back((self.start, 0));

        while to_visit.is_empty().not() {
            let (next_c, next_d) = to_visit.pop_front().unwrap();

            if self.plots.contains(&next_c).not() || next_d > steps || seen.contains_key(&next_c) {
                continue;
            };

            seen.insert(next_c, next_d);

            [
                ((next_c.0, next_c.1 - 1), next_d + 1),
                ((next_c.0 + 1, next_c.1), next_d + 1),
                ((next_c.0, next_c.1 + 1), next_d + 1),
                ((next_c.0 - 1, next_c.1), next_d + 1),
            ]
                .iter()
                .for_each(|&x| to_visit.push_back(x));
        }

        seen
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_21_part_01_sample() {
        let sample = vec![
            "...........",
            ".....###.#.",
            ".###.##..#.",
            "..#.#...#..",
            "....#.#....",
            ".##..S####.",
            ".##..#...#.",
            ".......##..",
            ".##.#.####.",
            ".##..##.##.",
            "...........",
        ];

        assert_eq!(16, solve_1(&sample, 6));
    }

    #[test]
    fn day_21_part_01_solution() {
        let input = include_str!("../../inputs/day_21.txt")
            .lines()
            .collect_vec();

        assert_eq!(3_795, solve_1(&input, 64));
    }
}
