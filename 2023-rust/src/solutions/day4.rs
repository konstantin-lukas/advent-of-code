use crate::utils::load_data;

///!
///! # WARNING
///! This part of the code was originally written in Python and was just moved over to Rust.
///! Code quality might not be optimal
///!

fn get_correct_numbers(numbers: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut c = 0;
    for holding in &numbers.1 {
        if numbers.0.contains(holding) {
            c += 1;
        }
    }
    c
}

fn get_points(correct_numbers: u32) -> u32 {
    if correct_numbers <= 1 {
        return correct_numbers;
    }
    return 2u32.pow(correct_numbers - 1)
}

fn get_scratchcards(t: &(Vec<u32>, Vec<u32>), i: usize, d: &Vec<(Vec<u32>, Vec<u32>)>) -> u32 {
    let mut c = get_correct_numbers(t);
    for idx in i + 1..i + usize::try_from(c).unwrap() + 1 {
        c += get_scratchcards(&d[idx], idx, d)
    }
    return c;
}

pub fn run(test: bool) -> (u32, u32) {
    let data = load_data(4, test);
    let data: Vec<(Vec<u32>, Vec<u32>)> = data.split('\n').map(|x| {
        let numbers = &x[x.chars().position(|c| c == ':').unwrap() + 1..];
        let numbers: Vec<&str> = numbers
            .split('|')
            .map(|y| y.trim())
            .collect();
        let winning: Vec<u32> = numbers[0]
            .split_whitespace()
            .map(|y| y.parse::<u32>().unwrap())
            .collect();
        let holding: Vec<u32> = numbers[1]
            .split_whitespace()
            .map(|y| y.parse::<u32>().unwrap())
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