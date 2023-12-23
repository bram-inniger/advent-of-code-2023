use std::ops::Not;

use itertools::Itertools;
use rustc_hash::FxHashSet;

type Coord = (usize, usize);
type NeighbourFn = dyn Fn(&Trails, &Coord, &FxHashSet<Coord>) -> Vec<(Coord, u32)>;

pub fn solve_1(trails: &[&str]) -> u32 {
    Trails::new(trails).longest_path(&Trails::neighbours_sloped)
}

pub fn solve_2(trails: &[&str]) -> u32 {
    Trails::new(trails).longest_path(&Trails::neighbours_all)
}

#[derive(Debug)]
struct Trails {
    tiles: Vec<Vec<Tile>>,
    height: usize,
    width: usize,
    start: Coord,
    end: Coord,
}

impl Trails {
    fn new(tiles: &[&str]) -> Self {
        let tiles = tiles
            .iter()
            .map(|&l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Tile::Paths,
                        '#' => Tile::Forest,
                        '^' => Tile::SlopeUp,
                        '>' => Tile::SlopeRight,
                        'v' => Tile::SlopeDown,
                        '<' => Tile::SlopeLeft,
                        _ => unreachable!(),
                    })
                    .collect_vec()
            })
            .collect_vec();
        let height = tiles.len();
        let width = tiles[0].len();
        let start = (1, 0);
        let end = (width - 2, height - 1);

        Trails {
            tiles,
            height,
            width,
            start,
            end,
        }
    }

    fn longest_path(&self, neighbours: &NeighbourFn) -> u32 {
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
        neighbours: &NeighbourFn,
    ) {
        if coord == self.end {
            hikes.push(distance)
        }

        seen.insert(coord);

        for (neighbour, delta) in neighbours(self, &coord, seen) {
            Self::dfs(self, neighbour, seen, distance + delta, hikes, neighbours);
            seen.remove(&neighbour);
        }
    }

    fn neighbours_sloped(&self, coord: &Coord, seen: &FxHashSet<Coord>) -> Vec<(Coord, u32)> {
        let mut neighbours = vec![];

        if coord.0 > 0 && seen.contains(&(coord.0 - 1, coord.1)).not() {
            match self.tiles[coord.1][coord.0 - 1] {
                Tile::Paths => neighbours.push(((coord.0 - 1, coord.1), 1)),
                Tile::SlopeLeft => neighbours.push(((coord.0 - 2, coord.1), 2)),
                _ => {}
            }
        }
        if coord.1 > 0 && seen.contains(&(coord.0, coord.1 - 1)).not() {
            match self.tiles[coord.1 - 1][coord.0] {
                Tile::Paths => neighbours.push(((coord.0, coord.1 - 1), 1)),
                Tile::SlopeUp => neighbours.push(((coord.0, coord.1 - 2), 2)),
                _ => {}
            }
        }
        if coord.0 < self.width - 1 && seen.contains(&(coord.0 + 1, coord.1)).not() {
            match self.tiles[coord.1][coord.0 + 1] {
                Tile::Paths => neighbours.push(((coord.0 + 1, coord.1), 1)),
                Tile::SlopeRight => neighbours.push(((coord.0 + 2, coord.1), 2)),
                _ => {}
            }
        }
        if coord.1 < self.height - 1 && seen.contains(&(coord.0, coord.1 + 1)).not() {
            match self.tiles[coord.1 + 1][coord.0] {
                Tile::Paths => neighbours.push(((coord.0, coord.1 + 1), 1)),
                Tile::SlopeDown => neighbours.push(((coord.0, coord.1 + 2), 2)),
                _ => {}
            }
        }

        neighbours
    }

    fn neighbours_all(&self, coord: &Coord, seen: &FxHashSet<Coord>) -> Vec<(Coord, u32)> {
        let mut neighbours = vec![];

        if coord.0 > 0
            && seen.contains(&(coord.0 - 1, coord.1)).not()
            && matches!(self.tiles[coord.1][coord.0 - 1], Tile::Forest).not()
        {
            neighbours.push(((coord.0 - 1, coord.1), 1))
        }
        if coord.1 > 0
            && seen.contains(&(coord.0, coord.1 - 1)).not()
            && matches!(self.tiles[coord.1 - 1][coord.0], Tile::Forest).not()
        {
            neighbours.push(((coord.0, coord.1 - 1), 1))
        }
        if coord.0 < self.width - 1
            && seen.contains(&(coord.0 + 1, coord.1)).not()
            && matches!(self.tiles[coord.1][coord.0 + 1], Tile::Forest).not()
        {
            neighbours.push(((coord.0 + 1, coord.1), 1))
        }
        if coord.1 < self.height - 1
            && seen.contains(&(coord.0, coord.1 + 1)).not()
            && matches!(self.tiles[coord.1 + 1][coord.0], Tile::Forest).not()
        {
            neighbours.push(((coord.0, coord.1 + 1), 1))
        }

        neighbours
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
