use std::str::FromStr;
use itertools::Itertools;

const DELTAS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub fn solve_1(plan: &str) -> i64 {
    let mut prev = (0, 0);
    let mut current = (0, 0);
    let mut perimeter = 0;
    let mut triangles_sum = 0;

    for instr in plan.lines() {
        let split = instr.split(' ').collect_vec();
        let steps = i64::from_str(split[1]).unwrap();
        let direction = match split[0] {
            "U" => 0,
            "R" => 1,
            "D" => 2,
            "L" => 3,
            _ => unreachable!(),
        };
        let (d_x, d_y) = DELTAS[direction];

        current = (current.0 + d_x * steps, current.1 + d_y * steps);
        perimeter += steps;
        triangles_sum += (prev.1 + current.1) * (prev.0 - current.0);
        prev = current;
    }

    i64::abs(triangles_sum / 2) + perimeter / 2 + 1
}

pub fn solve_2(plan: &str) -> i64 {
    let mut prev = (0, 0);
    let mut current = (0, 0);
    let mut perimeter = 0;
    let mut triangles_sum = 0;

    for instr in plan.lines() {
        let steps = i64::from_str_radix(&instr[instr.len() - 7..instr.len() - 2], 16).unwrap();
        let direction = instr.as_bytes()[instr.len() - 2] - b'0';
        let (d_x, d_y) = DELTAS[direction as usize];

        current = (current.0 + d_x * steps, current.1 + d_y * steps);
        perimeter += steps;
        triangles_sum += (prev.1 + current.1) * (prev.0 - current.0);
        prev = current;
    }

    i64::abs(triangles_sum / 2) + perimeter / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_18_part_01_sample() {
        let sample = "R 6 (#70c710)\n\
            D 5 (#0dc571)\n\
            L 2 (#5713f0)\n\
            D 2 (#d2c081)\n\
            R 2 (#59c680)\n\
            D 2 (#411b91)\n\
            L 5 (#8ceee2)\n\
            U 2 (#caa173)\n\
            L 1 (#1b58a2)\n\
            U 2 (#caa171)\n\
            R 2 (#7807d2)\n\
            U 3 (#a77fa3)\n\
            L 2 (#015232)\n\
            U 2 (#7a21e3)";

        assert_eq!(62, solve_1(sample));
    }

    #[test]
    fn day_18_part_01_solution() {
        let input = include_str!("../../inputs/day_18.txt");

        assert_eq!(72_821, solve_1(input));
    }

    #[test]
    fn day_18_part_02_sample() {
        let sample = "R 6 (#70c710)\n\
            D 5 (#0dc571)\n\
            L 2 (#5713f0)\n\
            D 2 (#d2c081)\n\
            R 2 (#59c680)\n\
            D 2 (#411b91)\n\
            L 5 (#8ceee2)\n\
            U 2 (#caa173)\n\
            L 1 (#1b58a2)\n\
            U 2 (#caa171)\n\
            R 2 (#7807d2)\n\
            U 3 (#a77fa3)\n\
            L 2 (#015232)\n\
            U 2 (#7a21e3)";

        assert_eq!(952_408_144_115, solve_2(sample));
    }

    #[test]
    fn day_18_part_02_solution() {
        let input = include_str!("../../inputs/day_18.txt");

        assert_eq!(127_844_509_405_501, solve_2(input));
    }
}
