use crate::solutions::day_09::Direction::{Future, Past};
use std::str::FromStr;

pub fn solve_1(histories: Vec<&str>) -> i64 {
    solve(histories, Future)
}

pub fn solve_2(histories: Vec<&str>) -> i64 {
    solve(histories, Past)
}

fn solve(histories: Vec<&str>, direction: Direction) -> i64 {
    let mut prediction = 0;

    for &history in histories.iter() {
        let mut history: Vec<i64> = history.split(' ').flat_map(i64::from_str).collect();
        let mut sequence = 0;

        prediction += match direction {
            Future => *history.last().unwrap(),
            Past => *history.first().unwrap() * sign(sequence),
        };

        while history.iter().any(|h| *h != 0) {
            history = (1..history.len())
                .map(|i| history[i] - history[i - 1])
                .collect();

            sequence += 1;
            prediction += match direction {
                Future => *history.last().unwrap(),
                Past => *history.first().unwrap() * sign(sequence),
            }
        }
    }

    prediction
}

enum Direction {
    Future,
    Past,
}

fn sign(iteration: usize) -> i64 {
    (-1i64).pow(iteration as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_09_part_01_sample() {
        let sample = vec!["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];

        assert_eq!(114, solve_1(sample));
    }

    #[test]
    fn day_09_part_01_solution() {
        let input = include_str!("../../inputs/day_09.txt").lines().collect();

        assert_eq!(1_921_197_370, solve_1(input));
    }

    #[test]
    fn day_09_part_02_sample() {
        let sample = vec!["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];

        assert_eq!(2, solve_2(sample));
    }

    #[test]
    fn day_09_part_02_solution() {
        let input = include_str!("../../inputs/day_09.txt").lines().collect();

        assert_eq!(1_124, solve_2(input));
    }
}
