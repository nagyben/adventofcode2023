use std::collections::{HashMap, VecDeque};

use dyn_clone::{clone_trait_object, DynClone};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Clone)]
struct Pulse {
    source: String,
    signal: bool,
    target: String,
}

impl std::fmt::Debug for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Pulse: {} -> {} -> {}",
            self.source,
            match self.signal {
                true => "HIGH",
                false => "LOW",
            },
            self.target
        ))
    }
}

trait PulseReceiver: std::fmt::Debug + DynClone {
    fn receive_pulse(&mut self, pulse: Pulse, pulse_queue: &mut VecDeque<Pulse>);
    fn targets(&self) -> &Vec<String>;
    fn id(&self) -> &String;
    fn is_conjunction(&self) -> bool {
        false
    }
}

clone_trait_object!(PulseReceiver);

#[derive(Debug, Clone)]
struct FlipFlop {
    id: String,
    state: bool,
    targets: Vec<String>,
}

impl PulseReceiver for FlipFlop {
    fn receive_pulse(&mut self, pulse: Pulse, pulse_queue: &mut VecDeque<Pulse>) {
        if !pulse.signal {
            self.state = !self.state;
            for target in &self.targets {
                pulse_queue.push_back(Pulse {
                    source: self.id.clone(),
                    signal: self.state,
                    target: target.clone(),
                });
            }
        }
    }

    fn targets(&self) -> &Vec<String> {
        &self.targets
    }

    fn id(&self) -> &String {
        &self.id
    }
}

impl FlipFlop {
    fn parse(input: &str) -> IResult<&str, (String, Box<dyn PulseReceiver>)> {
        // e.g.
        // %lg -> zx, lx
        let (input, (id, targets)) = separated_pair(
            preceded(tag("%"), alpha1),
            tag(" -> "),
            separated_list0(tag(", "), alpha1),
        )(input)?;
        Ok((
            input,
            (
                id.to_string(),
                Box::new(Self {
                    id: id.to_string(),
                    state: false,
                    targets: targets
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>(),
                }),
            ),
        ))
    }
}

#[derive(Debug, Clone)]
struct Conjunction {
    id: String,
    states: HashMap<String, bool>,
    targets: Vec<String>,
}

impl Conjunction {
    fn parse(input: &str) -> IResult<&str, (String, Box<dyn PulseReceiver>)> {
        // e.g.
        // &lg -> zx, lx
        let (input, (id, targets)) = separated_pair(
            preceded(tag("&"), alpha1),
            tag(" -> "),
            separated_list0(tag(", "), alpha1),
        )(input)?;
        Ok((
            input,
            (
                id.to_string(),
                Box::new(Self {
                    id: id.to_string(),
                    states: HashMap::new(),
                    targets: targets
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>(),
                }),
            ),
        ))
    }
}

impl PulseReceiver for Conjunction {
    fn receive_pulse(&mut self, pulse: Pulse, pulse_queue: &mut VecDeque<Pulse>) {
        // update state
        self.states.insert(pulse.source, pulse.signal);

        // if all states are true, send pulse
        let signal = !self.states.values().all(|&v| v);

        // send pulse
        for target in &self.targets {
            pulse_queue.push_back(Pulse {
                source: self.id.clone(),
                signal: signal,
                target: target.clone(),
            });
        }
    }

    fn targets(&self) -> &Vec<String> {
        &self.targets
    }

    fn is_conjunction(&self) -> bool {
        true
    }

    fn id(&self) -> &String {
        &self.id
    }
}

#[derive(Debug, Clone)]
struct Broadcaster {
    id: String,
    targets: Vec<String>,
}

impl Broadcaster {
    fn parse(input: &str) -> IResult<&str, (String, Box<dyn PulseReceiver>)> {
        // e.g.
        // broadcaster -> a, b, c
        let (input, (id, targets)) =
            separated_pair(alpha1, tag(" -> "), separated_list0(tag(", "), alpha1))(input)?;
        Ok((
            input,
            (
                id.to_string(),
                Box::new(Self {
                    id: id.to_string(),
                    targets: targets
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>(),
                }),
            ),
        ))
    }
}

impl PulseReceiver for Broadcaster {
    fn receive_pulse(&mut self, pulse: Pulse, pulse_queue: &mut VecDeque<Pulse>) {
        // send pulse
        for target in &self.targets {
            pulse_queue.push_back(Pulse {
                source: self.id.clone(),
                signal: pulse.signal,
                target: target.clone(),
            });
        }
    }

    fn targets(&self) -> &Vec<String> {
        &self.targets
    }

    fn id(&self) -> &String {
        &self.id
    }
}

fn parse(input: &str) -> HashMap<String, Box<dyn PulseReceiver>> {
    let mut machines: HashMap<String, Box<dyn PulseReceiver>> = HashMap::new();
    for line in input.lines() {
        let (_, (id, machine)) =
            alt((FlipFlop::parse, Conjunction::parse, Broadcaster::parse))(line).unwrap();

        machines.insert(id.to_string(), machine);
    }

    // prime all the conjunction states
    let mut conjunction_primers: HashMap<String, Vec<String>> = HashMap::new();
    for machine in machines.values() {
        for target in machine.targets() {
            if let Some(target_machine) = machines.get(target) {
                if target_machine.is_conjunction() {
                    conjunction_primers
                        .entry(target_machine.id().clone())
                        .or_default()
                        .push(machine.id().to_string());
                }
            }
        }
    }
    for (conjunction_id, primers) in conjunction_primers {
        let conjunction_machine = machines.get_mut(&conjunction_id).unwrap();
        for primer in primers {
            conjunction_machine.receive_pulse(
                Pulse {
                    source: primer.to_string(),
                    signal: false,
                    target: conjunction_machine.id().to_string(),
                },
                &mut VecDeque::new(),
            );
        }
    }
    machines
}

fn part1(input: &str) -> usize {
    let mut pulse_queue: VecDeque<Pulse> = VecDeque::new();
    let mut pulse_history: Vec<Pulse> = Vec::new();
    let mut machines = parse(input);

    for _ in 0..1000 {
        pulse_queue.push_back(Pulse {
            source: "button".to_string(),
            signal: false,
            target: "broadcaster".to_string(),
        });
        while !pulse_queue.is_empty() {
            let pulse = pulse_queue.pop_front().unwrap();
            pulse_history.push(pulse.clone());
            if let Some(machine) = machines.get_mut(&pulse.target) {
                machine.receive_pulse(pulse, &mut pulse_queue);
            }
        }
    }
    let high_pulses = pulse_history.iter().filter(|p| p.signal).count();
    let low_pulses = pulse_history.iter().filter(|p| !p.signal).count();
    high_pulses * low_pulses
}

fn part2(input: &str) -> usize {
    let machines = parse(input);

    // get the machines that have rx's parent as the target
    // for my case it is &bq -> rx
    let mut rx_parent: Option<String> = None;

    for machine in machines.values() {
        for target in machine.targets() {
            if target == "bq" {
                rx_parent = Some(machine.id().to_string());
            }
        }
    }

    assert!(rx_parent.is_some());

    let mut rx_parents: Vec<String> = Vec::new();
    for machine in machines.values() {
        for target in machine.targets() {
            if target == "bq" {
                rx_parents.push(machine.id().to_string());
            }
        }
    }

    dbg!(&rx_parents);

    let button_presses: Vec<usize> = rx_parents
        .iter()
        .map(|parent| {
            let mut pulse_queue: VecDeque<Pulse> = VecDeque::new();
            let mut pulse_history: Vec<Pulse> = Vec::new();
            let mut button_presses = 0;
            let mut machines = machines.clone();
            'outer: loop {
                pulse_queue.push_back(Pulse {
                    source: "button".to_string(),
                    signal: false,
                    target: "broadcaster".to_string(),
                });
                button_presses += 1;
                while !pulse_queue.is_empty() {
                    let pulse = pulse_queue.pop_front().unwrap();
                    if &pulse.target == parent && !pulse.signal {
                        break 'outer;
                    }
                    pulse_history.push(pulse.clone());
                    if let Some(machine) = machines.get_mut(&pulse.target) {
                        machine.receive_pulse(pulse, &mut pulse_queue);
                    }
                }
            }
            button_presses
        })
        .collect();

    button_presses
        .iter()
        .fold(1, |acc, x| num::integer::lcm(acc, *x))
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE1: &str = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;

    static EXAMPLE2: &str = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(EXAMPLE1), 32000000);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(EXAMPLE2), 11687500);
    }
}
