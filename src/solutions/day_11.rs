use std::ops::{Not, Range};

use itertools::Itertools;
use rustc_hash::FxHashSet;

pub fn solve_1(image: Vec<&str>) -> u64 {
    Space::new(image).expand(1).distance_sums()
}

pub fn solve_2(image: Vec<&str>, increase: i64) -> u64 {
    Space::new(image).expand(increase).distance_sums()
}

#[derive(Debug)]
struct Space {
    galaxies: FxHashSet<(i64, i64)>,
    horizontal: Range<i64>,
    vertical: Range<i64>,
}

impl Space {
    fn new(image: Vec<&str>) -> Space {
        let image = image.iter().map(|&l| l.chars().collect_vec()).collect_vec();

        let galaxies = (0..image.len())
            .flat_map(|y| (0..image[0].len()).map(move |x| (x, y)))
            .filter(|&(x, y)| image[y][x] == '#')
            .map(|(x, y)| (x as i64, y as i64))
            .collect();
        let horizontal = 0..(image[0].len() as i64);
        let vertical = 0..(image.len() as i64);

        Space {
            galaxies,
            horizontal,
            vertical,
        }
    }

    fn expand(&self, increase: i64) -> Space {
        let empty_rows = Self::empty_rows(self);
        let empty_cols = Self::empty_cols(self);

        let mut galaxies = self.galaxies.clone();

        empty_rows.iter().rev().for_each(|&r| {
            galaxies = galaxies
                .iter()
                .map(|&g| if g.1 > r { (g.0, g.1 + increase) } else { g })
                .collect()
        });

        empty_cols.iter().rev().for_each(|&c| {
            galaxies = galaxies
                .iter()
                .map(|&g| if g.0 > c { (g.0 + increase, g.1) } else { g })
                .collect()
        });

        Space {
            galaxies,
            horizontal: 0..self.horizontal.end + empty_rows.len() as i64,
            vertical: 0..self.vertical.end + empty_cols.len() as i64,
        }
    }

    fn empty_rows(&self) -> Vec<i64> {
        self.vertical
            .clone()
            .filter(|y| self.galaxies.iter().any(|g| g.1 == *y).not())
            .collect()
    }

    fn empty_cols(&self) -> Vec<i64> {
        self.horizontal
            .clone()
            .filter(|x| self.galaxies.iter().any(|g| g.0 == *x).not())
            .collect()
    }

    fn distance_sums(&self) -> u64 {
        self.galaxies
            .iter()
            .map(|g_1| {
                self.galaxies
                    .iter()
                    .filter(|&g_2| g_1 != g_2)
                    .map(|g_2| i64::abs(g_1.0 - g_2.0) + i64::abs(g_1.1 - g_2.1))
                    .map(|d| d as u64)
                    .sum::<u64>()
            })
            .sum::<u64>()
            / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_11_part_01_sample() {
        let sample = vec![
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#.....",
        ];

        assert_eq!(374, solve_1(sample));
    }

    #[test]
    fn day_11_part_01_solution() {
        let input = include_str!("../../inputs/day_11.txt").lines().collect();

        assert_eq!(10_494_813, solve_1(input));
    }

    #[test]
    fn day_11_part_02_sample() {
        let sample = vec![
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#.....",
        ];

        assert_eq!(1_030, solve_2(sample.clone(), 10 - 1));
        assert_eq!(8_410, solve_2(sample, 100 - 1));
    }

    #[test]
    fn day_11_part_02_solution() {
        let input = include_str!("../../inputs/day_11.txt").lines().collect();

        assert_eq!(840_988_812_853, solve_2(input, 1000000 - 1));
    }
}
