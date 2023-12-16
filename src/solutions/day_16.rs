use itertools::Itertools;
use std::collections::HashSet;
use std::collections::{HashMap, VecDeque};
use std::ops::Not;

type Coord = (i16, i16);

pub fn solve_1(layout: Vec<&str>) -> u32 {
    Layout::new(layout).energized_count()
}

#[derive(Debug)]
struct Layout {
    grid: HashMap<Coord, Tile>,
}

impl Layout {
    fn new(layout: Vec<&str>) -> Layout {
        let grid: HashMap<Coord, Tile> = layout
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

        Layout { grid }
    }

    fn energized_count(&self) -> u32 {
        Self::bounce_light(self)
            .iter()
            .map(|((x, y), _)| (x, y))
            .unique()
            .count() as u32
    }

    fn bounce_light(&self) -> HashSet<(Coord, Direction)> {
        let mut seen = HashSet::new();
        let mut to_visit: VecDeque<(Coord, Direction)> = VecDeque::new();

        to_visit.push_back(((0, 0), Direction::Right));

        while to_visit.is_empty().not() {
            let (coord, direction) = to_visit.pop_front().unwrap();
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

        seen
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
}
