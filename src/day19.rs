use std::{collections::HashMap, fs::File, io};

use crate::files::read_lines;

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn process(&self, part: &Part) -> String {
        for rule in &self.rules {
            if rule.accept(part) {
                return rule.target.clone();
            }
        }

        panic!("None of the workflow rule processed the input")
    }

    fn parse(s: &str) -> Workflow {
        let s = s.split("{").collect::<Vec<&str>>();
        let name = s[0];
        let rules = s[1].replace("}", "");
        let rules = rules.split(",").collect::<Vec<&str>>();

        let mut rs = Vec::new();
        for rule in rules {
            let rule = Rule::parse(rule);
            rs.push(rule);
        }

        Workflow {
            name: name.into(),
            rules: rs,
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    kind: RuleKind,
    segment: Segment,
    value: i32,
    target: String,
}

impl Rule {
    fn invert(&self) -> Rule {
        let (kind, value) = match self.kind {
            RuleKind::Empty => (self.kind.clone(), self.value),
            // > 2007
            // <= 2007
            // < 2008
            RuleKind::Greater => (RuleKind::Lesser, self.value + 1),
            // < 1999
            // >= 1999
            // > 1998
            RuleKind::Lesser => (RuleKind::Greater, self.value - 1),
        };

        Rule {
            kind,
            value,
            segment: self.segment,
            target: self.target.clone()
        }
    }

    fn parse(s: &str) -> Rule {
        if !s.contains(":") {
            return Rule {
                kind: RuleKind::Empty,
                segment: Segment::X,
                value: -1,
                target: s.into(),
            };
        }

        let s = s.split(":").collect::<Vec<&str>>();
        let target = s[1];
        let s = s[0];

        let segment = match &s[0..1] {
            "x" => Segment::X,
            "m" => Segment::M,
            "a" => Segment::A,
            "s" => Segment::S,
            _ => panic!("unknown segment"),
        };

        let kind = match &s[1..2] {
            ">" => RuleKind::Greater,
            "<" => RuleKind::Lesser,
            _ => panic!("unknown rule kind"),
        };

        let value = *&s[2..].parse::<i32>().unwrap();

        Rule {
            kind,
            segment,
            value,
            target: target.into(),
        }
    }

    fn accept(&self, part: &Part) -> bool {
        let v = match self.segment {
            Segment::X => part.x,
            Segment::M => part.m,
            Segment::A => part.a,
            Segment::S => part.s,
        };

        match self.kind {
            RuleKind::Empty => true,
            RuleKind::Greater => v > self.value,
            RuleKind::Lesser => v < self.value,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum RuleKind {
    Empty,
    Greater,
    Lesser,
}

#[derive(PartialEq, Debug, Clone, Copy)]

enum Segment {
    X,
    M,
    A,
    S,
}

struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
    target: String,
}

impl Part {
    fn parse(s: &str) -> Part {
        let s = &s[1..s.len() - 1];
        let values = s.split(",").collect::<Vec<&str>>();

        let mut vs = vec![];
        for value in values {
            let s = value.split("=").collect::<Vec<&str>>();
            let x: i32 = s[1].parse().unwrap();

            vs.push(x);
        }

        Part {
            x: vs[0],
            m: vs[1],
            a: vs[2],
            s: vs[3],
            target: "in".into(),
        }
    }

    fn rating(&self) -> i32 {
        return self.x + self.m + self.a + self.s;
    }
}

fn process_part(part: &mut Part, workflows: &HashMap<String, Workflow>) -> bool {
    loop {
        match part.target.as_str() {
            "A" => {
                return true;
            }
            "R" => {
                return false;
            }
            target => {
                let workflow = workflows.get(target).expect(
                    format!(
                        "Workflow of given name was not found in workflows: {}",
                        target
                    )
                    .as_str(),
                );
                part.target = workflow.process(part);
            }
        }
    }
}

fn process_parts(parts: &mut Vec<Part>, workflows: &HashMap<String, Workflow>) -> i32 {
    for part in parts.iter_mut() {
        process_part(part, workflows);
    }

    parts.iter()
        .filter(|p| p.target == "A")
        .map(|p| p.rating())
        .sum()
}

fn parse(filename: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let lines: io::Lines<io::BufReader<File>> = read_lines(filename).unwrap();

    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts = Vec::new();

    let mut parsing_workflows = true;
    for line in lines {
        if let Ok(line) = line {
            if line.is_empty() {
                parsing_workflows = false;
                continue;
            }

            if parsing_workflows {
                let workflow = Workflow::parse(&line);
                workflows.insert(workflow.name.clone(), workflow);
            } else {
                let part = Part::parse(&line);
                parts.push(part);
            }
        }
    }

    (workflows, parts)
}

pub fn part1() {
    let (workflow, mut parts) = parse("input/19.txt");
    let ans: i32 = process_parts(&mut parts, &workflow);

    println!("Part1: {ans}");
}

fn determine_ranges(constraints: &Vec<Rule>) -> i64 {
    let mut s_lower: i64 = 0;
    let mut s_upper: i64 = 4001;
    let mut x_lower: i64 = 0;
    let mut x_upper: i64 = 4001;
    let mut m_lower: i64 = 0;
    let mut m_upper: i64 = 4001;
    let mut a_lower: i64 = 0;
    let mut a_upper: i64 = 4001;

    for constraint in constraints {
        match constraint.segment {
            Segment::X => {
                match constraint.kind {
                    RuleKind::Empty => {
                        // do nothing, its not constrained
                    }
                    RuleKind::Greater => {
                        if constraint.value as i64 > x_lower {
                            x_lower = constraint.value as i64;
                        } 
                    },
                    RuleKind::Lesser => {
                        if (constraint.value as i64) < x_upper {
                            x_upper = constraint.value as i64;
                        }
                    }
                }
            },
            Segment::M => {
                match constraint.kind {
                    RuleKind::Empty => {
                        // do nothing, its not constrained
                    }
                    RuleKind::Greater => {
                        if (constraint.value as i64) > m_lower {
                            m_lower = constraint.value as i64;
                        } 
                    },
                    RuleKind::Lesser => {
                        if (constraint.value as i64) < m_upper {
                            m_upper = constraint.value as i64;
                        }
                    }
                }
            }
            Segment::A => {
                match constraint.kind {
                    RuleKind::Empty => {
                        // do nothing, its not constrained
                    }
                    RuleKind::Greater => {
                        if (constraint.value as i64) > a_lower {
                            a_lower = constraint.value as i64;
                        } 
                    },
                    RuleKind::Lesser => {
                        if (constraint.value as i64) < a_upper {
                            a_upper = constraint.value as i64;
                        }
                    }
                }
            }
            Segment::S => {
                match constraint.kind {
                    RuleKind::Empty => {
                        // do nothing, its not constrained
                    },
                    RuleKind::Greater => {
                        if (constraint.value as i64) > s_lower {
                            s_lower = constraint.value as i64;
                        } 
                    },
                    RuleKind::Lesser => {
                        if (constraint.value as i64) < s_upper {
                            s_upper = constraint.value as i64;
                        }
                    }
                }
            }
        }
    }

    // println!("X: ({x_lower} {x_upper})");
    // println!("M: ({m_lower} {m_upper})");
    // println!("A: ({a_lower} {a_upper})");
    // println!("S: ({s_lower} {s_upper})");
    // 10 2
    // 3 4 5 6 7 8 9
    
    let x = (x_upper - x_lower) - 1;
    let m = (m_upper - m_lower) - 1;
    let a = (a_upper - a_lower) - 1;
    let s = (s_upper - s_lower) - 1;

    return x * m * a * s;
    // println!("{ans}");
}


fn traverse(current_workflow: &str, workflows: &HashMap<String, Workflow>, constraints: &mut Vec<Rule>, path: &mut Vec<String>) -> i64 {
    path.push(current_workflow.into());

    if current_workflow == "A" {
        for (idx, a) in path.iter().enumerate() {
            if idx != path.len() - 1 {
                print!("{} -> ", a);
            } else {
                print!("{a}");
            }
        }
        println!();
        // println!("{:?}",  constraints);
        path.pop();
        return determine_ranges(constraints);
    } else if current_workflow == "R" {
        path.pop();
        return 0;
    }


    let mut ans = 0;
    let workflow = workflows.get(current_workflow).expect("workflow should exist");
    for rule in &workflow.rules {
        constraints.push(rule.clone());
        ans += traverse(&rule.target, workflows, constraints, path);
        constraints.pop();
        constraints.push(rule.invert());
    }

    for _ in 0..workflow.rules.len() {
        constraints.pop();
    }

    path.pop();

    return ans;
}


pub fn part2() {
    let (workflows, _) = parse("input/19.txt");


    let mut constraints = Vec::new();
    let mut path = Vec::new();
    let ans = traverse("in", &workflows, &mut constraints, &mut path);

    // start from in, dfs into each rule, invert if necessary
    // if reached, then display, we'll be gooood
    // upper_bound - lower_bound - 1 (num of values)

    println!("Part 2: {ans}");
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn sanity() {
        let part = Part {
            x: 787,
            m: 2655,
            a: 1222,
            s: 2876,
            target: "in".into(),
        };
        let mut parts = vec![part];

        let mut workflows: HashMap<String, Workflow> = HashMap::new();

        let w_in = Workflow {
            name: "in".into(),
            rules: vec![
                Rule {
                    kind: RuleKind::Lesser,
                    segment: Segment::S,
                    value: 1351,
                    target: "px".into(),
                },
                Rule {
                    kind: RuleKind::Empty,
                    segment: Segment::S,
                    value: 0,
                    target: "qqz".into(),
                },
            ],
        };
        let w_qqz = Workflow {
            name: "qqz".into(),
            rules: vec![
                Rule {
                    kind: RuleKind::Greater,
                    segment: Segment::S,
                    value: 2770,
                    target: "qs".into(),
                },
                Rule {
                    kind: RuleKind::Lesser,
                    segment: Segment::M,
                    value: 1801,
                    target: "hdj".into(),
                },
                Rule {
                    kind: RuleKind::Empty,
                    segment: Segment::S,
                    value: 0,
                    target: "R".into(),
                },
            ],
        };
        let w_qs = Workflow {
            name: "qs".into(),
            rules: vec![
                Rule {
                    kind: RuleKind::Greater,
                    segment: Segment::S,
                    value: 3448,
                    target: "A".into(),
                },
                Rule {
                    kind: RuleKind::Empty,
                    segment: Segment::S,
                    value: 0,
                    target: "lnx".into(),
                },
            ],
        };
        let w_lnx = Workflow {
            name: "lnx".into(),
            rules: vec![
                Rule {
                    kind: RuleKind::Greater,
                    segment: Segment::M,
                    value: 1548,
                    target: "A".into(),
                },
                Rule {
                    kind: RuleKind::Empty,
                    segment: Segment::S,
                    value: 0,
                    target: "A".into(),
                },
            ],
        };
        workflows.insert("in".into(), w_in);
        workflows.insert("qqz".into(), w_qqz);
        workflows.insert("qs".into(), w_qs);
        workflows.insert("lnx".into(), w_lnx);

        process_parts(&mut parts, &workflows);
    }

    #[test]
    fn parse_rule() {
        let rule = Rule::parse("a>2006:qkq");

        assert_eq!(rule.kind, RuleKind::Greater);
        assert_eq!(rule.value, 2006);
        assert_eq!(rule.target, "qkq");
    }

    #[test]
    fn parse_empty_rule() {
        let rule = Rule::parse("rfg");

        assert_eq!(rule.kind, RuleKind::Empty);
        assert_eq!(rule.value, -1);
        assert_eq!(rule.target, "rfg");
    }

    #[test]
    fn parse_workflow() {
        let workflow = Workflow::parse("px{a<2006:qkq,m>2090:A,rfg}");

        assert_eq!("px", workflow.name);
        assert_eq!(3, workflow.rules.len());
    }

    #[test]
    fn parse_part() {
        let part = Part::parse("{x=787,m=2655,a=1222,s=2876}");

        assert_eq!("in", part.target);
        assert_eq!(787, part.x);
        assert_eq!(2655, part.m);
        assert_eq!(1222, part.a);
        assert_eq!(2876, part.s);
    }

    #[test]
    fn part1() {
        let (workflow, mut parts) = parse("input/19.txt");
        let ans: i32 = process_parts(&mut parts, &workflow);

        assert_eq!(19114, ans);
    }
}
