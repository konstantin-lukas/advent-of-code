use crate::utils::load_data;

///!
///! # WARNING
///! This part of the code was originally written in Python and was just moved over to Rust.
///! Code quality might not be optimal
///!

fn get_correct_numbers(numbers: &(Vec<i64>, Vec<i64>)) -> i64 {
    let mut c = 0;
    for holding in &numbers.1 {
        if numbers.0.contains(holding) {
            c += 1;
        }
    }
    c
}

fn get_points(correct_numbers: i64) -> i64 {
    if correct_numbers <= 1 {
        return correct_numbers;
    }
    return 2i64.pow(correct_numbers as u32 - 1)
}

fn get_scratchcards(t: &(Vec<i64>, Vec<i64>), i: usize, d: &Vec<(Vec<i64>, Vec<i64>)>) -> i64 {
    let mut c = get_correct_numbers(t);
    for idx in i + 1..i + usize::try_from(c).unwrap() + 1 {
        c += get_scratchcards(&d[idx], idx, d)
    }
    return c;
}

pub fn run() -> (i64, i64) {
    let data = load_data(4);
    let data: Vec<(Vec<i64>, Vec<i64>)> = data.split('\n').map(|x| {
        let numbers = &x[x.chars().position(|c| c == ':').unwrap() + 1..];
        let numbers: Vec<&str> = numbers
            .split('|')
            .map(|y| y.trim())
            .collect();
        let winning: Vec<i64> = numbers[0]
            .split_whitespace()
            .map(|y| y.parse::<i64>().unwrap())
            .collect();
        let holding: Vec<i64> = numbers[1]
            .split_whitespace()
            .map(|y| y.parse::<i64>().unwrap())
            .collect();
        (winning, holding)
    }).collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for (i, numbers) in data.iter().enumerate() {
        let correct_numbers = get_correct_numbers(numbers);
        part1 += get_points(correct_numbers);
        part2 += get_scratchcards(numbers, i, &data) + 1;
    }

    (part1, part2)
}