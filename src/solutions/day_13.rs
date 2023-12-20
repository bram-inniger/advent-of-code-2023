use std::cmp::min;
use std::fmt::{self, Formatter};

use itertools::Itertools;

pub fn solve_1(ash: &str) -> usize {
    solve(ash, &(|p| p.find_mirror()))
}

pub fn solve_2(ash: &str) -> usize {
    solve(ash, &(|p| p.repair_smudge()))
}

fn solve(ash: &str, finder: &dyn Fn(Pattern) -> Mirror) -> usize {
    ash.split("\n\n")
        .map(Pattern::new)
        .map(finder)
        .map(|m| match m.alignment {
            Align::Horizontal => 100 * m.location,
            Align::Vertical => m.location,
        })
        .sum()
}

#[derive(Debug, Clone)]
struct Pattern {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let pattern = self
            .tiles
            .iter()
            .map(|l| {
                l.iter()
                    .map(|t| match t {
                        Tile::Ash => '.',
                        Tile::Rocks => '#',
                    })
                    .join("")
            })
            .join("\n");
        write!(f, "{}", pattern)
    }
}

impl Pattern {
    fn new(pattern: &str) -> Pattern {
        let tiles = pattern
            .split('\n')
            .map(|s| {
                s.chars()
                    .map(|c| match c {
                        '.' => Tile::Ash,
                        '#' => Tile::Rocks,
                        _ => panic!("Invalid tile: {}", c),
                    })
                    .collect_vec()
            })
            .collect_vec();
        let width = tiles[0].len();
        let height = tiles.len();

        Pattern {
            tiles,
            width,
            height,
        }
    }

    fn find_mirror(&self) -> Mirror {
        Self::single(Self::find_mirrors(self))
    }

    fn find_mirrors(&self) -> Vec<Mirror> {
        let mut mirrors = Vec::new();

        for y in 1..self.height {
            if Self::is_reflection(self, y, &Align::Horizontal) {
                mirrors.push(Mirror {
                    location: y,
                    alignment: Align::Horizontal,
                });
            }
        }

        for x in 1..self.width {
            if Self::is_reflection(self, x, &Align::Vertical) {
                mirrors.push(Mirror {
                    location: x,
                    alignment: Align::Vertical,
                });
            }
        }

        mirrors
    }

    fn single<T: Copy>(vec: Vec<T>) -> T {
        if vec.len() != 1 {
            panic!("Expected 1 element in Vec but received {}", vec.len());
        }

        vec[0]
    }

    fn is_reflection(&self, location: usize, alignment: &Align) -> bool {
        let delta = match alignment {
            Align::Horizontal => min(location, self.height - location),
            Align::Vertical => min(location, self.width - location),
        };

        (0..delta).all(|d| Self::equals(self, location - 1 - d, location + d, alignment))
    }

    fn equals(&self, a: usize, b: usize, alignment: &Align) -> bool {
        match alignment {
            Align::Horizontal => (0..self.width).all(|x| self.tiles[a][x] == self.tiles[b][x]),
            Align::Vertical => (0..self.height).all(|y| self.tiles[y][a] == self.tiles[y][b]),
        }
    }

    fn repair_smudge(&self) -> Mirror {
        let original = self.find_mirror();
        let repaired = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .flat_map(|t| Self::smudge(self, t.0, t.1))
            .unique()
            .filter(|&m| m != original)
            .collect_vec();

        Self::single(repaired)
    }

    fn smudge(&self, x: usize, y: usize) -> Vec<Mirror> {
        let mut smudged = self.clone();
        smudged.tiles[y][x] = match smudged.tiles[y][x] {
            Tile::Ash => Tile::Rocks,
            Tile::Rocks => Tile::Ash,
        };

        smudged.find_mirrors()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Ash,
    Rocks,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Mirror {
    location: usize,
    alignment: Align,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Align {
    Horizontal,
    Vertical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_13_part_01_sample() {
        let sample = "#.##..##.\n\
            ..#.##.#.\n\
            ##......#\n\
            ##......#\n\
            ..#.##.#.\n\
            ..##..##.\n\
            #.#.##.#.\n\
            \n\
            #...##..#\n\
            #....#..#\n\
            ..##..###\n\
            #####.##.\n\
            #####.##.\n\
            ..##..###\n\
            #....#..#";

        assert_eq!(405, solve_1(sample));
    }

    #[test]
    fn day_13_part_01_solution() {
        let input = include_str!("../../inputs/day_13.txt");

        assert_eq!(33_047, solve_1(input));
    }

    #[test]
    fn day_13_part_02_sample() {
        let sample = "#.##..##.\n\
            ..#.##.#.\n\
            ##......#\n\
            ##......#\n\
            ..#.##.#.\n\
            ..##..##.\n\
            #.#.##.#.\n\
            \n\
            #...##..#\n\
            #....#..#\n\
            ..##..###\n\
            #####.##.\n\
            #####.##.\n\
            ..##..###\n\
            #....#..#";

        assert_eq!(400, solve_2(sample));
    }

    #[test]
    fn day_13_part_02_solution() {
        let input = include_str!("../../inputs/day_13.txt");

        assert_eq!(28_806, solve_2(input));
    }
}
