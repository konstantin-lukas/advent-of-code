use std::collections::HashMap;
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
    result
}

pub fn part2(_data: &str) -> i64 {
    0
}