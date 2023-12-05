use itertools::Itertools;
use std::collections::HashSet;
use std::ops::Range;
use std::str::FromStr;

pub fn solve_1(almanac: &str) -> u64 {
    let almanac = Almanac::new(almanac);

    almanac
        .seeds
        .iter()
        .map(|e| almanac.map_to(e))
        .min()
        .unwrap()
}

pub fn solve_2(almanac: &str) -> u64 {
    let almanac = Almanac::new(almanac);
    let mut ranges = almanac.seeds_as_ranges();

    for category in almanac.categories {
        ranges = category.convert(&ranges)
    }

    ranges.iter().map(|r| r.start).min().unwrap()
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    categories: Vec<Category>,
}

#[derive(Debug)]
struct Category {
    _name: String,
    mappings: Vec<Mapping>,
}

#[derive(Debug)]
struct Mapping {
    source: Range<u64>,
    destination: Range<u64>,
}

impl Almanac {
    fn new(almanac: &str) -> Almanac {
        let split: Vec<&str> = almanac.split("\n\n").collect();

        let seeds: Vec<u64> = split[0]
            .trim_start_matches("seeds: ")
            .split(' ')
            .flat_map(u64::from_str)
            .collect();
        let categories: Vec<Category> = split.iter().skip(1).map(|&s| Category::new(s)).collect();

        Almanac { seeds, categories }
    }

    fn map_to(&self, element: &u64) -> u64 {
        let mut location = *element;

        for category in &self.categories {
            location = category.map_to(&location)
        }

        location
    }

    fn seeds_as_ranges(&self) -> HashSet<Range<u64>> {
        self.seeds
            .chunks(2)
            .map(|c| {
                let (seed, range) = (c[0], c[1]);
                seed..seed + range
            })
            .collect()
    }
}

impl Category {
    fn new(category: &str) -> Category {
        let split: Vec<&str> = category.split('\n').collect();

        let _name = split[0].trim_end_matches(" map:").to_string();
        let mappings: Vec<Mapping> = split
            .iter()
            .skip(1)
            .map(|&s| Mapping::new(s))
            .sorted_by_key(|m| m.source.start)
            .collect();

        Category { _name, mappings }
    }

    fn convert(&self, ranges: &HashSet<Range<u64>>) -> HashSet<Range<u64>> {
        ranges
            .iter()
            .flat_map(|r| Self::convert_single(self, r))
            .collect()
    }

    fn convert_single(&self, range: &Range<u64>) -> HashSet<Range<u64>> {
        let mut converted_ranges = HashSet::new();
        let mut current = range.start;

        for mapping in &self.mappings {
            let s_start = mapping.source.start;
            let s_end = mapping.source.end;
            let d_start = mapping.destination.start;

            // We're done once we start seeing mapping ranges beyond our own, so stop
            if range.end <= s_start {
                break;
            }

            // We're not interested in this mapping if its values are before our own range, so skip
            if current >= s_end {
                continue;
            }

            // This means (at least the start of) our range is unmapped and should be recorded
            // A check is needed to see which comes first, our range's end, or the mapping
            if current < s_start && range.end > s_start {
                converted_ranges.insert(current..s_start);
                current = s_start
            } else if current < s_start && range.end <= s_start {
                converted_ranges.insert(current..range.end);
                break; // We fully mapped out our range
            }

            // At this point we are sure at least some part of our current range has a mapping
            if range.end <= s_end {
                converted_ranges
                    .insert((current - s_start + d_start)..(range.end - s_start + d_start));
                break; // We fully mapped out our range
            } else {
                converted_ranges.insert((current - s_start + d_start)..(s_end - s_start + d_start));
                current = s_end;
            }
        }

        // Final case to account for is our range going beyond the last mapping's end
        let last_mapping_end = self.mappings.last().unwrap().source.end;
        if range.end > last_mapping_end {
            converted_ranges.insert(last_mapping_end..range.end);
        }

        converted_ranges
    }

    fn map_to(&self, element: &u64) -> u64 {
        self.mappings
            .iter()
            .find(|&m| m.contains(element))
            .map(|m| m.map_to(element))
            .unwrap_or(*element)
    }
}

impl Mapping {
    fn new(mapping: &str) -> Mapping {
        let split: Vec<u64> = mapping.split(' ').flat_map(u64::from_str).collect();

        let destination_start = split[0];
        let source_start = split[1];
        let range_length = split[2];

        Mapping {
            source: source_start..source_start + range_length,
            destination: destination_start..destination_start + range_length,
        }
    }

    fn contains(&self, element: &u64) -> bool {
        self.source.contains(element)
    }

    fn map_to(&self, element: &u64) -> u64 {
        element - self.source.start + self.destination.start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_05_part_01_sample() {
        let sample = "seeds: 79 14 55 13\n\
            \n\
            seed-to-soil map:\n\
            50 98 2\n\
            52 50 48\n\
            \n\
            soil-to-fertilizer map:\n\
            0 15 37\n\
            37 52 2\n\
            39 0 15\n\
            \n\
            fertilizer-to-water map:\n\
            49 53 8\n\
            0 11 42\n\
            42 0 7\n\
            57 7 4\n\
            \n\
            water-to-light map:\n\
            88 18 7\n\
            18 25 70\n\
            \n\
            light-to-temperature map:\n\
            45 77 23\n\
            81 45 19\n\
            68 64 13\n\
            \n\
            temperature-to-humidity map:\n\
            0 69 1\n\
            1 0 69\n\
            \n\
            humidity-to-location map:\n\
            60 56 37\n\
            56 93 4";

        assert_eq!(35, solve_1(sample));
    }

    #[test]
    fn day_05_part_01_solution() {
        let input = include_str!("../../inputs/day_05.txt");

        assert_eq!(199_602_917, solve_1(input));
    }

    #[test]
    fn day_05_part_02_sample() {
        let sample = "seeds: 79 14 55 13\n\
            \n\
            seed-to-soil map:\n\
            50 98 2\n\
            52 50 48\n\
            \n\
            soil-to-fertilizer map:\n\
            0 15 37\n\
            37 52 2\n\
            39 0 15\n\
            \n\
            fertilizer-to-water map:\n\
            49 53 8\n\
            0 11 42\n\
            42 0 7\n\
            57 7 4\n\
            \n\
            water-to-light map:\n\
            88 18 7\n\
            18 25 70\n\
            \n\
            light-to-temperature map:\n\
            45 77 23\n\
            81 45 19\n\
            68 64 13\n\
            \n\
            temperature-to-humidity map:\n\
            0 69 1\n\
            1 0 69\n\
            \n\
            humidity-to-location map:\n\
            60 56 37\n\
            56 93 4";

        assert_eq!(46, solve_2(sample));
    }

    #[test]
    fn day_05_part_02_solution() {
        let input = include_str!("../../inputs/day_05.txt");

        assert_eq!(2_254_686, solve_2(input));
    }
}
