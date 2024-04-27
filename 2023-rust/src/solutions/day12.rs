use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

fn get_arrangements<'a>(springs: &'a str, groups: &'a [i64], cache: Option<Rc<RefCell<HashMap<(&'a str, &'a [i64]), i64>>>>) -> i64 {

    if groups.is_empty() {
        return if springs.contains('#') { 0 } else { 1 }
    }

    if springs.is_empty() {
        return if groups.is_empty() { 1 } else { 0 }
    }

    if let Some(cache) = cache.clone() {
        if let Some(&value) = cache.borrow().get(&(springs, groups)) {
            return value;
        }
    }

    let mut count = 0;

    // NOT BROKEN
    if springs.starts_with(|x| x == '.' || x == '?') {
        count += get_arrangements(&springs[1..], groups, cache.clone())
    }

    // BROKEN
    if springs.starts_with(|x| x == '#' || x == '?') {
        let len = springs.len();
        let end_of_group = groups[0] as usize;
        if end_of_group <= len
            && !springs[..end_of_group].contains('.')
            && (end_of_group == len || springs.chars().nth(end_of_group).unwrap() != '#')
        {
            count += get_arrangements(
                &springs[usize::min(end_of_group + 1, len)..],
                &groups[1..],
                cache.clone()
            );
        }
    }

    if let Some(cache) = cache {
        cache.borrow_mut().insert((springs, groups), count);
    }
    count
}

fn parse(data: &str) -> Vec<(String, Vec<i64>)>{
    data
        .lines()
        .map(|x| x.split_once(' ').unwrap())
        .map(|(x, y)|
            (
                x.to_string(),
                y.split(',').map(|z|
                    z.parse::<i64>().unwrap()
                ).collect::<Vec<_>>()
            )
        ).collect::<Vec<_>>()
}

pub fn part1(data: &str) -> i64 {
    let data = parse(data);

    let mut result = 0;
    for line in data.iter() {
        result += get_arrangements(line.0.as_str(), &line.1, None);
    }
    result
}

pub fn part2(data: &str) -> i64 {
    let data= parse(data).iter().map(|(x,y)|
        (format!("{x}?{x}?{x}?{x}?{x}"), y.iter().cycle().take(y.len() * 5).map(|&x| x).collect::<Vec<_>>())
    ).collect::<Vec<_>>();

    let cache = Rc::new(RefCell::new(HashMap::new()));
    let mut result = 0;
    for line in data.iter() {
        result += get_arrangements(line.0.as_str(), &line.1, Some(cache.clone()));
    }
    result
}