use crate::utils::load_data;
use regex::Regex;


///!
///! # WARNING
///! This part of the code was originally written in Python and was just moved over to Rust.
///! Code quality might not be optimal
///!


pub fn run(test: bool) -> (i64, i64) {
    let data = load_data(3, test);
    let data = data.split('\n').collect::<Vec<_>>();
    let mut part1 = 0;
    let mut part2 = 0;
    for (line_index, line) in data.iter().enumerate() {
        if line.is_empty() { continue; }
        let mut current_number = String::new();
        let mut keep_number = false;

        // PART 1
        for (char_index, char) in line.chars().enumerate() {
            if char.is_numeric() {
                current_number.push(char);
                let min_char_index = if char_index == 0 { 0 } else { char_index - 1 };
                let max_char_index = if char_index == line.len() - 1 {
                    line.len() - 1
                } else {
                    char_index + 1
                };
                let special_char = Regex::new("[^0-9.]").unwrap();
                let adjacent_char_on_prev_line = line_index > 0 && (
                    special_char.is_match(&data[line_index - 1]
                        .chars()
                        .nth(min_char_index)
                        .unwrap()
                        .to_string()
                    ) || special_char.is_match(&data[line_index - 1]
                        .chars()
                        .nth(char_index)
                        .unwrap()
                        .to_string()
                    ) || special_char.is_match(&data[line_index - 1]
                        .chars()
                        .nth(max_char_index)
                        .unwrap()
                        .to_string()
                    )
                );
                let adjacent_char_on_current_line = special_char.is_match(&data[line_index]
                    .chars()
                    .nth(min_char_index)
                    .unwrap()
                    .to_string()
                ) || special_char.is_match(&data[line_index]
                    .chars()
                    .nth(char_index)
                    .unwrap()
                    .to_string()
                ) || special_char.is_match(&data[line_index]
                    .chars()
                    .nth(max_char_index)
                    .unwrap()
                    .to_string()
                );
                let adjacent_char_on_next_line = line_index < data.len() - 1 && (
                    special_char.is_match(&data[line_index + 1]
                        .chars()
                        .nth(min_char_index)
                        .unwrap()
                        .to_string()
                    ) || special_char.is_match(&data[line_index + 1]
                        .chars()
                        .nth(char_index)
                        .unwrap()
                        .to_string()
                    ) || special_char.is_match(&data[line_index + 1]
                        .chars()
                        .nth(max_char_index)
                        .unwrap()
                        .to_string()
                    )
                );
                if adjacent_char_on_prev_line || adjacent_char_on_current_line || adjacent_char_on_next_line {
                    keep_number = true;
                }
            }
            if !char.is_numeric() || char_index == line.len() - 1 {
                if keep_number && !current_number.is_empty() {
                    part1 += current_number.parse::<i64>().unwrap();
                    keep_number = false;
                }
                current_number.clear();
            }
        }

        // PART 2
        let asterisks = Regex::new("\\*").unwrap();
        let numeric = Regex::new("[0-9]+").unwrap();
        let asterisks = asterisks.find_iter(line);
        for asterisk in asterisks {
            let position = asterisk.start();
            let numbers_on_prev_line = if line_index == 0 {
                vec![]
            } else {
                numeric.find_iter(data[line_index - 1])
                    .filter(|m| (m.start() as i64 - position as i64).abs() <= 1 || (m.end() as i64 - position as i64 - 1).abs() <= 1)
                    .map(|m| data[line_index - 1][m.start()..m.end()].parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
            };
            let numbers_on_current_line = numeric.find_iter(line)
                .filter(|m| m.start() == position + 1 || m.end() == position)
                .map(|m| line[m.start()..m.end()].parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let numbers_on_next_line = if line_index == data.len() - 1 {
                vec![]
            } else {
                numeric.find_iter(data[line_index + 1])
                    .filter(|m| (m.start() as i64 - position as i64).abs() <= 1 ||(m.end() as i64 - position as i64 - 1).abs() <= 1)
                    .map(|m| data[line_index + 1][m.start()..m.end()].parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
            };
            if numbers_on_prev_line.len() + numbers_on_current_line.len() + numbers_on_next_line.len() == 2 {
                part2 += numbers_on_prev_line.iter().product::<i64>() * numbers_on_current_line.iter().product::<i64>() * numbers_on_next_line.iter().product::<i64>();
            }
        }
    }

    return (part1 as i64, part2 as i64);
}