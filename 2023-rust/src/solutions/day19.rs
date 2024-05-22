use std::collections::{HashMap, HashSet};
use std::ops::Deref;

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64
}

#[derive(Debug)]
enum Op {
    Greater,
    Lesser
}

#[derive(Debug)]
struct If<'a> {
    var: char,
    op: Op,
    val: i64,
    then: Box<Rule<'a>>
}

#[derive(Debug)]
enum Rule<'a> {
    Accept,
    Reject,
    Rule(&'a str),
    Condition(Box<If<'a>>)
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct RangeCollection {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64)
}

type Workflow<'a> = Vec<Rule<'a>>;
type Workflows<'a> = HashMap<&'a str, Workflow<'a>>;

fn parse_parts(parts: &str) -> Vec<Part> {
    parts
        .lines()
        .map(|x| {
            let p = x
                .strip_prefix('{')
                .unwrap()
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .collect::<Vec<_>>();
            Part {
                x: i64::from_str_radix(&p[0][2..], 10).unwrap(),
                m: i64::from_str_radix(&p[1][2..], 10).unwrap(),
                a: i64::from_str_radix(&p[2][2..], 10).unwrap(),
                s: i64::from_str_radix(&p[3][2..], 10).unwrap()
            }
        }
        ).collect::<Vec<_>>()
}

fn parse_workflows(rules: &str) -> Workflows {
    let mut result = HashMap::new();
    for line in rules.lines() {
        let (name, rules) = line.split_once('{').unwrap();
        let rules = rules.strip_suffix('}').unwrap();
        let rules = rules.split(',').map(|x| {
            if let Some((l, r)) = x.split_once('>') {
                let (x, y) = r.split_once(':').unwrap();
                Rule::Condition(Box::from(If {
                    var: l.chars().nth(0).unwrap(),
                    op: Op::Greater,
                    val: i64::from_str_radix(x, 10).unwrap(),
                    then: Box::new(if y == "A" { Rule::Accept } else if y == "R" { Rule::Reject } else { Rule::Rule(y) })
                }))
            } else if let Some((l, r)) = x.split_once('<') {
                let (x, y) = r.split_once(':').unwrap();
                Rule::Condition(Box::from(If {
                    var: l.chars().nth(0).unwrap(),
                    op: Op::Lesser,
                    val: i64::from_str_radix(x, 10).unwrap(),
                    then: Box::new(if y == "A" { Rule::Accept } else if y == "R" { Rule::Reject } else { Rule::Rule(y) })
                }))
            } else if x == "A" {
                Rule::Accept
            } else if x == "R" {
                Rule::Reject
            } else {
                Rule::Rule(x)
            }
        }).collect::<Workflow>();
        result.insert(name, rules);
    }
    result
}

fn get_value(ch: char, part: &Part) -> i64 {
    if ch == 'x' { return part.x; }
    if ch == 'm' { return part.m; }
    if ch == 'a' { return part.a; }
    if ch == 's' { return part.s; }
    panic!();
}

pub fn part1(data: &str) -> i64 {
    let (workflows, parts) = data.split_once("\n\n").or_else(|| data.split_once("\r\n\r\n")).unwrap();
    let parts = parse_parts(parts);
    let workflows = parse_workflows(workflows);
    let mut result = 0;
    for part in parts.iter() {
        let mut current_workflow = workflows.get("in").unwrap();
        let mut i = 0;
        let value = loop {
            if let Rule::Accept = current_workflow[i] { break part.x + part.m + part.a + part.s; }
            if let Rule::Reject = current_workflow[i] { break 0; }
            if let Rule::Rule(w) = current_workflow[i] {
                i = 0;
                current_workflow = workflows.get(w).unwrap();
                continue;
            }
            if let Rule::Condition(c) = &current_workflow[i] {
                if let If { op: Op::Lesser, then, var, val } = c.deref() {
                    if get_value(*var, part) < *val {
                        if let Rule::Rule(next) = then.deref() {
                            current_workflow = workflows.get(next).unwrap();
                            i = 0;
                            continue;
                        } else if let Rule::Accept = then.deref() {
                            break part.x + part.m + part.a + part.s;
                        } else {
                            break 0;
                        }
                    }
                } else if let If { op: Op::Greater, then, var, val } = c.deref() {
                    if get_value(*var, part) > *val {
                        if let Rule::Rule(next) = then.deref() {
                            current_workflow = workflows.get(next).unwrap();
                            i = 0;
                            continue;
                        } else if let Rule::Accept = then.deref() {
                            break part.x + part.m + part.a + part.s;
                        } else {
                            break 0;
                        }
                    }
                }
            }
            i += 1;
        };
        result += value;
    }
    return result;
}

fn count(
    ranges: &mut HashSet<RangeCollection>,
    workflows: &Workflows,
    workflow: &Workflow,
    rule_idx: usize,
    range: RangeCollection
) {
    let mut range = range;
    let rules = &workflow[rule_idx..];
    for rule in rules {
        match rule {
            Rule::Accept => {
                ranges.insert(range);
                return;
            }
            Rule::Reject => {
                return;
            }
            Rule::Condition(c) => {
                let (op, val, then, &ch) = match c.deref() {
                    If { op, val, then, var: ch } => (op, val, then, ch)
                };

                let mut range_lower = range.clone();
                let mut range_upper = range.clone();

                if let Op::Lesser = op {
                    match ch {
                        'x' => {
                            range_lower.x.1 = range_lower.x.1.min(val - 1);
                            range_upper.x.0 = range_upper.x.0.max(*val);
                        }
                        'm' => {
                            range_lower.m.1 = range_lower.m.1.min(val - 1);
                            range_upper.m.0 = range_upper.m.0.max(*val);
                        }
                        'a' => {
                            range_lower.a.1 = range_lower.a.1.min(val - 1);
                            range_upper.a.0 = range_upper.a.0.max(*val);
                        }
                        's' => {
                            range_lower.s.1 = range_lower.s.1.min(val - 1);
                            range_upper.s.0 = range_upper.s.0.max(*val);
                        }
                        _ => panic!("Unknown variable"),
                    }
                    match then.deref() {
                        Rule::Accept => {
                            ranges.insert(range_lower.clone());
                        }
                        Rule::Reject => {}
                        Rule::Rule(r) => {
                            let w = workflows.get(r).unwrap();
                            count(ranges, workflows, w, 0, range_lower.clone());
                        }
                        _ =>  panic!("Unexpected Rule")
                    }
                    range = range_upper.clone();
                } else if let Op::Greater = op {
                    match ch {
                        'x' => {
                            range_lower.x.1 = range_lower.x.1.min(*val);
                            range_upper.x.0 = range_upper.x.0.max(val + 1);
                        }
                        'm' => {
                            range_lower.m.1 = range_lower.m.1.min(*val);
                            range_upper.m.0 = range_upper.m.0.max(val + 1);
                        }
                        'a' => {
                            range_lower.a.1 = range_lower.a.1.min(*val);
                            range_upper.a.0 = range_upper.a.0.max(val + 1);
                        }
                        's' => {
                            range_lower.s.1 = range_lower.s.1.min(*val);
                            range_upper.s.0 = range_upper.s.0.max(val + 1);
                        }
                        _ => panic!("Unknown variable"),
                    }
                    match then.deref() {
                        Rule::Accept => {
                            ranges.insert(range_upper.clone());
                        }
                        Rule::Reject => {}
                        Rule::Rule(r) => {
                            let w = workflows.get(r).unwrap();
                            count(ranges, workflows, w, 0, range_upper.clone());
                        }
                        _ =>  panic!("Unexpected Rule")
                    }
                    range = range_lower.clone();
                } else { panic!("Unknown Operator"); }
            }
            Rule::Rule(r) => {
                let w = workflows.get(r).unwrap();
                count(ranges, workflows, w, 0, range.clone());
            }
        }
    }
}


pub fn part2(data: &str) -> i64 {
    let (workflows, ..) = data.split_once("\n\n").or_else(|| data.split_once("\r\n\r\n")).unwrap();
    let workflows = parse_workflows(workflows);
    let current_workflow = workflows.get("in").unwrap();
    let mut ranges: HashSet<RangeCollection> = HashSet::new();
    let current_range_collection = RangeCollection {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000)
    };
    count(&mut ranges, &workflows, &current_workflow, 0, current_range_collection);

    ranges.iter().filter_map(|r| {
        if r.x.0 > r.x.1 || r.m.0 > r.m.1 || r.a.0 > r.a.1 || r.s.0 > r.s.1 { return None; }
        Some((r.x.1 - r.x.0 + 1) * (r.m.1 - r.m.0 + 1) * (r.a.1 - r.a.0 + 1) * (r.s.1 - r.s.0 + 1))
    }).sum()
}