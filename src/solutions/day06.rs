use regex::Regex;
use std::str::FromStr;

pub fn solve_1(races: Vec<&str>) -> u32 {
    parse_races(races).iter().map(|r| r.nr_wins()).product()
}

fn parse_races(timings: Vec<&str>) -> Vec<Race> {
    let re = Regex::new(r"\d+").unwrap();

    let times: Vec<u32> = re
        .find_iter(timings[0])
        .map(|m| m.as_str())
        .flat_map(u32::from_str)
        .collect();
    let distances: Vec<u32> = re
        .find_iter(timings[1])
        .map(|m| m.as_str())
        .flat_map(u32::from_str)
        .collect();

    times
        .iter()
        .zip(distances)
        .map(|(&time, distance)| Race { time, distance })
        .collect()
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn nr_wins(&self) -> u32 {
        (1..self.time)
            .map(|speed| speed * (self.time - speed))
            .filter(|&distance| distance > self.distance)
            .count() as u32
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
}
