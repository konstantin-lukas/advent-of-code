use std::collections::LinkedList;
use std::fmt::{Debug, Formatter};

fn hash(string: &[u8]) -> i64 {
    let mut current_value = 0;
    for ch in string.iter() {
        current_value += *ch as i64;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn parse(data: &str) -> Vec<Vec<u8>> {
    data
        .split(',')
        .map(|x| x.chars().filter_map(|y|
            if y == '\n' || y == '\r' { None } else { Some(y as u8) }
        ).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn part1(data: &str) -> i64 {
    let data = parse(data);
    data.iter().map(|x| hash(x)).sum()
}

type Label<'a> = &'a [u8];

struct HashMap<'a> {
    data: [LinkedList<(Label<'a>, u8)>; 256]
}

impl Debug for HashMap<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for (box_number, datum) in self.data.iter().enumerate() {
            result.push_str(format!("Box {box_number}: ").as_str());
            for (label, value) in datum.iter() {
                let mut l = String::new();
                for &label in label.iter() {
                    l.push(label as char);
                }
                result.push_str(format!("[{} {}]", l, value).as_str());
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

impl<'a> HashMap<'a> {
    fn new() -> Self {
        Self {
            data: [(); 256].map(|_| LinkedList::new())
        }
    }

    fn insert(&mut self, l1: Label<'a>, v1: u8) {
        let hash = hash(l1) as usize;
        let mut insert = true;
        for (l2, v2) in self.data[hash].iter_mut() {
            if l1 == *l2 {
                *v2 = v1;
                insert = false;
                break;
            }
        }
        if insert {
            self.data[hash].push_back((l1, v1));
        }
    }

    fn remove(&mut self, label: Label) {
        let hash = hash(label) as usize;
        if let Some(pos) = self.data[hash].iter().position(|(l, _)| *l == label) {
            let mut split_list = self.data[hash].split_off(pos);
            split_list.pop_front();
            self.data[hash].append(&mut split_list);
        }
    }

    fn sum(&self) -> i64 {
        let mut result = 0;
        for (box_number, slots) in self.data.iter().enumerate() {
            for (slot_number, (_, focal_length )) in slots.iter().enumerate() {
                result += ((box_number + 1) * (slot_number + 1) * (*focal_length as usize)) as i64
            }
        }
        result
    }
}

pub fn part2(data: &str) -> i64 {
    let data = parse(data);
    let mut hashmap = HashMap::new();

    for datum in data.iter() {
        if datum[datum.len() - 2] == '=' as u8 {
            hashmap.insert(&datum[0..datum.len() - 2], datum[datum.len() - 1] - 48);
        } else {
            hashmap.remove(&datum[0..datum.len() - 1]);
        }
    }

    hashmap.sum()
}