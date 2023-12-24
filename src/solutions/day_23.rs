use std::collections::VecDeque;
use std::ops::Not;

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

type Coord = (i16, i16);
type Weight = u16;
type NeighboursFn = dyn Fn(&FxHashMap<Coord, Tile>, &Coord, &FxHashSet<Coord>) -> Vec<Coord>;

const DIRECTIONS: [(i16, i16); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub fn solve_1(trails: &[&str]) -> u16 {
    Trails::new(trails)
        .to_graph(&Graph::neighbours_sloped)
        .longest_path()
}

pub fn solve_2(trails: &[&str]) -> u16 {
    Trails::new(trails)
        .to_graph(&Graph::neighbours_all)
        .longest_path()
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

    fn to_graph(&self, neighbours: &NeighboursFn) -> Graph {
        Graph::build(self, neighbours)
    }
}

#[derive(Debug)]
struct Graph {
    edges: Vec<Vec<(usize, Weight)>>,
    start: usize,
    end: usize,
}

impl Graph {
    fn build(trails: &Trails, neighbours: &NeighboursFn) -> Self {
        let start = trails.start;
        let end = trails.end;

        let mut vertices: FxHashSet<Coord> = trails
            .tiles
            .keys()
            .filter(|&c| Self::neighbours_count(&trails.tiles, c) > 2)
            .copied()
            .collect();
        vertices.insert(start);
        vertices.insert(end);

        let edges: FxHashMap<_, _> = vertices
            .iter()
            .map(|v| (*v, Self::edges(*v, &vertices, &trails.tiles, neighbours)))
            .collect();

        let vertices: Vec<_> = edges.keys().copied().collect();
        let vertices_lookup: FxHashMap<_, _> =
            vertices.iter().enumerate().map(|(k, v)| (v, k)).collect();
        let better_edges: Vec<Vec<(usize, Weight)>> = vertices
            .iter()
            .map(|v| {
                edges[v]
                    .iter()
                    .map(|(c, w)| (vertices_lookup[c], *w))
                    .collect_vec()
            })
            .collect_vec();
        let better_start = vertices_lookup[&start];
        let better_end = vertices_lookup[&end];

        Self {
            edges: better_edges,
            start: better_start,
            end: better_end,
        }
    }

    fn edges(
        vertex: Coord,
        vertices: &FxHashSet<Coord>,
        tiles: &FxHashMap<Coord, Tile>,
        neighbours: &NeighboursFn,
    ) -> Vec<(Coord, Weight)> {
        let mut edges = vec![];
        let mut to_visit = VecDeque::new();
        let mut seen = FxHashSet::default();
        to_visit.push_back((vertex, 0));

        while let Some((c, w)) = to_visit.pop_front() {
            seen.insert(c);

            if c != vertex && vertices.contains(&c) {
                edges.push((c, w));
                continue;
            }

            for neighbour in neighbours(tiles, &c, &seen) {
                to_visit.push_back((neighbour, w + 1));
            }
        }

        edges
    }

    fn neighbours_sloped(
        tiles: &FxHashMap<Coord, Tile>,
        coord: &Coord,
        seen: &FxHashSet<Coord>,
    ) -> Vec<Coord> {
        match tiles[&coord] {
            Tile::Paths => Self::neighbours_all(tiles, coord, seen),
            Tile::Slope(delta) => vec![(coord.0 + delta.0, coord.1 + delta.1)],
        }
    }

    fn neighbours_all(
        tiles: &FxHashMap<Coord, Tile>,
        coord: &Coord,
        seen: &FxHashSet<Coord>,
    ) -> Vec<Coord> {
        DIRECTIONS
            .iter()
            .map(|d| (coord.0 + d.0, coord.1 + d.1))
            .filter(|c| tiles.contains_key(c))
            .filter(|c| seen.contains(c).not())
            .collect()
    }

    fn neighbours_count(tiles: &FxHashMap<Coord, Tile>, coord: &Coord) -> usize {
        Self::neighbours_all(tiles, coord, &FxHashSet::default()).len()
    }

    fn longest_path(&self) -> u16 {
        let mut seen = vec![false; self.edges.len()];

        Self::dfs(self, self.start, &mut seen, 0)
    }

    fn dfs(&self, vertex: usize, seen: &mut Vec<bool>, distance: u16) -> u16 {
        if vertex == self.end {
            return distance;
        }

        let mut longest = 0;
        seen[vertex] = true;

        for (neighbour, weight) in &self.edges[vertex] {
            if seen[*neighbour].not() {
                longest = longest.max(Self::dfs(self, *neighbour, seen, distance + weight));
            }
        }

        seen[vertex] = false;
        longest
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

    #[test]
    fn day_23_part_02_solution() {
        let input = include_str!("../../inputs/day_23.txt")
            .lines()
            .collect_vec();

        assert_eq!(6_322, solve_2(&input));
    }
}
