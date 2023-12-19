use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::ops::Not;
use std::str::FromStr;

pub fn solve_1(list: &str) -> u64 {
    let mut split = list.split("\n\n");

    let workflows = split
        .next()
        .unwrap()
        .split('\n')
        .map(Workflow::new)
        .map(|w| (w.name, w))
        .collect();
    let parts = split
        .next()
        .unwrap()
        .split('\n')
        .map(Part::new)
        .collect_vec();

    resolve_system(parts, workflows)
}

pub fn solve_2(list: &str) -> u64 {
    let workflows: HashMap<_, _> = list
        .split("\n\n")
        .next()
        .unwrap()
        .split('\n')
        .map(SimpleWorkflow::new)
        .map(|w| (w.name, w))
        .collect();
    let rd_start = RangeDestination {
        range: RatingsRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
        destination: "in",
    };

    let mut total_combinations = 0;
    let mut queue: VecDeque<RangeDestination> = VecDeque::new();
    queue.push_back(rd_start);

    while queue.is_empty().not() {
        let rd = queue.pop_front().unwrap();

        match rd.destination {
            "A" => total_combinations += rd.range.combinations(),
            "R" => {}
            _ => resolve_workflow(&rd, &workflows)
                .iter()
                .for_each(|&rd_n| queue.push_back(rd_n)),
        }
    }

    total_combinations
}

// TODO, properly reference / dereference the range tuple to combat code duplication
fn resolve_workflow<'a>(
    rd: &RangeDestination<'a>,
    workflows: &HashMap<&'a str, SimpleWorkflow<'a>>,
) -> Vec<RangeDestination<'a>> {
    let workflow = &workflows[rd.destination];
    let mut rds = Vec::new();
    let mut range = rd.range;

    for rule in &workflow.rules {
        match rule {
            SimpleRule::Conditional {
                category,
                sign,
                value,
                destination,
            } => match category {
                Category::X => match sign {
                    Sign::Gt => {
                        if &range.x.1 > value {
                            if &range.x.0 > value {
                                rds.push(RangeDestination { range, destination });
                                break;
                            } else {
                                let mut range_split = range;
                                range_split.x.0 = *value + 1;
                                rds.push(RangeDestination {
                                    range: range_split,
                                    destination,
                                });

                                range.x.1 = *value;
                            }
                        };
                    }
                    Sign::Lt => {
                        if &range.x.0 < value {
                            if &range.x.1 < value {
                                rds.push(RangeDestination { range, destination });
                                break;
                            } else {
                                let mut range_split = range;
                                range_split.x.1 = *value - 1;
                                rds.push(RangeDestination {
                                    range: range_split,
                                    destination,
                                });

                                range.x.0 = *value;
                            }
                        };
                    }
                },
                Category::M => match sign {
                    Sign::Gt => {
                        if &range.m.1 > value {
                            if &range.m.0 > value {
                                rds.push(RangeDestination { range, destination });
                                break;
                            } else {
                                let mut range_split = range;
                                range_split.m.0 = *value + 1;
                                rds.push(RangeDestination {
                                    range: range_split,
                                    destination,
                                });

                                range.m.1 = *value;
                            }
                        };
                    }
                    Sign::Lt => {
                        if &range.m.0 < value {
                            if &range.m.1 < value {
                                rds.push(RangeDestination { range, destination });
                                break;
                            } else {
                                let mut range_split = range;
                                range_split.m.1 = *value - 1;
                                rds.push(RangeDestination {
                                    range: range_split,
                                    destination,
                                });

                                range.m.0 = *value;
                            }
                        };
                    }
                },
                Category::A => match sign {
                    Sign::Gt => {
                        if &range.a.1 > value {
                            if &range.a.0 > value {
                                rds.push(RangeDestination { range, destination });
                                break;
                            } else {
                                let mut range_split = range;
                                range_split.a.0 = *value + 1;
                                rds.push(RangeDestination {
                                    range: range_split,
                                    destination,
                                });

                                range.a.1 = *value;
                            }
                        };
                    }
                    Sign::Lt => {
                        if &range.a.0 < value {
                            if &range.a.1 < value {
                                rds.push(RangeDestination { range, destination });
                                break;
                            } else {
                                let mut range_split = range;
                                range_split.a.1 = *value - 1;
                                rds.push(RangeDestination {
                                    range: range_split,
                                    destination,
                                });

                                range.a.0 = *value;
                            }
                        };
                    }
                },
                Category::S => match sign {
                    Sign::Gt => {
                        if &range.s.1 > value {
                            if &range.s.0 > value {
                                rds.push(RangeDestination { range, destination });
                                break;
                            } else {
                                let mut range_split = range;
                                range_split.s.0 = *value + 1;
                                rds.push(RangeDestination {
                                    range: range_split,
                                    destination,
                                });

                                range.s.1 = *value;
                            }
                        };
                    }
                    Sign::Lt => {
                        if &range.s.0 < value {
                            if &range.s.1 < value {
                                rds.push(RangeDestination { range, destination });
                                break;
                            } else {
                                let mut range_split = range;
                                range_split.s.1 = *value - 1;
                                rds.push(RangeDestination {
                                    range: range_split,
                                    destination,
                                });

                                range.s.0 = *value;
                            }
                        };
                    }
                },
            },
            SimpleRule::Unconditional { destination } => {
                rds.push(RangeDestination { range, destination });
                break;
            }
        }
    }

    rds
}

fn resolve_system<'a>(parts: Vec<Part>, workflows: HashMap<&'a str, Workflow<'a>>) -> u64 {
    let mut total_rating_sum = 0;

    for part in parts {
        let mut workflow = &workflows["in"];
        let mut cur_destination = "";
        loop {
            for rule in &workflow.rules {
                match rule {
                    Rule::Conditional {
                        destination,
                        predicate,
                    } => {
                        if predicate(&part) {
                            cur_destination = destination;
                            break;
                        }
                    }
                    Rule::Unconditional { destination } => {
                        cur_destination = destination;
                        break;
                    }
                }
            }

            match cur_destination {
                "A" => {
                    total_rating_sum += part.rating_sum();
                    break;
                }
                "R" => {
                    break;
                }
                _ => {
                    workflow = &workflows[cur_destination];
                }
            }
        }
    }

    total_rating_sum
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
        destination: &'a str,
        predicate: Box<dyn Fn(&Part) -> bool>,
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
                let category = &rule[..1];
                let sign = &rule[1..2];
                let value = u64::from_str(&rule[2..idx]).unwrap();
                let destination = &rule[idx + 1..];

                let predicate: Box<dyn Fn(&Part) -> bool> = match category {
                    "x" => match sign {
                        ">" => Box::new(move |p: &Part| p.x > value),
                        "<" => Box::new(move |p: &Part| p.x < value),
                        _ => unreachable!(),
                    },
                    "m" => match sign {
                        ">" => Box::new(move |p: &Part| p.m > value),
                        "<" => Box::new(move |p: &Part| p.m < value),
                        _ => unreachable!(),
                    },
                    "a" => match sign {
                        ">" => Box::new(move |p: &Part| p.a > value),
                        "<" => Box::new(move |p: &Part| p.a < value),
                        _ => unreachable!(),
                    },
                    "s" => match sign {
                        ">" => Box::new(move |p: &Part| p.s > value),
                        "<" => Box::new(move |p: &Part| p.s < value),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                };

                Rule::Conditional {
                    destination,
                    predicate,
                }
            }
        }
    }
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

struct SimpleWorkflow<'a> {
    name: &'a str,
    rules: Vec<SimpleRule<'a>>,
}

impl<'a> SimpleWorkflow<'a> {
    fn new(workflow: &'a str) -> SimpleWorkflow<'a> {
        let split = workflow.find('{').unwrap();

        let name = &workflow[..split];
        let rules = workflow[split + 1..workflow.len() - 1]
            .split(',')
            .map(SimpleRule::new)
            .collect_vec();

        SimpleWorkflow { name, rules }
    }
}

#[derive(Debug)]
enum SimpleRule<'a> {
    Conditional {
        category: Category,
        sign: Sign,
        value: u64,
        destination: &'a str,
    },
    Unconditional {
        destination: &'a str,
    },
}

impl<'a> SimpleRule<'a> {
    fn new(rule: &'a str) -> SimpleRule<'a> {
        let split = rule.find(':');
        match split {
            None => SimpleRule::Unconditional { destination: rule },
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

                SimpleRule::Conditional {
                    category,
                    sign,
                    value,
                    destination,
                }
            }
        }
    }
}

#[derive(Debug)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum Sign {
    Gt,
    Lt,
}

#[derive(Debug, Copy, Clone)]
struct RatingsRange {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

#[derive(Debug, Copy, Clone)]
struct RangeDestination<'a> {
    range: RatingsRange,
    destination: &'a str,
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
