use std::collections::HashMap;

fn parse(data: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let path = data.lines().nth(0).unwrap().chars().collect::<Vec<_>>();
    let mut rules = HashMap::new();
    for rule in data.lines().skip(2) {
        let origin = &rule[0..3];
        let left = &rule[7..10];
        let right = &rule[12..15];
        rules.insert(origin, (left, right));
    }
    (path, rules)
}
pub fn part1(data: &str) -> i64 {

    let (path, rules) = parse(data);
    let mut iterations = 0;
    let mut current = "AAA";
    let mut path_index = 0;
    while current != "ZZZ" {
        let (left, right) = rules.get(current).unwrap();
        if path[path_index] == 'L' { current = left; }
        else { current = right; }
        iterations += 1;
        path_index = (path_index + 1) % path.len();
    }
    iterations
}

pub fn part2(data: &str) -> i64 {
    let (path, rules) = parse(data);
    let mut iterations = 0;
    let mut current = rules
        .iter()
        .filter_map(|(&x,_)| if x.ends_with('A') { Some(x) } else { None })
        .collect::<Vec<_>>();
    let mut path_index = 0;
    let mut largest_common_multiple = 1;
    while !current.is_empty() {
        current.retain_mut(|c| {
            if c.ends_with('Z') {
                largest_common_multiple = num::Integer::lcm(&largest_common_multiple, &iterations);
                false
            } else {
                let (left, right) = rules.get(c).unwrap();
                if path[path_index] == 'L' { *c = left; }
                else { *c = right; }
                true
            }
        });
        iterations += 1;
        path_index = (path_index + 1) % path.len();
    }
    largest_common_multiple
}