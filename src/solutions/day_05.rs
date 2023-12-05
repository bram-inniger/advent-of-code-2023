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

struct Almanac {
    seeds: Vec<u64>,
    categories: Vec<Category>,
}

struct Category {
    _name: String,
    mappings: Vec<Mapping>,
}

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
}

impl Category {
    fn new(category: &str) -> Category {
        let split: Vec<&str> = category.split('\n').collect();

        let _name = split[0].trim_end_matches(" map:").to_string();
        let mappings: Vec<Mapping> = split.iter().skip(1).map(|&s| Mapping::new(s)).collect();

        Category { _name, mappings }
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
}
