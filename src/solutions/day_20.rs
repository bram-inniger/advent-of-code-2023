use std::collections::VecDeque;
use std::ops::Not;

use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::util::lcm;

pub fn solve_1(modules: &[&str]) -> u64 {
    let mut modules = parse_modules(modules);
    let mut results = Vec::new();

    for _ in 0..1000 {
        let result = push_button(&mut modules, "");
        results.push(result)
    }

    results.iter().map(|r| r.high).sum::<u64>() * results.iter().map(|r| r.low).sum::<u64>()
}

/// Inspecting the input file shows the "rx" module receives pulses through "ql".
/// "ql" is a conjunction which will emit a low pulse if and only if its 4 inputs: ["fh", "mf", "fz", "ss"],
/// are all high.
///
/// Printing the graph of inputs using [GraphViz](https://graphviz.org) shows 4 independent loops,
/// ending in the inputs above.
///
/// Re-applying the solution of Day 8, and applying LCM on the cycle lengths of these loop yields the solution.
///
/// The graph source can be found [here](../../problems/other/day_20.dot)
/// The rendered graph can be found [here](../../problems/other/day_20.png)
///
/// The broadcast module and the 4 cycle start modules are shown in light blue.
/// The "rx" module, its source and the 4 modules finishing up the cycles are shown in light green.
pub fn solve_2(modules: &[&str]) -> u64 {
    ["fh", "mf", "fz", "ss"]
        .iter()
        .map(|&m| find_cycle(&mut parse_modules(modules), m))
        .fold(1, lcm)
}

fn find_cycle(modules: &mut FxHashMap<&str, Module>, cycle_module: &str) -> u64 {
    let mut cycle_length = 0;
    let mut result = PulseResult {
        high: 0,
        low: 0,
        cycle: false,
    };

    while result.cycle.not() {
        result = push_button(modules, cycle_module);
        cycle_length += 1;
    }

    cycle_length
}

fn push_button(modules: &mut FxHashMap<&str, Module>, cycle_module: &str) -> PulseResult {
    let mut result = PulseResult {
        high: 0,
        low: 0,
        cycle: false,
    };
    let mut queue = VecDeque::new();
    queue.push_back(Signal {
        source: "button",
        destination: "broadcaster",
        pulse: Pulse::Low,
    });

    while let Some(signal) = queue.pop_front() {
        if signal.destination == "ql"
            && signal.source == cycle_module
            && matches!(signal.pulse, Pulse::High)
        {
            result.cycle = true;
            return result;
        }

        match signal.pulse {
            Pulse::High => result.high += 1,
            Pulse::Low => result.low += 1,
        }

        let module = match modules.get_mut(signal.destination) {
            None => continue,
            Some(m) => m,
        };
        let source = module.label();

        match module {
            Module::FlipFlop {
                label: _,
                ref mut state,
                destinations,
            } => match signal.pulse {
                Pulse::High => {}
                Pulse::Low => {
                    destinations.iter().for_each(|&d| {
                        queue.push_back(Signal {
                            source,
                            destination: d,
                            pulse: match state {
                                State::On => Pulse::Low,
                                State::Off => Pulse::High,
                            },
                        })
                    });
                    match state {
                        State::On => *state = State::Off,
                        State::Off => *state = State::On,
                    };
                }
            },
            Module::Conjunction {
                label: _,
                ref mut state,
                destinations,
            } => {
                state.insert(signal.source, signal.pulse);

                let pulse = if state.values().all(|p| matches!(p, Pulse::High)) {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                destinations.iter().for_each(|&d| {
                    queue.push_back(Signal {
                        source,
                        destination: d,
                        pulse,
                    })
                });
            }
            Module::Broadcast {
                label: _,
                destinations,
            } => destinations.iter().for_each(|&d| {
                queue.push_back(Signal {
                    source,
                    destination: d,
                    pulse: signal.pulse,
                });
            }),
        }
    }

    result
}

fn parse_modules<'a>(modules: &[&'a str]) -> FxHashMap<&'a str, Module<'a>> {
    // Read the modules from the input line by line
    let mut modules: FxHashMap<&str, Module> = modules
        .iter()
        .map(|&m| Module::new(m))
        .map(|m| {
            (
                match m {
                    Module::FlipFlop { label, .. }
                    | Module::Conjunction { label, .. }
                    | Module::Broadcast { label, .. } => label,
                },
                m,
            )
        })
        .collect();

    // Now "wire" together the modules having a conjunction as their destination, to that conjunction
    let conjunctions = modules
        .values()
        .filter(|m| matches!(m, Module::Conjunction { .. }))
        .map(|m| m.label())
        .collect_vec();

    for conjunction in conjunctions {
        let incoming_modules = modules
            .values()
            .filter(|&m| m.destinations().iter().any(|&d| d == conjunction))
            .map(|m| m.label())
            .collect_vec();
        let state = modules
            .get_mut(conjunction)
            .unwrap()
            .try_conjunction_state();

        incoming_modules.iter().for_each(|&i| {
            state.insert(i, Pulse::Low);
        });
    }

    modules
}

#[derive(Debug)]
enum Module<'a> {
    FlipFlop {
        label: &'a str,
        state: State,
        destinations: Vec<&'a str>,
    },
    Conjunction {
        label: &'a str,
        state: FxHashMap<&'a str, Pulse>,
        destinations: Vec<&'a str>,
    },
    Broadcast {
        label: &'a str,
        destinations: Vec<&'a str>,
    },
}

impl<'a> Module<'a> {
    fn new(module: &'a str) -> Module<'a> {
        let split = module.split(" -> ").collect_vec();
        let destinations = split[1].split(", ").collect_vec();

        match &split[0][..1] {
            "%" => Module::FlipFlop {
                label: &split[0][1..],
                state: State::Off,
                destinations,
            },
            "&" => Module::Conjunction {
                label: &split[0][1..],
                state: Default::default(),
                destinations,
            },
            "b" => Module::Broadcast {
                label: "broadcaster",
                destinations,
            },
            _ => unreachable!(),
        }
    }

    fn label(&self) -> &'a str {
        match self {
            Module::FlipFlop { label, .. } => label,
            Module::Conjunction { label, .. } => label,
            Module::Broadcast { label, .. } => label,
        }
    }

    fn destinations(&self) -> &Vec<&'a str> {
        match self {
            Module::FlipFlop { destinations, .. } => destinations,
            Module::Conjunction { destinations, .. } => destinations,
            Module::Broadcast { destinations, .. } => destinations,
        }
    }

    fn try_conjunction_state(&mut self) -> &mut FxHashMap<&'a str, Pulse> {
        match self {
            Module::Conjunction { state, .. } => state,
            _ => panic!("Trying to get conjunction state from a non-conjunction module"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum State {
    On,
    Off,
}

#[derive(Debug, Copy, Clone)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
struct PulseResult {
    high: u64,
    low: u64,
    cycle: bool,
}

#[derive(Debug)]
struct Signal<'a> {
    source: &'a str,
    destination: &'a str,
    pulse: Pulse,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_20_part_01_sample() {
        let sample = vec![
            "broadcaster -> a, b, c",
            "%a -> b",
            "%b -> c",
            "%c -> inv",
            "&inv -> a",
        ];

        assert_eq!(32_000_000, solve_1(&sample));

        let sample = vec![
            "broadcaster -> a",
            "%a -> inv, con",
            "&inv -> b",
            "%b -> con",
            "&con -> output",
        ];

        assert_eq!(11_687_500, solve_1(&sample));
    }

    #[test]
    fn day_20_part_01_solution() {
        let input = include_str!("../../inputs/day_20.txt")
            .lines()
            .collect_vec();

        assert_eq!(787_056_720, solve_1(&input));
    }

    #[test]
    fn day_20_part_02_sample() {
        // No sample input(s) for part 2
    }

    #[test]
    fn day_20_part_02_solution() {
        let input = include_str!("../../inputs/day_20.txt")
            .lines()
            .collect_vec();

        assert_eq!(212_986_464_842_911, solve_2(&input));
    }
}
