use std::{collections::{HashMap, VecDeque}, vec, fs::File, io};

use crate::files::read_lines;



struct Module {
    name: String,
    kind: ModuleKind,
    targets: Vec<String>,
    // Conjuction state
    saved: HashMap<String, Signal>,
    // flipflop state
    state: Signal,
}

enum ModuleKind {
    FlipFlop,
    Conjuction,
    Broadcaster,
    Empty
}

#[derive(Clone, Debug, Copy)]
enum Signal {
    LOW,
    HIGH
}

impl Signal {
    fn invert(self) -> Signal {
        match self {
            Signal::LOW => Signal::HIGH,
            Signal::HIGH => Signal::LOW,
        }
    }
}


impl Module {
    fn process(&mut self, source: String, signal: Signal) -> Vec<(String, String, Signal)> {
        match self.kind {
            ModuleKind::FlipFlop => {
                match signal {
                    Signal::LOW => {
                        self.state = self.state.invert();

                        self.targets.iter()
                            .map(|t| (self.name.clone(), t.clone(), self.state.clone()))
                            .collect()
                    },
                    Signal::HIGH => {
                        // do nothing
                        vec![]
                    }
                }
            },
            ModuleKind::Conjuction => {
                // assumption, HashMap contains all inputs set to false before processing
                self.saved.insert(source, signal);

                let mut all_high = true;
                for saved_signal in self.saved.values() {
                    match saved_signal {
                        Signal::HIGH => {
                            // do nothing
                        },
                        Signal::LOW => {
                            all_high = false;
                            break;
                        }
                    }
                }

                let response = if all_high {
                    Signal::LOW 
                } else {
                    Signal::HIGH
                };

                self.targets.iter()
                    .map(|t| (self.name.clone(), t.clone(), response))
                    .collect()
            },
            ModuleKind::Broadcaster => {
                self.targets.iter()
                    .map(|t| (self.name.clone(), t.clone(), signal))
                    .collect()
            },
            ModuleKind::Empty => {
                match signal {
                    Signal::LOW => panic!("Got it!"),
                    Signal::HIGH => {
                        // DO NOTHING!
                        vec![]
                    },
                }
            },
        }
    }


    fn parse(s: &str) -> Module {
        let (s, kind) = match &s[0..1] {
            "%" => (&s[1..], ModuleKind::FlipFlop),
            "&" => (&s[1..], ModuleKind::Conjuction),
            _ => (s, ModuleKind::Broadcaster)
        };

        let s = s.split(" -> ").collect::<Vec<&str>>();
        let name = s[0].into();
        let targets = s[1].split(", ").map(|s| s.into()).collect::<Vec<String>>();

        Module {
            name,
            kind,
            targets,
            state: Signal::LOW,
            saved: HashMap::new()
        }
    }

    fn empty(name: &str) -> Module { 
        Module {
            kind: ModuleKind::Empty,
            name: name.into(),
            saved: HashMap::new(),
            state: Signal::LOW,
            targets: vec![]
        }
    }
}


fn print_signals(signals: &Vec<(String, String, Signal)>) {
    for s in signals {
        print_signal(s);
    }
}

fn print_signal((from, target, signal): &(String, String, Signal)) {
    println!("{} -{:?}-> {}", from, signal, target);
}




fn preprocess(modules: Vec<Module>) -> HashMap<String, Module> {
    let mut module_map: HashMap<String, Module> = HashMap::new();
    for module in modules {
        module_map.insert(module.name.clone(), module);
    }

    // fix converters
    let keys: Vec<String> = module_map.keys().map(|k| k.into()).collect();
    for key in keys {
        let targets = module_map.get(key.as_str()).unwrap().targets.clone();

        for target in targets {
            println!("Getting {}", target);
            if !module_map.contains_key(&target) {
                module_map.insert(target.clone(), Module::empty(&target));
            }
            let target = module_map.get_mut(&target).unwrap();
            target.saved.insert(key.clone(), Signal::LOW);
        }
    }

    module_map
}

fn push_button(modules: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut lows = 0;
    let mut highs = 0;
    let mut queue = VecDeque::new();
    queue.push_back((String::from("button"), String::from("broadcaster"), Signal::LOW));

    while queue.len() > 0 {
        let signal = queue.pop_front().unwrap();
        // print_signal(&signal);
        let (source, target, value) = signal;
        match value {
            Signal::LOW => lows += 1,
            Signal::HIGH => highs += 1,
        }

        let module = modules.get_mut(&target)
            .expect(format!("Module should be there: {}", target).as_str());

        let next_signals = module.process(source, value);

        for s in next_signals {
            queue.push_back(s);
        }
    }

    (highs, lows)
}


fn parse(filename: &str) -> (HashMap<String, Module>) {
    let lines: io::Lines<io::BufReader<File>> = read_lines(filename).unwrap();

    let mut modules: Vec<Module> = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            if line.is_empty() {
                continue;
            }

            modules.push(Module::parse(&line));
        }
    }

    preprocess(modules)
}

pub fn part1() {
    let mut modules = parse("input/20.txt");


    let mut ans_high = 0;
    let mut ans_low = 0;
    for _ in 0..1000 {
        let (high, lows) = push_button(&mut modules);
        ans_high += high;
        ans_low += lows;
    }

    println!("Part 1: {}", ans_high * ans_low);
}

pub fn part2() {
    let mut modules = parse("input/20.txt");

    let mut button_pushes = 0;
    loop {
        button_pushes += 1;
        push_button(&mut modules);
    }

}



#[cfg(test)]
mod tests {
    use std::collections::{VecDeque};

    use super::*;

    #[test]
    fn sanity() {
        let mut modules = preprocess(vec![
            Module::parse("broadcaster -> a, b, c"),
            Module::parse("%a -> b"),
            Module::parse("%b -> c"),
            Module::parse("%c -> inv"),
            Module::parse("&inv -> a"),
        ]);

        let mut ans_high = 0;
        let mut ans_low = 0;
        for _ in 0..1000 {
            let (high, lows) = push_button(&mut modules);
            ans_high += high;
            ans_low += lows;
        }

        assert_eq!(32000000, ans_high * ans_low);
    }

    #[test]
    fn sanity2() {
        let mut modules = vec![
            Module::parse("broadcaster -> a"),
            Module::parse("%a -> inv, con"),
            Module::parse("&inv -> b"),
            Module::parse("%b -> con"),
            Module::parse("&con -> output"),
            Module {
                kind: ModuleKind::Empty,
                name: "output".into(),
                saved: HashMap::new(),
                state: Signal::LOW,
                targets: vec![]
            }
        ];
        let mut modules = preprocess(modules);
        push_button(&mut modules);
        println!("Second time");
        push_button(&mut modules);
        println!("Third time");
        push_button(&mut modules);
        println!("Fourth time");
        push_button(&mut modules);
    }
}