use std::collections::VecDeque;
use std::ops::Not;

use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};

type Coord = (i16, i16);

pub fn solve_1(layout: Vec<&str>) -> u32 {
    Layout::new(layout).energized_count_single()
}

pub fn solve_2(layout: Vec<&str>) -> u32 {
    Layout::new(layout).energized_count_all()
}

#[derive(Debug)]
struct Layout {
    grid: FxHashMap<Coord, Tile>,
    height: i16,
    width: i16,
}

impl Layout {
    fn new(layout: Vec<&str>) -> Layout {
        let grid: FxHashMap<Coord, Tile> = layout
            .iter()
            .enumerate()
            .flat_map(|(y, &s)| {
                s.chars().enumerate().map(move |(x, tile)| {
                    (
                        (x as i16, y as i16),
                        match tile {
                            '.' => Tile::Empty,
                            '/' => Tile::MirrorLU,
                            '\\' => Tile::MirrorLD,
                            '-' => Tile::SplitterLR,
                            '|' => Tile::SplitterUD,
                            _ => panic!("Invalid tile: {tile}"),
                        },
                    )
                })
            })
            .collect();
        let height = layout.len() as i16;
        let width = layout[0].len() as i16;

        Layout {
            grid,
            height,
            width,
        }
    }

    fn energized_count_single(&self) -> u32 {
        Self::bounce_light(self, ((0, 0), Direction::Right))
    }

    fn energized_count_all(&self) -> u32 {
        let up_s = (0..self.width)
            .map(|x| ((x, self.height - 1), Direction::Up))
            .collect_vec();
        let right_s = (0..self.height)
            .map(|y| ((0, y), Direction::Right))
            .collect_vec();
        let down_s = (0..self.width)
            .map(|x| ((x, 0), Direction::Down))
            .collect_vec();
        let left_s = (0..self.height)
            .map(|y| ((self.width - 1, y), Direction::Left))
            .collect_vec();

        [up_s, right_s, down_s, left_s]
            .par_iter()
            .flatten()
            .map(|&init| Self::bounce_light(self, init))
            .max()
            .unwrap()
    }

    fn bounce_light(&self, init: (Coord, Direction)) -> u32 {
        let mut seen = FxHashSet::default();
        let mut to_visit: VecDeque<(Coord, Direction)> = VecDeque::new();

        to_visit.push_back((init.0, init.1));

        while let Some((coord, direction)) = to_visit.pop_front() {
            if self.grid.contains_key(&coord).not() || seen.contains(&(coord, direction)) {
                continue;
            }
            seen.insert((coord, direction));

            let tile = self.grid[&coord];
            match tile {
                Tile::Empty => match direction {
                    Direction::Up => Self::push_up(&mut to_visit, coord),
                    Direction::Right => Self::push_right(&mut to_visit, coord),
                    Direction::Down => Self::push_down(&mut to_visit, coord),
                    Direction::Left => Self::push_left(&mut to_visit, coord),
                },
                Tile::MirrorLU => match direction {
                    Direction::Up => Self::push_right(&mut to_visit, coord),
                    Direction::Right => Self::push_up(&mut to_visit, coord),
                    Direction::Down => Self::push_left(&mut to_visit, coord),
                    Direction::Left => Self::push_down(&mut to_visit, coord),
                },
                Tile::MirrorLD => match direction {
                    Direction::Up => Self::push_left(&mut to_visit, coord),
                    Direction::Right => Self::push_down(&mut to_visit, coord),
                    Direction::Down => Self::push_right(&mut to_visit, coord),
                    Direction::Left => Self::push_up(&mut to_visit, coord),
                },
                Tile::SplitterUD => match direction {
                    Direction::Up => Self::push_up(&mut to_visit, coord),
                    Direction::Down => Self::push_down(&mut to_visit, coord),
                    Direction::Right | Direction::Left => {
                        Self::push_up(&mut to_visit, coord);
                        Self::push_down(&mut to_visit, coord);
                    }
                },
                Tile::SplitterLR => match direction {
                    Direction::Right => Self::push_right(&mut to_visit, coord),
                    Direction::Left => Self::push_left(&mut to_visit, coord),
                    Direction::Up | Direction::Down => {
                        Self::push_right(&mut to_visit, coord);
                        Self::push_left(&mut to_visit, coord);
                    }
                },
            }
        }

        seen.iter().map(|((x, y), _)| (x, y)).unique().count() as u32
    }

    fn push_up(to_visit: &mut VecDeque<(Coord, Direction)>, coord: Coord) {
        to_visit.push_back(((coord.0, coord.1 - 1), Direction::Up))
    }

    fn push_right(to_visit: &mut VecDeque<(Coord, Direction)>, coord: Coord) {
        to_visit.push_back(((coord.0 + 1, coord.1), Direction::Right))
    }

    fn push_down(to_visit: &mut VecDeque<(Coord, Direction)>, coord: Coord) {
        to_visit.push_back(((coord.0, coord.1 + 1), Direction::Down))
    }

    fn push_left(to_visit: &mut VecDeque<(Coord, Direction)>, coord: Coord) {
        to_visit.push_back(((coord.0 - 1, coord.1), Direction::Left))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Tile {
    Empty,
    MirrorLU,
    MirrorLD,
    SplitterUD,
    SplitterLR,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_16_part_01_sample() {
        let sample = vec![
            r".|...\....",
            r"|.-.\.....",
            r".....|-...",
            r"........|.",
            r"..........",
            r".........\",
            r"..../.\\..",
            r".-.-/..|..",
            r".|....-|.\",
            r"..//.|....",
        ];

        assert_eq!(46, solve_1(sample));
    }

    #[test]
    fn day_16_part_01_solution() {
        let input = include_str!("../../inputs/day_16.txt").lines().collect();

        assert_eq!(8_116, solve_1(input));
    }

    #[test]
    fn day_16_part_02_sample() {
        let sample = vec![
            r".|...\....",
            r"|.-.\.....",
            r".....|-...",
            r"........|.",
            r"..........",
            r".........\",
            r"..../.\\..",
            r".-.-/..|..",
            r".|....-|.\",
            r"..//.|....",
        ];

        assert_eq!(51, solve_2(sample));
    }

    #[test]
    fn day_16_part_02_solution() {
        let input = include_str!("../../inputs/day_16.txt").lines().collect();

        assert_eq!(8_383, solve_2(input));
    }
}
