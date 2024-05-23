use std::collections::{VecDeque};
use crate::solutions::day20::Module::{Broadcaster, Conjunction, FlipFlop};
use indexmap::{IndexMap};

#[derive(Debug, Clone)]
enum Module<'a> {
    FlipFlop(FlipFlopModule<'a>),
    Conjunction(ConjunctionModule<'a>),
    Broadcaster(BroadcasterModule<'a>),
}

#[derive(Debug, Clone)]
struct FlipFlopModule<'a> {
    state: bool,
    outputs: Vec<&'a str>
}

#[derive(Debug, Clone)]
struct ConjunctionModule<'a> {
    states: IndexMap<&'a str, bool>,
    outputs: Vec<&'a str>
}

#[derive(Debug, Clone)]
struct BroadcasterModule<'a> {
    outputs: Vec<&'a str>
}

#[derive(Debug, Clone)]
struct Pulse<'a>(bool, &'a str, &'a str);

fn parse(data: &str) -> IndexMap<&str, Module> {
    let mut parsed_data = data.lines().map(|line| {
        let (left, right) = line.split_once(" -> ").unwrap();
        let mod_type = left.chars().nth(0).unwrap();
        let name = if &left[1..] == "roadcaster" { "broadcaster" } else { &left[1..] };
        let outputs = right.split(", ").collect::<Vec<_>>();
        (name, match mod_type {
            '%' => FlipFlop(FlipFlopModule {
                state: false,
                outputs
            }),
            '&' => Conjunction(ConjunctionModule {
                states: IndexMap::new(),
                outputs
            }),
            'b' => Broadcaster(BroadcasterModule {
                outputs
            }),
            _ => panic!("Unknown module type")
        })
    }).collect::<IndexMap<_, _>>();


    for (name, module) in parsed_data.clone().iter() {
        let inputs = match module {
            FlipFlop(f) => &f.outputs,
            Conjunction(c) => &c.outputs,
            Broadcaster(b) => &b.outputs
        };
        for (name_mut, module_mut) in parsed_data.iter_mut() {
            if let Conjunction(c) = module_mut {
                if inputs.contains(name_mut) {
                    c.states.insert(name, false);
                }
            }
        }
    }
    
    parsed_data
}

pub fn part1(data: &str) -> i64 {
    let mut data = parse(data);
    let mut queue = VecDeque::new();
    let (mut lows, mut highs) = (0, 0);
    for _ in 0..1000 {
        lows += 1;
        let Broadcaster(b) = data.get("broadcaster").unwrap() else { panic!(); };
        for &output in b.outputs.iter() {
            queue.push_back(Pulse(false, "broadcaster", output));
        }
        while !queue.is_empty() {
            let Pulse(signal, source, target) = queue.pop_front().unwrap();
            if signal { highs += 1; } else { lows += 1; }
            if let Some(module) = data.get_mut(target) {
                match module {
                    FlipFlop(f) => {
                        if !signal {
                            f.state = !f.state;
                            for &output in &f.outputs {
                                queue.push_back(Pulse(f.state, target, output));
                            }
                        }
                    },
                    Conjunction(c) => {
                        if c.states.insert(source, signal) == None {
                            panic!("Unknown input on conjunction");
                        }
                        let send = !c.states.iter().all(|(_, state)| *state);
                        for &output in &c.outputs {
                            queue.push_back(Pulse(send, target, output));
                        }
                    },
                    _ => panic!()
                }
            }
        }
    }
    lows * highs
}

pub fn part2(data: &str) -> i64 {
    let mut data = parse(data);
    let mut queue = VecDeque::new();
    let mut i = 0;
    let mut loops = IndexMap::new();

    'outer: loop {
        i += 1;
        let Broadcaster(b) = data.get("broadcaster").unwrap() else { panic!(); };
        for &output in b.outputs.iter() {
            queue.push_back(Pulse(false, "broadcaster", output));
        }
        while !queue.is_empty() {
            let Pulse(signal, source, target) = queue.pop_front().unwrap();
            if let Some(module) = data.get_mut(target) {
                match module {
                    FlipFlop(f) => {
                        if !signal {
                            f.state = !f.state;
                            for &output in &f.outputs {
                                queue.push_back(Pulse(f.state, target, output));
                            }
                        }
                    },
                    Conjunction(c) => {
                        if c.states.insert(source, signal) == None {
                            panic!("Unknown input on conjunction");
                        }
                        let send = !c.states.iter().all(|(_, state)| *state);
                        for &output in &c.outputs {
                            queue.push_back(Pulse(send, target, output));
                        }
                    },
                    _ => panic!()
                }
            } else {
                if let Conjunction(c) = data.get(source).unwrap() {
                    if loops.is_empty() {
                        for (&input, _) in &c.states {
                            loops.insert(input, -1);
                        }
                    }
                    if let Some(elem) = c.states.iter().find(|s| *s.1) {
                        if *loops.get(elem.0).unwrap() == -1 {
                            loops.insert(elem.0, i);
                            if loops.iter().all(|(_, &x)| x > -1) {
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
    };

    let mut largest_common_multiple = 0;
    for (_, i) in loops {
        if largest_common_multiple == 0 {
            largest_common_multiple = i;
        } else {
            largest_common_multiple = num::Integer::lcm(&largest_common_multiple, &i);
        }
    }

    largest_common_multiple
}