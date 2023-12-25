use std::collections::VecDeque;
use std::ops::Not;

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

pub fn solve_1(diagram: &[&str], disconnects: &[(&str, &str)]) -> u32 {
    let diagram = Diagram::new(diagram, disconnects);
    diagram.group_len(disconnects[0].0) * diagram.group_len(disconnects[0].1)
}

pub fn solve_2() {
    // You supply all fifty stars and restart global snow production!
    //
    // As you reach the edge of Snow Island,
    // you can already tell from way up here that everyone will have a white Christmas this year after all.
}

#[derive(Debug)]
struct Diagram<'a> {
    wires: FxHashMap<&'a str, FxHashSet<&'a str>>,
}

impl<'a> Diagram<'a> {
    fn new(diagram: &[&'a str], disconnects: &[(&str, &str)]) -> Self {
        let mut wires: FxHashMap<&'a str, FxHashSet<&'a str>> = FxHashMap::default();

        diagram.iter().for_each(|&l| {
            let split = l.split(": ").collect_vec();

            let from = split[0];
            split[1]
                .split(' ')
                .filter(|&to| Self::should_connect(from, to, disconnects))
                .for_each(|to| {
                    wires.entry(from).or_default().insert(to);
                    wires.entry(to).or_default().insert(from);
                });
        });

        Self { wires }
    }

    fn should_connect(from: &str, to: &str, disconnects: &[(&str, &str)]) -> bool {
        disconnects
            .iter()
            .any(|d| (from == d.0 && to == d.1) || (from == d.1 && to == d.0))
            .not()
    }

    fn group_len(&self, component: &str) -> u32 {
        let mut len = 0;
        let mut seen = FxHashSet::default();
        let mut to_visit = VecDeque::new();
        to_visit.push_back(component);

        while let Some(next) = to_visit.pop_front() {
            if seen.contains(next) {
                continue;
            }

            seen.insert(next);
            len += 1;

            self.wires[next].iter().for_each(|&c| to_visit.push_back(c));
        }

        len
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_25_part_01_sample() {
        let sample = vec![
            "jqt: rhn xhk nvd",
            "rsh: frs pzl lsr",
            "xhk: hfx",
            "cmg: qnr nvd lhk bvb",
            "rhn: xhk bvb hfx",
            "bvb: xhk hfx",
            "pzl: lsr hfx nvd",
            "qnr: nvd",
            "ntq: jqt hfx bvb xhk",
            "nvd: lhk",
            "lsr: lhk",
            "rzs: qnr cmg lsr rsh",
            "frs: qnr lhk lsr",
        ];
        let sample_disconnects = vec![("hfx", "pzl"), ("bvb", "cmg"), ("nvd", "jqt")];

        assert_eq!(54, solve_1(&sample, &sample_disconnects));
    }

    #[test]
    fn day_25_part_01_solution() {
        let input = include_str!("../../inputs/day_25.txt")
            .lines()
            .collect_vec();
        let disconnects = vec![("fxr", "fzb"), ("vgk", "mbq"), ("nmv", "thl")];

        assert_eq!(600_369, solve_1(&input, &disconnects));
    }

    #[test]
    fn day_25_part_02_sample() {
        solve_2()
    }

    #[test]
    fn day_25_part_02_solution() {
        solve_2()
    }
}
