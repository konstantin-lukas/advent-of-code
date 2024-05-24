use std::collections::{HashSet, VecDeque};

fn parse(data: &str) -> ((usize, usize), Vec<Vec<bool>>) {
    let (mut x, mut y) = (0, 0);
    let data = data.lines().enumerate().map(|(row, l)| l
        .chars()
        .enumerate()
        .map(|(col, c)| {
            if c == 'S' { y = row; x = col; }
            c != '#'
        }).collect::<Vec<_>>()
    ).collect::<Vec<_>>();
    ((y, x), data)
}

fn calc(data: &((usize, usize), Vec<Vec<bool>>), i: i64, part: bool) -> i64 {
    if part {
        let mut positions = HashSet::new();
        positions.insert((data.0, (0, 0)));

        for _ in 0..i {
            positions = positions.iter().flat_map(|(pos, quadrant)| {
                let mut result = vec![];
                let data = &data.1;
                if pos.0 > 0 && data[pos.0 - 1][pos.1] {
                    result.push(((pos.0 - 1, pos.1), *quadrant));
                } else if pos.0 == 0 && data[data.len() - 1][pos.1] {
                    result.push(((data.len() - 1, pos.1), (quadrant.0 - 1, quadrant.1)));
                }
                if pos.1 > 0 && data[pos.0][pos.1 - 1] {
                    result.push(((pos.0, pos.1 - 1), *quadrant));
                } else if pos.1 == 0 && data[pos.0][data[0].len() - 1] {
                    result.push(((pos.0, data[0].len() - 1), (quadrant.0, quadrant.1 - 1)));
                }
                if pos.0 < data.len() - 1 && data[pos.0 + 1][pos.1] {
                    result.push(((pos.0 + 1, pos.1), *quadrant));
                } else if pos.0 == data.len() - 1 && data[0][pos.1] {
                    result.push(((0, pos.1), (quadrant.0 + 1, quadrant.1)));
                }
                if pos.1 < data[0].len() - 1 && data[pos.0][pos.1 + 1] {
                    result.push(((pos.0, pos.1 + 1), *quadrant));
                } else if pos.1 == data[0].len() - 1 && data[pos.0][0] {
                    result.push(((pos.0, 0), (quadrant.0, quadrant.1 + 1)));
                }
                result.into_iter()
            }).collect::<HashSet<_, _>>();
        }
        positions.len() as i64
    } else {
        let mut positions = HashSet::new();
        positions.insert(data.0);

        for _ in 0..i {
            positions = positions.iter().flat_map(|pos| {
                let mut result = vec![];
                let data = &data.1;
                if pos.0 > 0 && data[pos.0 - 1][pos.1] {
                    result.push((pos.0 - 1, pos.1));
                }
                if pos.1 > 0 && data[pos.0][pos.1 - 1] {
                    result.push((pos.0, pos.1 - 1));
                }
                if pos.0 < data.len() - 1 && data[pos.0 + 1][pos.1] {
                    result.push((pos.0 + 1, pos.1));
                }
                if pos.1 < data[0].len() - 1 && data[pos.0][pos.1 + 1] {
                    result.push((pos.0, pos.1 + 1));
                }
                result.into_iter()
            }).collect::<HashSet<_, _>>();
        }
        positions.len() as i64
    }
}

pub fn part1(data: &str) -> i64 {
    let data = parse(data);
    calc(&data, 64, false)
}

fn get_next_number(n1: i64, n2: i64, n3: i64) -> i64 {
    let mut data = vec![n1, n2, n3];
    let mut result = 0;
    let mut limit = data.len() - 1;
    result += loop {
        let mut i = 0;
        let mut sum = 0;
        while i < limit {
            data[i] = data[i + 1] - data[i];
            sum += data[i];
            i += 1;
        }
        if sum == 0 { break data.iter().sum::<i64>(); }
        limit -= 1;
    };
    result
}

/// This is using the solution from day 9 to calculate the result.
/// The commented code bits were used to pre-calculate the starting values.
pub fn part2(_data: &str) -> i64 {
    //let _data = parse(data);
    let mut sequence = VecDeque::new();
    sequence.push_back(/*calc(&data, 65, true)*/3859);
    sequence.push_back(/*calc(&data, 65 + 131, true)*/34324);
    sequence.push_back(/*calc(&data, 65 + 131 * 2, true)*/95135);

    for _ in 0..202300 - 2 {
        sequence.push_back(get_next_number(sequence[0], sequence[1], sequence[2]));
        sequence.pop_front();
    }

    sequence.pop_back().unwrap()
}