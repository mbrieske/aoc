use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fmt,
    sync::atomic::{AtomicUsize, Ordering},
};
use tracing::{event, instrument, Level};

static HIGH_PULSES: AtomicUsize = AtomicUsize::new(0);
static LOW_PULSES: AtomicUsize = AtomicUsize::new(0);
static PRESSES: AtomicUsize = AtomicUsize::new(0);

pub fn solve(input: &str) -> usize {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        // .with_span_events(FmtSpan::ENTER)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let circuit = Circuit::from(input);
    loop {
        circuit.press_button();
    }

    // for _ in 0..1000 {
    //     circuit.press_button();
    // }
    // LOW_PULSES.fetch_add(0, Ordering::SeqCst) * HIGH_PULSES.fetch_add(0, Ordering::SeqCst)
}

#[derive(Debug)]
struct Circuit<'a> {
    modules: HashMap<&'a str, Module<'a>>,
    queue: RefCell<VecDeque<Pulse<'a>>>,
}

impl<'a> From<&'a str> for Circuit<'a> {
    fn from(input: &'a str) -> Self {
        let mut modules: HashMap<&str, Module> = input
            .lines()
            .map(|line| {
                let (module, outputs) = line.split_once(" -> ").unwrap();
                let outputs = outputs.split(", ").collect::<Vec<&str>>();
                let (name, modtype) = match module.chars().next().unwrap() {
                    '%' => (&module[1..], ModType::FlipFlop(RefCell::new(false))),
                    '&' => (
                        &module[1..],
                        ModType::Conjunction(RefCell::new(HashMap::new())),
                    ),
                    'b' => (module, ModType::Broadcaster),
                    _ => unreachable!(),
                };
                (
                    name,
                    Module {
                        name,
                        modtype,
                        outputs,
                    },
                )
            })
            .collect();

        modules.insert(
            "button",
            Module {
                name: "button",
                modtype: ModType::Button,
                outputs: vec!["broadcaster"],
            },
        );

        let mut not_found = Vec::new();
        modules.iter().for_each(|(&name, module)| {
            module.outputs.iter().for_each(|&output| {
                if let Some(output) = modules.get(output) {
                    if let ModType::Conjunction(inputs) = &output.modtype {
                        inputs.borrow_mut().insert(name, false);
                    }
                } else {
                    not_found.push(output);
                }
            })
        });

        for module in not_found {
            modules.insert(
                module,
                Module {
                    name: module,
                    modtype: ModType::Output,
                    outputs: Vec::new(),
                },
            );
        }

        Self {
            modules,
            queue: RefCell::new(VecDeque::new()),
        }
    }
}

struct Pulse<'a> {
    from: &'a str,
    to: &'a str,
    level: bool,
}

impl<'a> fmt::Debug for Pulse<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level_str = if self.level { "high" } else { "low" };
        write!(f, " {} -{}-> {}", self.from, level_str, self.to)
    }
}

impl<'a> Circuit<'a> {
    #[instrument(level = "TRACE", skip(self))]
    fn press_button(&'a self) {
        PRESSES.fetch_add(1, Ordering::SeqCst);
        self.queue.borrow_mut().extend(self.pulse(Pulse {
            from: "button",
            to: "broadcaster",
            level: false,
        }));

        while let Some(pulse) = {
            let result = self.queue.borrow_mut().pop_front();
            result
        } {
            let next = self.pulse(pulse);
            self.queue.borrow_mut().extend(next);
        }
    }

    #[instrument(level = "TRACE", skip(self))]
    fn pulse(&self, pulse: Pulse) -> Vec<Pulse> {
        match pulse.level {
            false => LOW_PULSES.fetch_add(1, Ordering::SeqCst),
            true => HIGH_PULSES.fetch_add(1, Ordering::SeqCst),
        };
        let target = self.modules.get(pulse.to).unwrap();
        let next: Vec<Pulse> = match &target.modtype {
            ModType::Button => unreachable!(),
            ModType::Broadcaster => target
                .outputs
                .iter()
                .map(|&output| Pulse {
                    from: target.name,
                    to: output,
                    level: pulse.level,
                })
                .collect(),
            ModType::FlipFlop(state) => {
                if !pulse.level {
                    let new_state = !*state.borrow();
                    *state.borrow_mut() = new_state;
                    target
                        .outputs
                        .iter()
                        .map(|&output| Pulse {
                            from: target.name,
                            to: output,
                            level: new_state,
                        })
                        .collect()
                } else {
                    Vec::new()
                }
            }
            ModType::Conjunction(inputs) => {
                *inputs.borrow_mut().get_mut(pulse.from).unwrap() = pulse.level;
                let output_level = if inputs
                    .borrow()
                    .iter()
                    .map(|(_, &level)| level)
                    .all(|level| level)
                {
                    false
                } else {
                    match target.name {
                        "cq" | "rv" | "vp" | "dc" => event!(
                            Level::INFO,
                            "{} high at {} button presses",
                            target.name,
                            PRESSES.fetch_add(0, Ordering::SeqCst)
                        ),
                        _ => (),
                    }
                    true
                };
                target
                    .outputs
                    .iter()
                    .map(|&output| Pulse {
                        from: target.name,
                        to: output,
                        level: output_level,
                    })
                    .collect()
            }
            ModType::Output => Vec::new(),
        };
        // event!(Level::INFO, "triggered: {:?}", next);
        next
    }
}

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
    modtype: ModType<'a>,
    outputs: Vec<&'a str>,
}

#[derive(Debug)]
enum ModType<'a> {
    Broadcaster,
    FlipFlop(RefCell<bool>),
    Conjunction(RefCell<HashMap<&'a str, bool>>),
    Output,
    Button,
}

#[cfg(test)]
mod tests {
    use super::*;
    // use rstest::rstest;

    #[test]
    fn example() {
        let example = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!(solve(example), 32000000);
    }

    #[test]
    fn example_interesting() {
        let example = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!(solve(example), 11687500);
    }
}
