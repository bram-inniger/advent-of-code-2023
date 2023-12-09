use std::str::FromStr;

pub fn solve_1(histories: Vec<&str>) -> i32 {
    solve(histories, Direction::Future)
}

pub fn solve_2(histories: Vec<&str>) -> i32 {
    solve(histories, Direction::Past)
}

fn solve(histories: Vec<&str>, direction: Direction) -> i32 {
    histories
        .iter()
        .map(|h| h.split(' ').flat_map(i32::from_str).collect())
        .map(|h| predict(h, 0, &direction, 0))
        .sum()
}

fn predict(history: Vec<i32>, sequence: u32, direction: &Direction, prediction: i32) -> i32 {
    if history.iter().all(|&h| h == 0) {
        prediction
    } else {
        let prediction = prediction
            + match direction {
            Direction::Future => *history.last().unwrap(),
            Direction::Past => *history.first().unwrap() * sign(sequence),
            };
        let sequence = sequence + 1;
        let history: Vec<i32> = (1..history.len())
            .map(|i| history[i] - history[i - 1])
            .collect();

        predict(history, sequence, direction, prediction)
    }
}

enum Direction {
    Future,
    Past,
}

fn sign(sequence: u32) -> i32 {
    (-1i32).pow(sequence)
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
