fn get_column_value(pattern: &Vec<Vec<char>>, previous_value: i64) -> i64 {
    let mut result = 0;

    for i in 0..pattern[0].len() - 1 {
        if pattern.iter().enumerate().all(|(j, x)| x[i] == pattern[j][i + 1]) {
            let mut l = i;
            let mut r = i + 1;
            let mut is_mirror_line = true;
            while l > 0 && r < pattern[0].len() - 1 {
                l -= 1;
                r += 1;
                if pattern.iter().enumerate().any(|(j, x)| x[l] != pattern[j][r]) {
                    is_mirror_line = false;
                    break;
                }
            }
            if is_mirror_line && (i + 1) as i64 != previous_value {
                result = (i + 1) as i64;
                break;
            }
        }
    }

    result
}

fn get_row_value(pattern: &Vec<Vec<char>>, previous_value: i64) -> i64 {
    let mut result = 0;

    for i in 0..pattern.len() - 1 {
        if pattern[i] == pattern[i + 1] {
            let mut l = i;
            let mut r = i + 1;
            let mut is_mirror_line = true;
            while l > 0 && r < pattern.len() - 1 {
                l -= 1;
                r += 1;
                if pattern[l] != pattern[r] {
                    is_mirror_line = false;
                    break;
                }
            }
            if is_mirror_line && 100 * (i + 1) as i64 != previous_value {
                result = (i + 1) as i64;
                break;
            }
        }
    }

    result * 100
}

fn parse(data: &str) -> Vec<Vec<Vec<char>>> {
    data.split("\n\n")
        .map(|x| x.lines().map(|y| y.chars().collect::<Vec<_>>()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn part1(data: &str) -> i64 {
    let data = data.replace("\r\n", "\n");
    let data = parse(data.as_str());

    let mut result = 0;
    for datum in data {
        result += get_column_value(&datum, -1);
        result += get_row_value(&datum, -1);
    }
    result
}

pub fn part2(data: &str) -> i64 {
    let data = data.replace("\r\n", "\n");
    let mut data = parse(data.as_str());

    let mut result = 0;

    for datum in data.iter_mut() {
        let row_reflection = get_row_value(&datum, -1);
        let column_reflection = get_column_value(&datum, -1);

        'label: for i in 0..datum.len() {
            for j in 0..datum[i].len() {
                let original_char = datum[i][j];
                datum[i][j] = if original_char == '.' { '#' } else { '.' };

                let new_row_reflection = get_row_value(&datum, row_reflection);
                let new_column_reflection = get_column_value(&datum, column_reflection);
                let mut found = false;
                if new_row_reflection != row_reflection && new_row_reflection > 0 {
                    result += new_row_reflection;
                    found = true;
                }
                if new_column_reflection != column_reflection && new_column_reflection > 0 {
                    result += new_column_reflection;
                    found = true;
                }
                if found {
                    break 'label;
                }

                datum[i][j] = original_char;
            }
        }
    }

    result
}