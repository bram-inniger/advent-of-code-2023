use std::ops::Not;

use rustc_hash::{FxHashMap, FxHashSet};

type Coord = (i16, i16);
type NeighboursFn = dyn Fn(&Trails, Coord) -> Vec<Coord>;

const DIRECTIONS: [(i16, i16); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub fn solve_1(trails: &[&str]) -> u16 {
    Trails::new(trails).longest_path(&Trails::neighbours_sloped)
}

pub fn solve_2(trails: &[&str]) -> u16 {
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
                l.chars()
                    .enumerate()
                    .filter(|(_, c)| c != &'#')
                    .map(move |(x, c)| {
                        (
                            (x as i16, y as i16),
                            match c {
                                '.' => Tile::Paths,
                                '^' => Tile::Slope(DIRECTIONS[0]),
                                '>' => Tile::Slope(DIRECTIONS[1]),
                                'v' => Tile::Slope(DIRECTIONS[2]),
                                '<' => Tile::Slope(DIRECTIONS[3]),
                                _ => unreachable!(),
                            },
                        )
                    })
            })
            .collect();

        Self { tiles, start, end }
    }

    fn longest_path(&self, neighbours: &NeighboursFn) -> u16 {
        let mut seen = FxHashSet::default();

        Self::dfs(self, self.start, &mut seen, 0, neighbours)
    }

    fn dfs(
        &self,
        coord: Coord,
        seen: &mut FxHashSet<Coord>,
        distance: u16,
        neighbours: &NeighboursFn,
    ) -> u16 {
        if coord == self.end {
            return distance;
        }

        let mut longest = 0;
        seen.insert(coord);

        for neighbour in neighbours(self, coord) {
            if seen.contains(&neighbour).not() {
                longest = longest.max(Self::dfs(self, neighbour, seen, distance + 1, neighbours));
            }
        }

        seen.remove(&coord);

        longest
    }

    fn neighbours_sloped(&self, coord: Coord) -> Vec<Coord> {
        match self.tiles[&coord] {
            Tile::Paths => Self::neighbours_all(self, coord),
            Tile::Slope(delta) => vec![(coord.0 + delta.0, coord.1 + delta.1)],
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
    Slope(Coord),
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
