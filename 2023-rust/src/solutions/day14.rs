use std::collections::HashMap;

fn parse(data: &str) -> Vec<Vec<char>> {
    data
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn transpose(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let len = data[0].len();
    let mut iters: Vec<_> = data.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part1(data: &str) -> i64 {
    let data = parse(data);
    let data = transpose(data);
    let mut result = 0;
    for datum in data.iter() {
        let mut limit: i64 = -1;
        for (i, ch) in datum.iter().enumerate() {
            if *ch == 'O' {
                let mut j = i;
                while j as i64 > limit + 1 && datum[j - 1] != '#' {
                    j -= 1;
                }
                result += (datum.len() - j) as i64;
                limit = j as i64;
            }
        }
    }
    result
}

fn slide_north(data: &mut Vec<Vec<char>>) {
    for datum in data.iter_mut() {
        let mut limit: i64 = -1;
        for i in 0..datum.len() {
            if datum[i] == 'O' {
                let mut j = i;
                while j as i64 > limit + 1 && datum[j - 1] != '#' {
                    j -= 1;
                }
                datum[i] = '.';
                datum[j] = 'O';
                limit = j as i64;
            }
        }
    }
}

fn full_rotation(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // SLIDE NORTH
    let mut data = transpose(data);
    slide_north(&mut data);

    // SLIDE WEST
    let mut data = transpose(data);
    slide_north(&mut data);

    // SLIDE SOUTH
    let data = data.into_iter().rev().collect::<Vec<_>>();
    let mut data = transpose(data);
    slide_north(&mut data);

    // SLIDE EAST
    let data = data.into_iter().rev().collect::<Vec<_>>();
    let mut data = transpose(data);
    slide_north(&mut data);
    data
        .into_iter()
        .map(|x| x.into_iter().rev().collect::<Vec<_>>())
        .rev()
        .collect::<Vec<_>>()
}

pub fn part2(data: &str) -> i64 {
    let mut data = parse(data);

    let mut seen = HashMap::new();
    let mut loop_len = 0;
    let mut loop_start = 0;
    for i in 0..1000000000i64 {
        let rotated = full_rotation(data.clone());
        if let Some(j) = seen.remove(&rotated) {
            loop_len = i - j;
            loop_start = j;
            break;
        }
        seen.insert(rotated.clone(), i);
        data = rotated;
    }

    let loops_left = (1000000000 % loop_len) + (loop_len - loop_start);

    for _ in 0..loops_left {
        data = full_rotation(data);
    }

    let mut result = 0;
    for (i, datum) in data.iter().enumerate() {
        result += datum.iter().filter(|&&x| x == 'O').count() * (data.len() - i);
    }

    result as i64
}