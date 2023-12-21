use std::collections::VecDeque;
use std::ops::Not;

use rustc_hash::{FxHashMap, FxHashSet};

pub fn solve_1(garden: &[&str], steps: u8) -> u16 {
    Garden::new(garden)
        .reachable(steps)
        .iter()
        .filter(|(_, &d)| d % 2 == 0)
        .count() as u16
}

pub fn solve_2(garden: &[&str], steps: u64) -> u64 {
    let garden = Garden::new(garden);
    let visited = garden.reachable(u8::MAX);

    // Solution based on https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    let even_all = visited.values().filter(|&&v| v % 2 == 1).count() as u64;
    let odd_all = visited.values().filter(|&&v| v % 2 == 0).count() as u64;
    let even_corners = visited
        .values()
        .filter(|&&v| v % 2 == 0 && v > 65)
        .count() as u64;
    let odd_corners = visited
        .values()
        .filter(|&&v| v % 2 == 1 && v > 65)
        .count() as u64;

    // The input number "26_501_365" isn't randomly chosen,
    // it decomposes as "65 + (202300 * 131)".
    // "65" is the nr steps needed to walk to the edge of one garden square
    // "131" is the width of 1 garden square.
    // This means per direction the max distance we can walk is "202300" full additional squares.
    //
    // This same variable "n" will hold that value "202300".
    //
    // These arrange neatly in a "diamond" (like the card suit).
    // Counting all reachable steps taking into account odd and even distances,
    // and taking care of the edge condition at the corners yields the result.
    let n = (steps - (garden.init_w as u64 / 2)) / garden.init_h as u64;

    (n + 1) * (n + 1) * even_all
        + (n * n) * odd_all
        - (n + 1) * odd_corners
        + n * even_corners
}

#[derive(Debug)]
struct Garden {
    plots: FxHashSet<(i16, i16)>,
    start: (i16, i16),
    init_h: i16,
    init_w: i16,
}

impl Garden {
    fn new(plots: &[&str]) -> Garden {
        let init_h = plots.len() as i16;
        let init_w = plots[0].len() as i16;
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

        Garden { plots, start, init_h, init_w }
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

    #[test]
    fn day_21_part_02_sample() {
        // No valid sample input(s) for part 2,
        // as the real input has a property (open row and column around "start"),
        // which the sample doesn't have, meaning we cannot run them.
        // Manually testing them against the code for Part 1 works fine.

        // let sample = vec![
        //     "...........",
        //     ".....###.#.",
        //     ".###.##..#.",
        //     "..#.#...#..",
        //     "....#.#....",
        //     ".##..S####.",
        //     ".##..#...#.",
        //     ".......##..",
        //     ".##.#.####.",
        //     ".##..##.##.",
        //     "...........",
        // ];
        //
        // assert_eq!(16, solve_2(&sample, 6));
        // assert_eq!(50, solve_2(&sample, 10));
        // assert_eq!(1_594, solve_2(&sample, 50));
        // assert_eq!(6_536, solve_2(&sample, 100));
        // assert_eq!(167_004, solve_2(&sample, 500));
        // assert_eq!(668_697, solve_2(&sample, 1_000));
        // assert_eq!(16_733_044, solve_2(&sample, 5_000));
    }

    #[test]
    fn day_21_part_02_solution() {
        let input = include_str!("../../inputs/day_21.txt")
            .lines()
            .collect_vec();

        assert_eq!(630_129_824_772_393, solve_2(&input, 26_501_365));
    }
}
