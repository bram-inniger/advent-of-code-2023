// use regex::Regex;
// use std::str::FromStr;

pub fn solve_1(_records: Vec<&str>) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn day_12_part_01_sample() {
        let sample = vec![
            "???.### 1,1,3", // 1
            // ".??..??...?##. 1,1,3", // 4
            // "?#?#?#?#?#?#?#? 1,3,1,6", // 1
            // "????.#...#... 4,1,1", // 1
            // "????.######..#####. 1,6,5", // 4
            // "?###???????? 3,2,1", // 10
        ];

        assert_eq!(1, solve_1(sample));

        // TODO clean up above and uncomment this
        // let sample = vec![
        //     "???.### 1,1,3",
        //     ".??..??...?##. 1,1,3",
        //     "?#?#?#?#?#?#?#? 1,3,1,6",
        //     "????.#...#... 4,1,1",
        //     "????.######..#####. 1,6,5",
        //     "?###???????? 3,2,1",
        // ];
        //
        // assert_eq!(21, solve_1(sample));
    }

    #[ignore]
    #[test]
    fn day_12_part_01_solution() {
        let input = include_str!("../../inputs/day_12.txt").lines().collect();

        assert_eq!(0, solve_1(input));
    }
}
