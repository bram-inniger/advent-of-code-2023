use regex::Regex;
use std::str::FromStr;

pub fn solve_1(races: Vec<&str>) -> u64 {
    parse_races_bad_kerning(races)
        .iter()
        .map(|r| r.nr_wins())
        .product()
}

pub fn solve_2(races: Vec<&str>) -> u64 {
    parse_races_good_kerning(races)
        .iter()
        .map(|r| r.nr_wins_closed_form())
        .product()
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

    // The problem can be solved using a quadratic equation
    // The unknown (x) is how long to press
    // Knowns are the time available (time), and the distance to beat (dist)
    //
    // This creates the following equation, which we can simplify:
    // => x * (time - x) > dist
    // => -x^2 + x * time - dist > 0
    //
    // This gives us the following parameters to solve for the roots
    // => a = -1
    // => b = time
    // => c = -dist
    //
    // Finally the roots can be found at
    // => (-b +- sqrt(b * b - 4 * a * c)) / 2 * a
    //
    // The solution then becomes counting the number of elements in the range formed by the roots
    fn nr_wins_closed_form(&self) -> u64 {
        let a = -1.0;
        let b = self.time as f64;
        let c = -(self.distance as f64);

        let d = b * b - 4.0 * a * c;

        let root_1 = (-b + d.sqrt()) / -2.0;
        let root_2 = (-b - d.sqrt()) / -2.0;

        (root_2 - root_1) as u64
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
