pub fn part1(data: &str) -> i64 {
    let data = data.split('\n');
    let mut sum = 0;

    for line in data {
        let first = line.chars().find(|x| x.is_numeric());
        let last = line.chars().rfind(|x| x.is_numeric());
        if let (Some(first), Some(last)) = (first, last) {
            let number = format!("{}{}", first, last);
            sum += number.parse::<i64>().unwrap();
        }
    }

    sum
}

pub fn part2(data: &str) -> i64 {
    let data = data.split('\n');
    let mut sum = 0;

    for line in data {
        let mut first = 0;
        let mut last = 0;

        for i in 0..line.len() {
            let line = &line[i..];
            if line.starts_with("1") || line.starts_with("one") {
                first = 1;
                break;
            } else if line.starts_with("2") || line.starts_with("two") {
                first = 2;
                break;
            } else if line.starts_with("3") || line.starts_with("three") {
                first = 3;
                break;
            } else if line.starts_with("4") || line.starts_with("four") {
                first = 4;
                break;
            } else if line.starts_with("5") || line.starts_with("five") {
                first = 5;
                break;
            } else if line.starts_with("6") || line.starts_with("six") {
                first = 6;
                break;
            } else if line.starts_with("7") || line.starts_with("seven") {
                first = 7;
                break;
            } else if line.starts_with("8") || line.starts_with("eight") {
                first = 8;
                break;
            } else if line.starts_with("9") || line.starts_with("nine") {
                first = 9;
                break;
            }
        }
        for i in (0..line.len() + 1).rev() {
            let line = &line[..i];
            if line.ends_with("1") || line.ends_with("one") {
                last = 1;
                break;
            } else if line.ends_with("2") || line.ends_with("two") {
                last = 2;
                break;
            } else if line.ends_with("3") || line.ends_with("three") {
                last = 3;
                break;
            } else if line.ends_with("4") || line.ends_with("four") {
                last = 4;
                break;
            } else if line.ends_with("5") || line.ends_with("five") {
                last = 5;
                break;
            } else if line.ends_with("6") || line.ends_with("six") {
                last = 6;
                break;
            } else if line.ends_with("7") || line.ends_with("seven") {
                last = 7;
                break;
            } else if line.ends_with("8") || line.ends_with("eight") {
                last = 8;
                break;
            } else if line.ends_with("9") || line.ends_with("nine") {
                last = 9;
                break;
            }
        }
        let number = format!("{}{}", first, last);
        sum += number.parse::<i64>().unwrap();
    }

    sum
}