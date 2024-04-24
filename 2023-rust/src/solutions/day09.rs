fn parse(data: &str) -> Vec<Vec<i64>> {
    data.lines()
        .map(|x| x
            .split(' ')
            .map(|x| x
                .parse::<i64>()
                .unwrap()
            )
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>()
}

pub fn part1(data: &str) -> i64 {
    let mut result = 0;
    let mut data = parse(data);

    for line in data.iter_mut() {
        let mut limit = line.len() - 1;
        result += loop {
            let mut i = 0;
            let mut sum = 0;
            while i < limit {
                line[i] = line[i + 1] - line[i];
                sum += line[i];
                i += 1;
            }
            if sum == 0 { break line.iter().sum::<i64>(); }
            limit -= 1;
        };
    }
    result
}

pub fn part2(data: &str) -> i64 {
    let mut result = 0;
    let mut data = parse(data);

    for line in data.iter_mut() {
        let mut limit = 0;
        result += loop {
            let mut i = line.len() - 1;
            let mut sum = 0;
            while i > limit {
                line[i] = -(line[i] - line[i - 1]);
                sum += line[i];
                i -= 1;
            }
            if sum == 0 {
                break line[0] + line.iter().skip(1).sum::<i64>();
            }
            limit += 1;
        };
    }
    result
}