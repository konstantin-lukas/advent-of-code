pub fn part1(data: &str) -> i64 {
    let data = data.split('\n');
    let mut solution = 0;
    for line in data {
        //let (mut reds, mut greens, mut blues) = (0, 0, 0);
        let mut i = line.len();
        let possible = loop {
            let slice = &line[0..i];
            let (keyword, limit) =
                if slice.chars().last().unwrap() == 'd' { (4, 12) }
                else if slice.chars().last().unwrap() == 'n' { (6, 13) }
                else { (5, 14) };
            i -= keyword;
            let mut j = i;
            while line[0..j].chars().last().unwrap().is_numeric() { j -= 1; }
            let count = line[j..i].parse::<i64>().unwrap();
            if count > limit { break None; }
            i = j;
            i -= 2;
            if line[0..j].ends_with(": ") {
                break Some(i);
            }
        };
        if let Some(i) = possible {
            solution += line[5..i].parse::<i64>().unwrap();
        }
    }
    solution
}

pub fn part2(data: &str) -> i64 {
    let data = data.split('\n');
    let mut solution = 0;
    for line in data {
        let (mut reds, mut greens, mut blues) = (0, 0, 0);
        let mut i = line.len();
        loop {
            let slice = &line[0..i];
            let keyword =
                if slice.chars().last().unwrap() == 'd' { 4 }
                else if slice.chars().last().unwrap() == 'n' { 6 }
                else { 5 };
            i -= keyword;
            let mut j = i;
            while line[0..j].chars().last().unwrap().is_numeric() { j -= 1; }
            let count = line[j..i].parse::<i64>().unwrap();

            if keyword == 4 && count > reds { reds = count; }
            else if keyword == 5 && count > blues { blues = count; }
            if keyword == 6 && count > greens { greens = count; }
            i = j;
            i -= 2;
            if line[0..j].ends_with(": ") {
                break;
            }
        }
        solution += reds * greens * blues;
    }
    solution
}