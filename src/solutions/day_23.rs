use std::ops::Not;

use rustc_hash::{FxHashMap, FxHashSet};

type Coord = (i16, i16);
type NeighboursFn = dyn Fn(&Trails, Coord) -> Vec<Coord>;

const DIRECTIONS: [(i16, i16); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn solve_1(trails: &[&str]) -> u32 {
    Trails::new(trails).longest_path(&Trails::neighbours_sloped)
}

pub fn solve_2(trails: &[&str]) -> u32 {
    Trails::new(trails).longest_path(&Trails::neighbours_all)
}

#[derive(Debug)]
struct Trails {
    tiles: FxHashMap<Coord, Tile>,
    start: Coord,
    end: Coord,
}

impl Trails {
    fn new(tiles: &[&str]) -> Self {
        let start = (1, 0);
        let end = (tiles[0].len() as i16 - 2, tiles.len() as i16 - 1);
        let tiles = tiles
            .iter()
            .enumerate()
            .flat_map(|(y, &l)| {
                l.chars().enumerate().map(move |(x, c)| {
                    (
                        (x as i16, y as i16),
                        match c {
                            '.' => Tile::Paths,
                            '#' => Tile::Forest,
                            '^' => Tile::SlopeUp,
                            '>' => Tile::SlopeRight,
                            'v' => Tile::SlopeDown,
                            '<' => Tile::SlopeLeft,
                            _ => unreachable!(),
                        },
                    )
                })
            })
            .filter(|(_, t)| matches!(t, Tile::Forest).not())
            .collect();

        Self { tiles, start, end }
    }

    fn longest_path(&self, neighbours: &NeighboursFn) -> u32 {
        let mut hikes = vec![];
        let mut seen = FxHashSet::default();

        Self::dfs(self, self.start, &mut seen, 0, &mut hikes, neighbours);

        hikes.into_iter().max().unwrap()
    }

    fn dfs(
        &self,
        coord: Coord,
        seen: &mut FxHashSet<Coord>,
        distance: u32,
        hikes: &mut Vec<u32>,
        neighbours: &NeighboursFn,
    ) {
        if coord == self.end {
            hikes.push(distance)
        }

        seen.insert(coord);

        for neighbour in neighbours(self, coord) {
            if seen.contains(&neighbour).not() {
                Self::dfs(self, neighbour, seen, distance + 1, hikes, neighbours);
            }
        }

        seen.remove(&coord);
    }

    fn neighbours_sloped(&self, coord: Coord) -> Vec<Coord> {
        match self.tiles[&coord] {
            Tile::Paths => Self::neighbours_all(self, coord),
            Tile::Forest => unreachable!(),
            Tile::SlopeUp => vec![(coord.0, coord.1 - 1)],
            Tile::SlopeRight => vec![(coord.0 + 1, coord.1)],
            Tile::SlopeDown => vec![(coord.0, coord.1 + 1)],
            Tile::SlopeLeft => vec![(coord.0 - 1, coord.1)],
        }
    }

    fn neighbours_all(&self, coord: Coord) -> Vec<Coord> {
        DIRECTIONS
            .iter()
            .map(|d| (coord.0 + d.0, coord.1 + d.1))
            .filter(|c| self.tiles.contains_key(c))
            .collect()
    }
}

#[derive(Debug)]
enum Tile {
    Paths,
    Forest,
    SlopeUp,
    SlopeRight,
    SlopeDown,
    SlopeLeft,
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_23_part_01_sample() {
        let sample = vec![
            "#.#####################",
            "#.......#########...###",
            "#######.#########.#.###",
            "###.....#.>.>.###.#.###",
            "###v#####.#v#.###.#.###",
            "###.>...#.#.#.....#...#",
            "###v###.#.#.#########.#",
            "###...#.#.#.......#...#",
            "#####.#.#.#######.#.###",
            "#.....#.#.#.......#...#",
            "#.#####.#.#.#########v#",
            "#.#...#...#...###...>.#",
            "#.#.#v#######v###.###v#",
            "#...#.>.#...>.>.#.###.#",
            "#####v#.#.###v#.#.###.#",
            "#.....#...#...#.#.#...#",
            "#.#########.###.#.#.###",
            "#...###...#...#...#.###",
            "###.###.#.###v#####v###",
            "#...#...#.#.>.>.#.>.###",
            "#.###.###.#.###.#.#v###",
            "#.....###...###...#...#",
            "#####################.#",
        ];

        assert_eq!(94, solve_1(&sample));
    }

    #[test]
    fn day_23_part_01_solution() {
        let input = include_str!("../../inputs/day_23.txt")
            .lines()
            .collect_vec();

        assert_eq!(2_114, solve_1(&input));
    }

    #[test]
    fn day_23_part_02_sample() {
        let sample = vec![
            "#.#####################",
            "#.......#########...###",
            "#######.#########.#.###",
            "###.....#.>.>.###.#.###",
            "###v#####.#v#.###.#.###",
            "###.>...#.#.#.....#...#",
            "###v###.#.#.#########.#",
            "###...#.#.#.......#...#",
            "#####.#.#.#######.#.###",
            "#.....#.#.#.......#...#",
            "#.#####.#.#.#########v#",
            "#.#...#...#...###...>.#",
            "#.#.#v#######v###.###v#",
            "#...#.>.#...>.>.#.###.#",
            "#####v#.#.###v#.#.###.#",
            "#.....#...#...#.#.#...#",
            "#.#########.###.#.#.###",
            "#...###...#...#...#.###",
            "###.###.#.###v#####v###",
            "#...#...#.#.>.>.#.>.###",
            "#.###.###.#.###.#.#v###",
            "#.....###...###...#...#",
            "#####################.#",
        ];

        assert_eq!(154, solve_2(&sample));
    }

    #[ignore = "correct, but very slow"]
    #[test]
    fn day_23_part_02_solution() {
        let input = include_str!("../../inputs/day_23.txt")
            .lines()
            .collect_vec();

        assert_eq!(6_322, solve_2(&input));
    }
}
