use std::collections::{HashMap, VecDeque};
use aoc::*;
use regex::Regex;

type Modules = HashMap<String, Module>;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Module {
    FlipFlop(bool, Vec<String>),
    Conjunction(HashMap<String, bool>, Vec<String>),
    Broadcaster(Vec<String>),
}
use Module::*;

impl Module {
    fn handle_msg(&mut self, src: &String, pulse: bool) -> Vec<(String, bool)> {
        let mut new_msgs = Vec::<(String, bool)>::new();
        match self {
            FlipFlop(state, outputs) => {
                match pulse {
                    true => {
                       ()
                    }
                    false => {
                        *state = !*state;
                        for output in outputs.iter() {
                            new_msgs.push((output.clone(), *state));
                        }
                    }
                }
            }
            Conjunction(states, outputs) => {
                states.insert(src.clone(), pulse);
                let to_send = !states.values().cloned().all(identity);
                for output in outputs.iter() {
                    new_msgs.push((output.clone(), to_send));
                }
            }
            Broadcaster(outputs) => {
                for output in outputs.iter() {
                    new_msgs.push((output.clone(), pulse));
                }
            }
        }
        new_msgs
    }
}

fn first(mut modules: Modules) -> usize {
    let mut total_lows = 0;
    let mut total_highs = 0;
    for _ in 0..1000 {
        let (new_lows, new_highs) = cycle(&mut HashMap::new(), 0, &mut modules);
        total_lows += new_lows;
        total_highs += new_highs;
    }
    total_highs * total_lows
}

fn cycle(watchdog: &mut HashMap<String, usize>, i: usize, modules: &mut Modules) -> (usize, usize) {
    let mut messages: VecDeque<(String, String, bool)> = VecDeque::new();
    messages.push_back((String::from("button"), String::from("broadcaster"), false));
    let mut lows = 0;
    let mut highs = 0;
    loop {
        let Some((src, dst, pulse)) = messages.pop_front() else {
            return (lows, highs);
        };
        if watchdog.contains_key(&src) && pulse {
            watchdog.insert(src.clone(), i);
        }
        if pulse {
            highs += 1;
        }
        else {
            lows += 1;
        }
        match modules.get_mut(&dst) {
            Some(module) => {
                messages.extend(module.handle_msg(&src, pulse).into_iter().map(|(new_dst, new_pulse)| (dst.clone(), new_dst, new_pulse)));
            }
            None => {
                ()
            }
        }
    }
}

fn second(mut modules: Modules) -> usize {
    let rx = String::from("rx");
    let mut watchdog = modules.iter().find_map(|(_, module)| match module {
        Conjunction(states, outputs) if outputs.contains(&rx) => {
            Some(states.keys().cloned().map(|key| (key, 0usize)).collect::<HashMap<_, _>>())
        }
        _ => {
            None
        }
    }).expect("wtf");
    let mut i = 0;
    loop {
        i += 1;
        cycle(&mut watchdog, i, &mut modules);
        if watchdog.values().all(|&n| n > 0) {
            return watchdog.values().fold(1, |acc, &n| lcm(acc, n));
        }
    }
}

#[lines]
fn parse(ls: Vec<String>) -> Modules {
    let re = Regex::new(r"^(%|&|)([a-z]+) -> ([a-z ,]+)$").expect("wtf regex");
    let mut modules: Modules = ls.iter().filter_map(|line| re.captures(line)).map(|captures| {
        let outputs = captures[3].split(", ").map(String::from).collect_vec();
        let module = match &captures[1] {
            "%" => {
                FlipFlop(false, outputs)
            }
            "&" => {
                Conjunction(HashMap::new(), outputs)
            }
            "" => {
                Broadcaster(outputs)
            }
            _ => {
                panic!("unknown module type");
            }
        };
        (String::from(&captures[2]), module)
    }).collect();
    let outputs: HashMap<String, Vec<String>> = modules.clone().into_iter().map(|(name, module)| (name, match module {
        FlipFlop(_, outputs) => outputs,
        Conjunction(_, outputs) => outputs,
        Broadcaster(outputs) => outputs,
    })).collect();
    modules.iter_mut().for_each(|(name, module)| {
        match module {
            Conjunction(inputs, _) => {
                for subname in outputs.iter().filter(|(_, outs)| outs.contains(name)).map(|(subname, _)| subname) {
                    inputs.insert(subname.clone(), false);
                }
            }
            _ => {
                ()
            }
        }
    });
    modules
}

aoc!(parse);

const EXAMPLE: &str = "
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";