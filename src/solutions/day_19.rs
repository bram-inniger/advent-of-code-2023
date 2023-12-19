use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub fn solve_1(list: &str) -> u32 {
    let split = list.split("\n\n").collect_vec();

    let workflows = split[0]
        .split('\n')
        .map(Workflow::new)
        .map(|w| (w.name, w))
        .collect();
    let parts = split[1].split('\n').map(Part::new).collect_vec();

    resolve_system(parts, workflows)
}

fn resolve_system<'a>(parts: Vec<Part>, workflows: HashMap<&'a str, Workflow<'a>>) -> u32 {
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
        let re = Regex::new(r"^(?<name>\w+)\{(?<rules>.+)}$").unwrap();
        let caps = re.captures(workflow).unwrap();

        let name = caps.get(1).unwrap().as_str();
        let rules = caps.get(2).unwrap().as_str();
        let rules = rules.split(',').map(Rule::new).collect_vec();

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
        let re = Regex::new(r"^(?<category>[xmas])(?<sign>[<>])(?<value>\d+):(?<destination>\w+)$")
            .unwrap();
        match re.captures(rule) {
            None => Rule::Unconditional { destination: rule },
            Some(caps) => {
                let category = &caps["category"];
                let sign = &caps["sign"];
                let value = u32::from_str(&caps["value"]).unwrap();
                let destination = caps.get(4).unwrap().as_str();

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

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn new(part: &str) -> Part {
        let re = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}$").unwrap();
        let caps = re.captures(part).unwrap();

        Part {
            x: u32::from_str(&caps[1]).unwrap(),
            m: u32::from_str(&caps[2]).unwrap(),
            a: u32::from_str(&caps[3]).unwrap(),
            s: u32::from_str(&caps[4]).unwrap(),
        }
    }

    fn rating_sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
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

    #[ignore = "slow, need to increase performance"]
    #[test]
    fn day_19_part_01_solution() {
        let input = include_str!("../../inputs/day_19.txt");

        assert_eq!(492_702, solve_1(input));
    }
}
