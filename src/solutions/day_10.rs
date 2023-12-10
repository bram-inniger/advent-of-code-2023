use std::collections::HashSet;
use std::ops::Index;

pub fn solve_1(maze: Vec<&str>) -> usize {
    Maze::new(maze).pipe_loop.len() / 2
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
}
