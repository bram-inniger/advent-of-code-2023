use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::ops::Not;

pub fn solve_1(modules: Vec<&str>) -> u64 {
    let mut results = Vec::new();
    let mut modules = modules
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
    wire_conjunction_inputs(&mut modules);

    for _ in 0..1000 {
        let result = push_button(&mut modules);
        results.push(result)
    }

    results.iter().map(|r| r.high).sum::<u64>() * results.iter().map(|r| r.low).sum::<u64>()
}

fn wire_conjunction_inputs(modules: &mut HashMap<&str, Module>) {
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
}

fn push_button(modules: &mut HashMap<&str, Module>) -> PulseResult {
    let mut result = PulseResult { high: 0, low: 0 };
    let mut queue = VecDeque::new();
    queue.push_back(Signal {
        source: "button",
        destination: "broadcaster",
        pulse: Pulse::Low,
    });

    while queue.is_empty().not() {
        let signal = queue.pop_front().unwrap();

        match signal.pulse {
            Pulse::High => result.high += 1,
            Pulse::Low => result.low += 1,
        }

        let module = match modules.get_mut(signal.destination) {
            None => continue,
            Some(m) => m
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

#[derive(Debug)]
enum Module<'a> {
    FlipFlop {
        label: &'a str,
        state: State,
        destinations: Vec<&'a str>,
    },
    Conjunction {
        label: &'a str,
        state: HashMap<&'a str, Pulse>,
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
            "b" => Module::Broadcast { label: "broadcaster", destinations },
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

    fn try_conjunction_state(&mut self) -> &mut HashMap<&'a str, Pulse> {
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

        assert_eq!(32_000_000, solve_1(sample));

        let sample = vec![
            "broadcaster -> a",
            "%a -> inv, con",
            "&inv -> b",
            "%b -> con",
            "&con -> output",
        ];

        assert_eq!(11_687_500, solve_1(sample));
    }

    #[test]
    fn day_20_part_01_solution() {
        let input = include_str!("../../inputs/day_20.txt").lines().collect();

        assert_eq!(787_056_720, solve_1(input));
    }
}
