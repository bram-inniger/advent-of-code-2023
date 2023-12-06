use regex::Regex;
use std::str::FromStr;

pub fn solve_1(races: Vec<&str>) -> u64 {
    solve(parse_races_bad_kerning(races))
}

pub fn solve_2(races: Vec<&str>) -> u64 {
    solve(parse_races_good_kerning(races))
}

fn solve(races: Vec<Race>) -> u64 {
    races.iter().map(|r| r.nr_wins()).product()
}

fn parse_races_bad_kerning(timings: Vec<&str>) -> Vec<Race> {
    let re = Regex::new(r"\d+").unwrap();

    let times: Vec<u64> = re
        .find_iter(timings[0])
        .map(|m| m.as_str())
        .flat_map(u64::from_str)
        .collect();
    let distances: Vec<u64> = re
        .find_iter(timings[1])
        .map(|m| m.as_str())
        .flat_map(u64::from_str)
        .collect();

    times
        .iter()
        .zip(distances)
        .map(|(&time, distance)| Race { time, distance })
        .collect()
}

fn parse_races_good_kerning(timings: Vec<&str>) -> Vec<Race> {
    let re = Regex::new(r"\d+").unwrap();

    let time: String = re.find_iter(timings[0]).map(|m| m.as_str()).collect();
    let distance: String = re.find_iter(timings[1]).map(|m| m.as_str()).collect();

    vec![Race {
        time: u64::from_str(&time).unwrap(),
        distance: u64::from_str(&distance).unwrap(),
    }]
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn nr_wins(&self) -> u64 {
        (1..self.time)
            .map(|speed| speed * (self.time - speed))
            .filter(|&distance| distance > self.distance)
            .count() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_06_part_01_sample() {
        let sample = vec!["Time:      7  15   30", "Distance:  9  40  200"];

        assert_eq!(288, solve_1(sample));
    }

    #[test]
    fn day_06_part_01_solution() {
        let input = include_str!("../../inputs/day_06.txt").lines().collect();

        assert_eq!(160_816, solve_1(input));
    }

    #[test]
    fn day_06_part_02_sample() {
        let sample = vec!["Time:      7  15   30", "Distance:  9  40  200"];

        assert_eq!(71_503, solve_2(sample));
    }

    #[test]
    fn day_06_part_02_solution() {
        let input = include_str!("../../inputs/day_06.txt").lines().collect();

        assert_eq!(46_561_107, solve_2(input));
    }
}
