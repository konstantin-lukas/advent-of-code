use crate::utils::load_data;
use regex::Regex;

///!
///! # WARNING
///! This part of the code was originally written in Python and was just moved over to Rust.
///! Code quality might not be optimal
///!


fn parse_number(num: &str) -> String {
    let digit_names = [
        ("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"),
        ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")
    ];

    for &(name, digit) in digit_names.iter() {
        if num == name {
            return digit.to_string();
        }
    }

    num.to_string()
}

pub fn run() -> (i64, i64) {
    let data = load_data(1);
    let data = data.split('\n');
    let mut part1 = 0;
    let mut part2 = 0;
    let digit_regex = Regex::new("\\d").unwrap();
    let number_word_regex = Regex::new(
        "(\\d|one|two|three|four|five|six|seven|eight|nine)"
    ).unwrap();
    let number_word_regex_reverse = Regex::new(
        "(\\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)"
    ).unwrap();

    for datum in data {
        if !datum.is_empty() {
            let first = digit_regex.find_iter(datum).next().unwrap();
            let last = digit_regex.find_iter(datum).last().unwrap();
            let mut s = String::from(datum.get(first.start()..first.end()).unwrap());
            s.push_str(datum.get(last.start()..last.end()).unwrap());
            part1 += s.parse::<i64>().unwrap();
            let first = number_word_regex.find_iter(datum).next().unwrap();
            let rev = datum.chars().rev().collect::<String>();
            let last = number_word_regex_reverse.find_iter(rev.as_str()).next().unwrap();
            let mut s = parse_number(datum.get(first.start()..first.end()).unwrap());
            let combined: String = rev.get(last.start()..last.end()).unwrap().chars().rev().collect();
            s.push_str(parse_number(combined.as_str()).as_str());
            if let Ok(x) = s.parse::<i64>() {
                part2 += x;
            }
        }
    }
    (part1, part2)
}