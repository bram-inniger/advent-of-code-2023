use std::collections::HashSet;
use std::ops::{Index, Not};

pub fn solve_1(maze: Vec<&str>) -> usize {
    Maze::new(maze).pipe_loop.len() / 2
}

pub fn solve_2(maze: Vec<&str>) -> usize {
    Maze::new(maze).count_enclosed_tiles()
}

#[derive(Debug)]
struct Maze {
    tiles: Vec<Vec<Tile>>,
    pipe_loop: HashSet<(usize, usize)>,
}

impl Maze {
    fn new(maze: Vec<&str>) -> Maze {
        let tiles: Vec<_> = maze
            .iter()
            .map(|l| l.chars().map(Tile::new).collect())
            .collect();
        let pipe_loop = Self::find_loop(&tiles);

        Maze { tiles, pipe_loop }
    }

    fn find_loop(tiles: &Vec<Vec<Tile>>) -> HashSet<(usize, usize)> {
        let start = Self::find_start(tiles);
        let mut pipe_loop = HashSet::new();
        pipe_loop.insert(start);

        let mut prev = start;
        let mut current = Self::next(tiles, start, start);

        while current != start {
            pipe_loop.insert(current);
            let next = Self::next(tiles, prev, current);

            prev = current;
            current = next;
        }

        pipe_loop
    }

    fn find_start(tiles: &Vec<Vec<Tile>>) -> (usize, usize) {
        (0..tiles.len())
            .flat_map(|y| (0..tiles[0].len()).map(move |x| (x, y)))
            .find(|&(x, y)| matches!(tiles[y][x], Tile::Start))
            .unwrap()
    }

    fn next(
        tiles: &Vec<Vec<Tile>>,
        prev: (usize, usize),
        current: (usize, usize),
    ) -> (usize, usize) {
        *Self::neighbours(tiles, current)
            .iter()
            .find(|&&n| n != prev)
            .unwrap()
    }

    fn neighbours(tiles: &Vec<Vec<Tile>>, tile: (usize, usize)) -> Vec<(usize, usize)> {
        match tiles[tile.1][tile.0] {
            Tile::NorthSouth => vec![(tile.0, tile.1 - 1), (tile.0, tile.1 + 1)],
            Tile::EastWest => vec![(tile.0 + 1, tile.1), (tile.0 - 1, tile.1)],
            Tile::NorthEast => vec![(tile.0, tile.1 - 1), (tile.0 + 1, tile.1)],
            Tile::NorthWest => vec![(tile.0, tile.1 - 1), (tile.0 - 1, tile.1)],
            Tile::SouthWest => vec![(tile.0, tile.1 + 1), (tile.0 - 1, tile.1)],
            Tile::SouthEast => vec![(tile.0, tile.1 + 1), (tile.0 + 1, tile.1)],
            Tile::Ground => vec![],
            Tile::Start => Self::start_neighbours(tiles, tile),
        }
    }

    fn start_neighbours(tiles: &Vec<Vec<Tile>>, start: (usize, usize)) -> Vec<(usize, usize)> {
        let height = tiles.len();
        let width = tiles[0].len();
        let mut neighbours = Vec::new();

        if start.0 >= 1 {
            if let Tile::EastWest | Tile::NorthEast | Tile::SouthEast = &tiles[start.1][start.0 - 1]
            {
                neighbours.push((start.0 - 1, start.1));
            }
        }

        if start.1 >= 1 {
            if let Tile::NorthSouth | Tile::SouthWest | Tile::SouthEast =
                &tiles[start.1 - 1][start.0]
            {
                neighbours.push((start.0, start.1 - 1));
            }
        }

        if start.0 < width - 1 {
            if let Tile::EastWest | Tile::NorthWest | Tile::SouthWest = &tiles[start.1][start.0 + 1]
            {
                neighbours.push((start.0 + 1, start.1));
            }
        }

        if start.1 < height - 1 {
            if let Tile::NorthSouth | Tile::NorthEast | Tile::NorthWest =
                &tiles[start.1 + 1][start.0]
            {
                neighbours.push((start.0, start.1 + 1));
            }
        }

        neighbours
    }

    fn count_enclosed_tiles(&self) -> usize {
        let horizontal_starts: Vec<_> = (0..self.tiles[0].len()).map(|x| (x, 0usize)).collect();
        let vertical_starts: Vec<_> = (1..self.tiles.len()).map(|y| (0usize, y)).collect();

        horizontal_starts
            .into_iter()
            .chain(vertical_starts)
            .map(|s| Self::diagonal_ray(self, s))
            .sum()
    }

    fn diagonal_ray(&self, start: (usize, usize)) -> usize {
        let mut inside = false;
        let mut current = start;
        let mut inside_points = 0;

        let width = self.tiles[0].len();
        let height = self.tiles.len();

        while current.0 < width && current.1 < height {
            let tile = &self[current];

            if self.pipe_loop.contains(&current) {
                // The diagonal line "grazes" these specific 2 pipes
                // Because no crossing between outside/inside occurred, we simply ignore this tile
                if matches!(tile, Tile::NorthEast).not() && matches!(tile, Tile::SouthWest).not() {
                    inside = inside.not()
                }
            } else if inside {
                inside_points += 1
            }

            current = (current.0 + 1, current.1 + 1);
        }

        inside_points
    }
}

impl Index<(usize, usize)> for Maze {
    type Output = Tile;

    // the index is formatted (x, y)
    fn index(&self, idx: (usize, usize)) -> &Tile {
        &self.tiles[idx.1][idx.0]
    }
}

#[derive(Debug)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Tile {
    fn new(tile: char) -> Tile {
        match tile {
            '|' => Tile::NorthSouth,
            '-' => Tile::EastWest,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("Received invalid tile: {tile}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_10_part_01_sample() {
        let sample = vec!["-L|F7", "7S-7|", "L|7||", "-L-J|", "L|-JF"];

        assert_eq!(4, solve_1(sample));

        let sample = vec!["7-F7-", ".FJ|7", "SJLL7", "|F--J", "LJ.LJ"];

        assert_eq!(8, solve_1(sample));
    }

    #[test]
    fn day_10_part_01_solution() {
        let input = include_str!("../../inputs/day_10.txt").lines().collect();

        assert_eq!(7_107, solve_1(input));
    }

    #[test]
    fn day_10_part_02_sample() {
        let sample = vec![
            "...........",
            ".S-------7.",
            ".|F-----7|.",
            ".||.....||.",
            ".||.....||.",
            ".|L-7.F-J|.",
            ".|..|.|..|.",
            ".L--J.L--J.",
            "...........",
        ];

        assert_eq!(4, solve_2(sample));

        let sample = vec![
            "..........",
            ".S------7.",
            ".|F----7|.",
            ".||....||.",
            ".||....||.",
            ".|L-7F-J|.",
            ".|..||..|.",
            ".L--JL--J.",
            "..........",
        ];

        assert_eq!(4, solve_2(sample));

        let sample = vec![
            ".F----7F7F7F7F-7....",
            ".|F--7||||||||FJ....",
            ".||.FJ||||||||L7....",
            "FJL7L7LJLJ||LJ.L-7..",
            "L--J.L7...LJS7F-7L7.",
            "....F-J..F7FJ|L7L7L7",
            "....L7.F7||L7|.L7L7|",
            ".....|FJLJ|FJ|F7|.LJ",
            "....FJL-7.||.||||...",
            "....L---J.LJ.LJLJ...",
        ];

        assert_eq!(8, solve_2(sample));

        let sample = vec![
            "FF7FSF7F7F7F7F7F---7",
            "L|LJ||||||||||||F--J",
            "FL-7LJLJ||||||LJL-77",
            "F--JF--7||LJLJ7F7FJ-",
            "L---JF-JLJ.||-FJLJJ7",
            "|F|F-JF---7F7-L7L|7|",
            "|FFJF7L7F-JF7|JL---7",
            "7-L-JL7||F7|L7F-7F7|",
            "L.L7LFJ|||||FJL7||LJ",
            "L7JLJL-JLJLJL--JLJ.L",
        ];

        assert_eq!(10, solve_2(sample));
    }

    #[test]
    fn day_10_part_02_solution() {
        let input = include_str!("../../inputs/day_10.txt").lines().collect();

        assert_eq!(281, solve_2(input));
    }
}
