use std::str::FromStr;

pub fn solve_1(histories: Vec<&str>) -> i64 {
    let mut sum = 0;
    for history in &histories {
        let mut history: Vec<i64> = history.split(' ').flat_map(i64::from_str).collect();
        sum += history.last().unwrap();

        while history.iter().any(|h| *h != 0) {
            history = (1..history.len()).map(|i| history[i] - history[i-1]).collect();
            sum += history.last().unwrap();
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_09_part_01_sample() {
        let sample = vec![
            "0 3 6 9 12 15",
            "1 3 6 10 15 21",
            "10 13 16 21 30 45",
        ];

        assert_eq!(114, solve_1(sample));
    }

    #[test]
    fn day_09_part_01_solution() {
        let input = include_str!("../../inputs/day_09.txt").lines().collect();

        assert_eq!(1_921_197_370, solve_1(input));
    }
}
