use itertools::Itertools;
use std::cmp::min;

pub fn solve_1(ash: &str) -> usize {
    ash.split("\n\n")
        .map(Pattern::new)
        .map(|p| p.find_mirror())
        .map(|m| match m.alignment {
            Align::Horizontal => 100 * m.location,
            Align::Vertical => m.location,
        })
        .sum()
}

#[derive(Debug)]
struct Pattern {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
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
        for y in 1..self.height {
            if Self::is_reflection(self, y, &Align::Horizontal) {
                return Mirror {
                    location: y,
                    alignment: Align::Horizontal,
                };
            }
        }

        for x in 1..self.width {
            if Self::is_reflection(self, x, &Align::Vertical) {
                return Mirror {
                    location: x,
                    alignment: Align::Vertical,
                };
            }
        }

        panic!("Couldn't find the mirror in pattern {:?}", self)
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
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Ash,
    Rocks,
}

#[derive(Debug)]
struct Mirror {
    location: usize,
    alignment: Align,
}

#[derive(Debug)]
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
}
