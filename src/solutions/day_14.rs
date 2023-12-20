use std::ops::Not;

use rustc_hash::FxHashMap;

pub fn solve_1(dish: Vec<&str>) -> usize {
    Dish::new(dish).tilt(&Direction::North).total_load()
}

pub fn solve_2(dish: Vec<&str>) -> usize {
    Dish::new(dish).cycle().total_load()
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Dish {
    tiles: Vec<Vec<Tile>>,
    height: usize,
    width: usize,
}

impl Dish {
    fn new(dish: Vec<&str>) -> Dish {
        let tiles: Vec<Vec<Tile>> = dish
            .iter()
            .map(|&s| {
                s.chars()
                    .map(|c| match c {
                        'O' => Tile::Rounded,
                        '#' => Tile::Cube,
                        '.' => Tile::Empty,
                        _ => panic!("Invalid tile: {c}"),
                    })
                    .collect()
            })
            .collect();
        let height = tiles.len();
        let width = tiles[0].len();

        Self {
            tiles,
            height,
            width,
        }
    }

    fn tilt(&mut self, direction: &Direction) -> &Self {
        let coords: Vec<(usize, usize)> = match direction {
            Direction::North => (0..self.height)
                .flat_map(|y| (0..self.width).map(move |x| (x, y)))
                .collect(),
            Direction::East => (0..self.width)
                .rev()
                .flat_map(|x| (0..self.height).map(move |y| (x, y)))
                .collect(),
            Direction::South => (0..self.height)
                .rev()
                .flat_map(|y| (0..self.width).map(move |x| (x, y)))
                .collect(),
            Direction::West => (0..self.width)
                .flat_map(|x| (0..self.height).map(move |y| (x, y)))
                .collect(),
        };

        for (x, y) in coords {
            if matches!(self.tiles[y][x], Tile::Rounded) {
                let new = Self::roll_single(self, (x, y), direction);

                self.tiles[y][x] = Tile::Empty;
                self.tiles[new.1][new.0] = Tile::Rounded;
            }
        }

        self
    }

    fn roll_single(&self, mut coord: (usize, usize), direction: &Direction) -> (usize, usize) {
        loop {
            match direction {
                Direction::North => {
                    if coord.1 == 0 || matches!(self.tiles[coord.1 - 1][coord.0], Tile::Empty).not()
                    {
                        return coord;
                    }
                    coord.1 -= 1;
                }
                Direction::East => {
                    if coord.0 == self.width - 1
                        || matches!(self.tiles[coord.1][coord.0 + 1], Tile::Empty).not()
                    {
                        return coord;
                    }
                    coord.0 += 1;
                }
                Direction::South => {
                    if coord.1 == self.height - 1
                        || matches!(self.tiles[coord.1 + 1][coord.0], Tile::Empty).not()
                    {
                        return coord;
                    }
                    coord.1 += 1;
                }
                Direction::West => {
                    if coord.0 == 0 || matches!(self.tiles[coord.1][coord.0 - 1], Tile::Empty).not()
                    {
                        return coord;
                    }
                    coord.0 -= 1;
                }
            }
        }
    }

    fn total_load(&self) -> usize {
        (0..self.height)
            .map(|y| {
                self.tiles[y]
                    .iter()
                    .filter(|&t| matches!(t, Tile::Rounded))
                    .count()
                    * (self.height - y)
            })
            .sum()
    }

    fn cycle(&mut self) -> &Self {
        let (repeat_start, repeat_length) = Self::detect_repeat(&mut self.clone());

        let nr_cycles = (1_000_000_000 - repeat_start) % repeat_length + repeat_start;

        for _ in 0..nr_cycles {
            self.tilt(&Direction::North);
            self.tilt(&Direction::West);
            self.tilt(&Direction::South);
            self.tilt(&Direction::East);
        }

        self
    }

    fn detect_repeat(&mut self) -> (usize, usize) {
        let mut memo: FxHashMap<Self, _> = FxHashMap::default();

        let mut repeat_start: usize = 0;
        let mut repeat_length: usize = 0;

        for cycle in 0..usize::MAX {
            if memo.contains_key(self) {
                repeat_start = memo[&self];
                repeat_length = cycle - repeat_start;
                break;
            } else {
                memo.insert(self.clone(), cycle);
            }

            self.tilt(&Direction::North);
            self.tilt(&Direction::West);
            self.tilt(&Direction::South);
            self.tilt(&Direction::East);
        }

        (repeat_start, repeat_length)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Tile {
    Rounded,
    Cube,
    Empty,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_14_part_01_sample() {
        let sample = vec![
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",
        ];

        assert_eq!(136, solve_1(sample));
    }

    #[test]
    fn day_14_part_01_solution() {
        let input = include_str!("../../inputs/day_14.txt").lines().collect();

        assert_eq!(108_857, solve_1(input));
    }

    #[test]
    fn day_14_part_02_sample() {
        let sample = vec![
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",
        ];

        assert_eq!(64, solve_2(sample));
    }

    #[test]
    fn day_14_part_02_solution() {
        let input = include_str!("../../inputs/day_14.txt").lines().collect();

        assert_eq!(95_273, solve_2(input));
    }
}
