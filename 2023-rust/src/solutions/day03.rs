pub fn part1(data: &str) -> i64 {
    let data: Vec<_> = data.lines().collect();
    let mut solution = 0;
    for (i, line) in data.iter().enumerate() {
        let mut number_start = 0;
        for (j, char) in line.char_indices() {
            if char.is_numeric() {
                if j == 0 || !line.chars().nth(j - 1).unwrap().is_numeric() {
                    number_start = j;
                }
                if j == line.len() - 1 || !line.chars().nth(j + 1).unwrap().is_numeric() {
                    let mut number_counts = false;

                    if number_start > 0 {
                        let x = line.chars().nth(number_start - 1).unwrap();
                        if !x.is_numeric() && x != '.' {
                            number_counts = true;
                        }
                    }
                    if !number_counts && j < line.len() - 1 {
                        let x = line.chars().nth(j + 1).unwrap();
                        if !x.is_numeric() && x != '.' {
                            number_counts = true;
                        }
                    }
                    if !number_counts && i > 0 {
                        let mut start = number_start;
                        let mut end = j + 1;
                        if start > 0 { start = start - 1; }
                        if end < data[i - 1].len() { end += 1; }
                        for char in data[i - 1][start..end].chars() {
                            if !char.is_numeric() && char != '.' {
                                number_counts = true;
                                break;
                            }
                        }
                    }
                    if !number_counts && i < data.len() - 1 {
                        let mut start = number_start;
                        let mut end = j + 1;
                        if start > 0 { start = start - 1; }
                        if end < data[i + 1].len() { end += 1; }
                        for char in data[i + 1][start..end].chars() {
                            if !char.is_numeric() && char != '.' {
                                number_counts = true;
                                break;
                            }
                        }
                    }
                    if number_counts {
                        solution += line[number_start..j + 1].parse::<i64>().unwrap();
                    }


                }
            }
        }

    }
    solution
}

pub fn part2(data: &str) -> i64 {
    let data: Vec<_> = data.lines().collect();
    let mut solution = 0;
    let mut numbers = vec![];
    for (i, line) in data.iter().enumerate() {
        let mut _number_start = 0;
        for (j, char) in line.char_indices() {
            if char == '*' {
                numbers.clear();

                // CHECK NUMBER BEFORE ASTERISK
                let mut k = j;
                while k > 0 {
                    k -= 1;
                    if !line.chars().nth(k).unwrap().is_numeric() { break; }
                }
                if j - k > 1 { numbers.push(line[k + 1..j].parse::<i64>().unwrap()); }

                // CHECK NUMBER AFTER ASTERISK
                let mut k = j;
                while k < line.len() + 1 {
                    k += 1;
                    if !line.chars().nth(k).unwrap().is_numeric() { break; }
                }
                if k - j > 1 { numbers.push(line[j + 1..k].parse::<i64>().unwrap()); }

                // CHECK NUMBERS ABOVE/BELOW ASTERISK
                for d in[i - 1, i + 1] {
                    if numbers.len() < 3 &&  d == i - 1 && i > 0 || d == i + 1 && i < data.len() - 1 {
                        let (mut start, mut end) = (j, j);
                        while start > 0 && data[d].chars().nth(start - 1).unwrap().is_numeric() {
                            start -= 1;
                        }
                        while end < data[d].len() - 1 && data[d].chars().nth(end + 1).unwrap().is_numeric() {
                            end += 1;
                        }
                        // POTENTIALLY TWO ADJACENT NUMBERS
                        if !data[d].chars().nth(j).unwrap().is_numeric() {
                            // POTENTIAL NUMBER TO THE TOP LEFT
                            if j > 0 {
                                if j > start { numbers.push(data[d][start..j].parse::<i64>().unwrap()); }
                            }
                            // POTENTIAL NUMBER TO THE TOP RIGHT
                            if j < data[d].len() {
                                if j < end { numbers.push(data[d][j + 1..end + 1].parse::<i64>().unwrap()); }
                            }
                        } else { // NO MORE THAN ONE ADJACENT NUMBER
                            numbers.push(data[d][start..end + 1].parse::<i64>().unwrap());
                        }
                    }
                }

                if numbers.len() == 2 {
                    solution += numbers.iter().product::<i64>();
                }
            }
        }

    }
    solution
}