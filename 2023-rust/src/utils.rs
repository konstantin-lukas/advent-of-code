use std::collections::HashMap;
use std::fs;
use std::time::{Instant};
use crate::solutions;
use fancy_regex::Regex;

#[cfg(not(test))]
pub fn load_data(day: i8) -> String {
    fs::read_to_string(format!("data/day{day}")).expect("Cannot load file.")
}
#[cfg(test)]
pub fn load_data(day: i8) -> String {
    return fs::read_to_string(format!("example/day{day}")).expect("Cannot load file.");
}

pub fn time() {
    return;

    let readme = fs::read_to_string("../README.md").unwrap();
    let split: Vec<_> = readme.split("<!-- SOT2023 -->\n").collect();
    assert_eq!(split.len(), 2);

    let mut new_readme = String::from(split[0]);
    let split: Vec<_> = split[1].split("<!-- EOT2023 -->\n").collect();
    assert_eq!(split.len(), 2);

    new_readme.push_str("<!-- SOT2023 -->\n| Day | Fastest Time | Code |\n|---|---|---|\n");
    let mut rows: HashMap<u32, (u128, &str)> = HashMap::new();
    let row_regex = Regex::new("\\| \\d \\| .* <!-- \\d* --> \\| .* \\|").unwrap();
    let comment_regex = Regex::new("(?<=<!--)([^-]*?)(?=-->)").unwrap();
    for row in split[0].split('\n').skip(2) {
        if row_regex.is_match(row).unwrap() {
            let columns: Vec<_> = row.split('|').filter(|x| return !x.is_empty()).collect();
            let day = columns[0].trim().parse::<u32>().unwrap();
            let time = comment_regex.find(columns[1]).unwrap();
            if let Some(time) = time {
                let time = columns[1][time.start()..time.end()].trim().parse::<u128>().unwrap();
                rows.insert(day, (time, row));
            }
        }
    }
    for day in 1u32..=25 {
        let start = Instant::now();
        let result = match day {
            1 => Ok(solutions::day1::run()),
            2 => Ok(solutions::day2::run()),
            3 => Ok(solutions::day3::run()),
            4 => Ok(solutions::day4::run()),
            5 => Ok(solutions::day5::run()),
            6 => Ok(solutions::day6::run()),
            _ => Err(()),
        };
        let elapsed = start.elapsed();

        if let Ok(_) = result {

            let row_exists = rows.contains_key(&day);


            if !row_exists || row_exists && rows[&day].0 > elapsed.as_nanos() {
                let time_str = if elapsed.as_secs() > 0 {
                    format!("{}s <!-- {} -->", elapsed.as_secs(),elapsed.as_nanos())
                } else if elapsed.as_millis() > 0 {
                    format!("{}.{}ms <!-- {} -->", elapsed.as_millis(), elapsed.subsec_millis(),elapsed.as_nanos())
                } else if elapsed.as_micros() > 0 {
                    format!("{}.{}μs <!-- {} -->", elapsed.as_micros(), elapsed.subsec_micros(),elapsed.as_nanos())
                } else {
                    format!("{}.{}ns <!-- {} -->", elapsed.as_nanos(), elapsed.subsec_nanos(),elapsed.as_nanos())
                };
                new_readme.push_str(&format!("| {} | {} | [day{}.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day{}.rs) |\n", day, time_str, day, day));
            } else {
                new_readme.push_str(rows[&day].1);
                new_readme.push('\n');
            }
        }


    }

    assert!(split.len() >= 2);
    new_readme.push_str("<!-- EOT2023 -->\n");
    new_readme.push_str(split[1]);
    fs::write("../README.md", new_readme).unwrap();

}