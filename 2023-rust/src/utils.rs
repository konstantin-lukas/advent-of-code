use std::collections::HashMap;
use std::fs;
use std::time::{Duration, Instant};
use crate::solutions;
use fancy_regex::Regex;

#[cfg(not(test))]
pub fn load_data(day: u8) -> String {
    let data = fs::read_to_string(format!("data/day{:02}", day))
        .expect("Cannot load file.");
    return data;
}
#[cfg(test)]
pub fn load_data(day: u8) -> String {
    let data = fs::read_to_string(format!("example/day{:02}", day))
        .expect("Cannot load file.");
    return data;
}

fn get_time_string(elapsed: &Duration) -> String {
    let nanos = elapsed.as_nanos();
    let time_str = if nanos >= 1_000_000_000 {
        let mut decimals = (nanos % 1_000_000_000).to_string();
        while decimals.len() > 3 || decimals.ends_with('0') && decimals.len() > 1 { decimals.pop(); }
        format!("{}.{}s <!-- {} -->", nanos / 1_000_000_000, decimals, nanos)
    } else if nanos >= 1_000_000 {
        let mut decimals = (nanos % 1_000_000).to_string();
        while decimals.len() > 3 || decimals.ends_with('0') && decimals.len() > 1 { decimals.pop(); }
        format!("{}.{}ms <!-- {} -->", nanos / 1_000_000, decimals, nanos)
    } else if nanos >= 1_000 {
        let mut decimals = (nanos % 1_000).to_string();
        while decimals.len() > 3 || decimals.ends_with('0') && decimals.len() > 1 { decimals.pop(); }
        format!("{}.{}μs <!-- {} -->", nanos / 1_000, decimals, nanos)
    } else {
        format!("{}ns <!-- {} -->", elapsed.as_nanos(), nanos)
    };

    time_str
}

pub fn benchmark() {

    let readme = fs::read_to_string("../README.md").unwrap().replace("\r\n","\n");
    let split: Vec<_> = readme.split("<!-- SOT2023 -->\n").collect();
    assert_eq!(split.len(), 2);

    let mut new_readme = String::from(split[0]);
    let split: Vec<_> = split[1].split("<!-- EOT2023 -->\n").collect();
    assert_eq!(split.len(), 2);

    new_readme.push_str("<!-- SOT2023 -->\n| Day | Best Time Part 1 | Best Time Part 2 | Code |\n|---|---|---|---|\n");
    let mut rows: HashMap<u8, (u128, u128, &str)> = HashMap::new();
    let row_regex = Regex::new("\\| \\d+ \\| .* <!-- \\d* --> \\| .* <!-- \\d* --> \\| .* \\|").unwrap();
    let comment_regex = Regex::new("(?<=<!--)([^-]*?)(?=-->)").unwrap();
    for row in split[0].split('\n').skip(2) {
        if row_regex.is_match(row).unwrap() {
            let columns: Vec<_> = row.split('|').filter(|x| return !x.is_empty()).collect();
            let day = columns[0].trim().parse::<u8>().unwrap();
            let time1 = comment_regex.find(columns[1]).unwrap();
            let time2 = comment_regex.find(columns[2]).unwrap();
            if let (Some(time1), Some(time2)) = (time1, time2) {
                let time1 = columns[1][time1.start()..time1.end()].trim().parse::<u128>().unwrap();
                let time2 = columns[2][time2.start()..time2.end()].trim().parse::<u128>().unwrap();
                rows.insert(day, (time1, time2, row));
            }
        }
    }
    for day in 1u8..=25 {
        let data = load_data(day);
        let start = Instant::now();
        let time1 = match match day {
            1 => Some(solutions::day01::part1(data.as_str())),
            2 => Some(solutions::day02::part1(data.as_str())),
            3 => Some(solutions::day03::part1(data.as_str())),
            4 => Some(solutions::day04::part1(data.as_str())),
            5 => Some(solutions::day05::part1(data.as_str())),
            6 => Some(solutions::day06::part1(data.as_str())),
            7 => Some(solutions::day07::part1(data.as_str())),
            8 => Some(solutions::day08::part1(data.as_str())),
            9 => Some(solutions::day09::part1(data.as_str())),
            10 => Some(solutions::day10::part1(data.as_str())),
            11 => Some(solutions::day11::part1(data.as_str())),
            12 => Some(solutions::day12::part1(data.as_str())),
            13 => Some(solutions::day13::part1(data.as_str())),
            14 => Some(solutions::day14::part1(data.as_str())),
            15 => Some(solutions::day15::part1(data.as_str())),
            16 => Some(solutions::day16::part1(data.as_str())),
            17 => Some(solutions::day17::part1(data.as_str())),
            18 => Some(solutions::day18::part1(data.as_str())),
            19 => Some(solutions::day19::part1(data.as_str())),
            20 => Some(solutions::day20::part1(data.as_str())),
            21 => Some(solutions::day21::part1(data.as_str())),
            22 => Some(solutions::day22::part1(data.as_str())),
            23 => Some(solutions::day23::part1(data.as_str())),
            _ => None,
        } {
            Some(_) => Some(start.elapsed()),
            None => None
        };
        let start = Instant::now();
        let time2 = match match day {
            1 => Some(solutions::day01::part2(data.as_str())),
            2 => Some(solutions::day02::part2(data.as_str())),
            3 => Some(solutions::day03::part2(data.as_str())),
            4 => Some(solutions::day04::part2(data.as_str())),
            5 => Some(solutions::day05::part2(data.as_str())),
            6 => Some(solutions::day06::part2(data.as_str())),
            7 => Some(solutions::day07::part2(data.as_str())),
            8 => Some(solutions::day08::part2(data.as_str())),
            9 => Some(solutions::day09::part2(data.as_str())),
            10 => Some(solutions::day10::part2(data.as_str())),
            11 => Some(solutions::day11::part2(data.as_str())),
            12 => Some(solutions::day12::part2(data.as_str())),
            13 => Some(solutions::day13::part2(data.as_str())),
            14 => Some(solutions::day14::part2(data.as_str())),
            15 => Some(solutions::day15::part2(data.as_str())),
            16 => Some(solutions::day16::part2(data.as_str())),
            17 => Some(solutions::day17::part2(data.as_str())),
            18 => Some(solutions::day18::part2(data.as_str())),
            19 => Some(solutions::day19::part2(data.as_str())),
            20 => Some(solutions::day20::part2(data.as_str())),
            21 => Some(solutions::day21::part2(data.as_str())),
            22 => Some(solutions::day22::part2(data.as_str())),
            23 => Some(solutions::day23::part2(data.as_str())),
            _ => None,
        } {
            Some(_) => Some(start.elapsed()),
            None => None
        };

        if let (Some(time1), Some(time2)) = (time1, time2) {
            let row_exists = rows.contains_key(&day);
            if !row_exists || row_exists && rows[&day].0 > time1.as_nanos() || row_exists && rows[&day].0 > time2.as_nanos() {
                let time_string1 = if !row_exists || rows[&day].0 > time1.as_nanos() {
                    get_time_string(&time1)
                } else {
                    let columns: Vec<_> = rows[&day].2.split('|').filter(|x| return !x.is_empty()).collect();
                    columns[1].trim().to_string()
                };
                let time_string2 = if !row_exists || rows[&day].1 > time2.as_nanos() {
                    get_time_string(&time2)
                } else {
                    let columns: Vec<_> = rows[&day].2.split('|').filter(|x| return !x.is_empty()).collect();
                    columns[2].trim().to_string()
                };
                new_readme.push_str(&format!("| {} | {} | {} | [day{:02}.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day{:02}.rs) |\n", day, time_string1, time_string2, day, day));
            } else {
                new_readme.push_str(rows[&day].2);
                new_readme.push('\n');
            }
        }

    }

    assert!(split.len() >= 2);
    new_readme.push_str("<!-- EOT2023 -->\n");
    new_readme.push_str(split[1]);
    fs::write("../README.md", new_readme).unwrap();

}