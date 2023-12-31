use std::collections::VecDeque;
use std::str::FromStr;

use itertools::Itertools;
use rustc_hash::FxHashMap;

pub fn solve_1(list: &str) -> u64 {
    let (workflows, parts) = parse_list(list);
    let a_ranges = find_a_ranges(workflows);

    parts
        .iter()
        .filter(|&p| a_ranges.iter().any(|r| r.contains(p)))
        .map(|p| p.rating_sum())
        .sum()
}

pub fn solve_2(list: &str) -> u64 {
    let (workflows, _) = parse_list(list);

    find_a_ranges(workflows)
        .iter()
        .map(|r| r.combinations())
        .sum()
}

fn parse_list(list: &str) -> (FxHashMap<&str, Workflow>, Vec<Part>) {
    let split = list.split("\n\n").collect_vec();

    let workflows = split[0]
        .split('\n')
        .map(Workflow::new)
        .map(|w| (w.name, w))
        .collect();
    let parts = split[1].split('\n').map(Part::new).collect_vec();

    (workflows, parts)
}

fn find_a_ranges(workflows: FxHashMap<&str, Workflow>) -> Vec<RatingsRange> {
    let rd_start = RangeDestination {
        r_range: RatingsRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
        dest: "in",
    };

    let mut a_ranges = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(rd_start);

    while let Some(rd) = queue.pop_front() {
        match rd.dest {
            "A" => a_ranges.push(rd.r_range),
            "R" => {}
            _ => resolve_workflow(&rd, &workflows)
                .iter()
                .for_each(|&rd_n| queue.push_back(rd_n)),
        }
    }

    a_ranges
}

fn resolve_workflow<'a>(
    rd: &RangeDestination<'a>,
    workflows: &FxHashMap<&'a str, Workflow<'a>>,
) -> Vec<RangeDestination<'a>> {
    let workflow = &workflows[rd.dest];
    let mut rds = Vec::new();
    let mut r_range = rd.r_range;

    for rule in &workflow.rules {
        match rule {
            Rule::Conditional {
                category,
                sign,
                value,
                dest,
            } => {
                let mut r_range_split = r_range;
                let range = borrow_range(category, &mut r_range);
                let split_range = borrow_range(category, &mut r_range_split);

                match sign {
                    Sign::Gt => {
                        if range.1 > *value {
                            if range.0 > *value {
                                rds.push(RangeDestination { r_range, dest });
                                break;
                            } else {
                                split_range.0 = *value + 1;
                                rds.push(RangeDestination {
                                    r_range: r_range_split,
                                    dest,
                                });
                                range.1 = *value;
                            }
                        };
                    }
                    Sign::Lt => {
                        if range.0 < *value {
                            if range.1 < *value {
                                rds.push(RangeDestination { r_range, dest });
                                break;
                            } else {
                                split_range.1 = *value - 1;
                                rds.push(RangeDestination {
                                    r_range: r_range_split,
                                    dest,
                                });
                                range.0 = *value;
                            }
                        };
                    }
                }
            }
            Rule::Unconditional { destination } => {
                rds.push(RangeDestination {
                    r_range,
                    dest: destination,
                });
                break;
            }
        }
    }

    rds
}

fn borrow_range<'a>(category: &Category, r_range: &'a mut RatingsRange) -> &'a mut (u64, u64) {
    match category {
        Category::X => &mut r_range.x,
        Category::M => &mut r_range.m,
        Category::A => &mut r_range.a,
        Category::S => &mut r_range.s,
    }
}

struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn new(workflow: &'a str) -> Workflow<'a> {
        let split = workflow.find('{').unwrap();

        let name = &workflow[..split];
        let rules = workflow[split + 1..workflow.len() - 1]
            .split(',')
            .map(Rule::new)
            .collect_vec();

        Workflow { name, rules }
    }
}

enum Rule<'a> {
    Conditional {
        category: Category,
        sign: Sign,
        value: u64,
        dest: &'a str,
    },
    Unconditional {
        destination: &'a str,
    },
}

impl<'a> Rule<'a> {
    fn new(rule: &'a str) -> Rule<'a> {
        let split = rule.find(':');
        match split {
            None => Rule::Unconditional { destination: rule },
            Some(idx) => {
                let category = match &rule[..1] {
                    "x" => Category::X,
                    "m" => Category::M,
                    "a" => Category::A,
                    "s" => Category::S,
                    _ => unreachable!(),
                };
                let sign = match &rule[1..2] {
                    ">" => Sign::Gt,
                    "<" => Sign::Lt,
                    _ => unreachable!(),
                };
                let value = u64::from_str(&rule[2..idx]).unwrap();
                let destination = &rule[idx + 1..];

                Rule::Conditional {
                    category,
                    sign,
                    value,
                    dest: destination,
                }
            }
        }
    }
}

enum Category {
    X,
    M,
    A,
    S,
}

enum Sign {
    Gt,
    Lt,
}

struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn new(part: &str) -> Part {
        let split = &part[1..part.len() - 1].split(',').collect_vec();

        Part {
            x: u64::from_str(&split[0][2..]).unwrap(),
            m: u64::from_str(&split[1][2..]).unwrap(),
            a: u64::from_str(&split[2][2..]).unwrap(),
            s: u64::from_str(&split[3][2..]).unwrap(),
        }
    }

    fn rating_sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Copy, Clone)]
struct RatingsRange {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl RatingsRange {
    fn contains(&self, part: &Part) -> bool {
        part.x >= self.x.0
            && part.x <= self.x.1
            && part.m >= self.m.0
            && part.m <= self.m.1
            && part.a >= self.a.0
            && part.a <= self.a.1
            && part.s >= self.s.0
            && part.s <= self.s.1
    }
}

#[derive(Copy, Clone)]
struct RangeDestination<'a> {
    r_range: RatingsRange,
    dest: &'a str,
}

impl RatingsRange {
    fn combinations(&self) -> u64 {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_19_part_01_sample() {
        let sample = "px{a<2006:qkq,m>2090:A,rfg}\n\
            pv{a>1716:R,A}\n\
            lnx{m>1548:A,A}\n\
            rfg{s<537:gd,x>2440:R,A}\n\
            qs{s>3448:A,lnx}\n\
            qkq{x<1416:A,crn}\n\
            crn{x>2662:A,R}\n\
            in{s<1351:px,qqz}\n\
            qqz{s>2770:qs,m<1801:hdj,R}\n\
            gd{a>3333:R,R}\n\
            hdj{m>838:A,pv}\n\
            \n\
            {x=787,m=2655,a=1222,s=2876}\n\
            {x=1679,m=44,a=2067,s=496}\n\
            {x=2036,m=264,a=79,s=2244}\n\
            {x=2461,m=1339,a=466,s=291}\n\
            {x=2127,m=1623,a=2188,s=1013}";

        assert_eq!(19_114, solve_1(sample));
    }

    #[test]
    fn day_19_part_01_solution() {
        let input = include_str!("../../inputs/day_19.txt");

        assert_eq!(492_702, solve_1(input));
    }

    #[test]
    fn day_19_part_02_sample() {
        let sample = "px{a<2006:qkq,m>2090:A,rfg}\n\
            pv{a>1716:R,A}\n\
            lnx{m>1548:A,A}\n\
            rfg{s<537:gd,x>2440:R,A}\n\
            qs{s>3448:A,lnx}\n\
            qkq{x<1416:A,crn}\n\
            crn{x>2662:A,R}\n\
            in{s<1351:px,qqz}\n\
            qqz{s>2770:qs,m<1801:hdj,R}\n\
            gd{a>3333:R,R}\n\
            hdj{m>838:A,pv}\n\
            \n\
            {x=787,m=2655,a=1222,s=2876}\n\
            {x=1679,m=44,a=2067,s=496}\n\
            {x=2036,m=264,a=79,s=2244}\n\
            {x=2461,m=1339,a=466,s=291}\n\
            {x=2127,m=1623,a=2188,s=1013}";

        assert_eq!(167_409_079_868_000, solve_2(sample));
    }

    #[test]
    fn day_19_part_02_solution() {
        let input = include_str!("../../inputs/day_19.txt");

        assert_eq!(138_616_621_185_978, solve_2(input));
    }
}
