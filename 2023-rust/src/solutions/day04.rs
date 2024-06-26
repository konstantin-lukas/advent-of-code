

use std::collections::HashMap;

pub fn part1(data: &str) -> i64 {
    let mut solution = 0;

    for line in data.lines() {
        if let Some((winning, holding)) = line.split_once('|') {
            let count = winning
                .split_whitespace()
                .filter(|&w| holding.split_whitespace().any(|h| h == w))
                .count();

            solution += if count == 0 { 0 } else { 2i64.pow(count as u32 - 1) };
        }
    }

    solution
}


fn get_scratchcards(data: &Vec<(Vec<i64>, Vec<i64>)>, i: usize, memo: &mut HashMap<usize, i64>) -> i64 {
    if let Some(&result) = memo.get(&i) {
        return result;
    }

    let mut scratchcards = 1;

    let (winning, holding) = &data[i];

    let count = winning
        .iter()
        .filter(|&w| holding.iter().any(|h| h == w))
        .count();

    for j in i + 1..i + count + 1 {
        scratchcards += get_scratchcards(data, j, memo);
    }

    memo.insert(i, scratchcards);

    scratchcards
}

pub fn part2(data: &str) -> i64 {
    let data = data
        .lines()
        .map(|x| x
            .split_once(':')
            .unwrap()
            .1
            .split_once('|')
            .unwrap()
        ).collect::<Vec<_>>();
    let data = data
        .iter()
        .map(|(a, b)|
            (a.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>(),
             b.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>())
        ).collect::<Vec<_>>();

    let mut solution = 0;
    let mut memo: HashMap<usize, i64> = HashMap::new();

    for i in 0..data.len() {
        solution += get_scratchcards(&data, i, &mut memo);
    }
    solution
}
