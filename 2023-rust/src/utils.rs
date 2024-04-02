use std::fs;
use std::time::{Instant};
use crate::solutions;

pub fn load_data(day: i8, test: bool) -> String {
    if test {
        return fs::read_to_string(format!("example/day{day}")).expect("Cannot load file.");
    }
    fs::read_to_string(format!("data/day{day}")).expect("Cannot load file.")
}

pub fn time() {

    let readme = fs::read_to_string("../README.md").unwrap();
    let split: Vec<_> = readme.split("### 2023 (Rust)\n").collect();
    assert_eq!(split.len(), 2);
    let mut new_readme = String::from(split[0]);
    new_readme.push_str("### 2023 (Rust)\n| Day | Fastest Time | Code |\n|---|---|---|\n");
    let rows: Vec<_> = split[1].split('\n').skip(2).collect();


    for day in 1..=25 {
        let start = Instant::now();

        let result = match day {
            1 => Ok(solutions::day1::run(false)),
            2 => Ok(solutions::day2::run(false)),
            3 => Ok(solutions::day3::run(false)),
            4 => Ok(solutions::day4::run(false)),
            5 => Ok(solutions::day5::run(false)),
            6 => Ok(solutions::day6::run(false)),
            _ => Err(()),
        };
        if let Ok(_) = result {
            let elapsed = start.elapsed();

            let row_exists = rows
                .get(day - 1)
                .unwrap_or(&"")
                .split('|')
                .collect::<Vec<_>>()
                .get(1)
                .unwrap_or(&"0")
                .trim()
                .parse::<usize>()
                .unwrap() == day;

            let previous_time = if row_exists {
                rows[day - 1]
                    .split("<!-- ")
                    .collect::<Vec<_>>()
                    .get(1)
                    .unwrap_or(&"")
                    .split(" -->")
                    .collect::<Vec<_>>()
                    .get(0)
                    .unwrap_or(&"")
                    .parse::<u128>()
                    .unwrap_or_else(|_| u128::MAX)
            } else {
                u128::MAX
            };



            if !row_exists || row_exists && previous_time > elapsed.as_nanos() {
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
                new_readme.push_str(rows[day - 1]);
                new_readme.push('\n');
            }
        }


    }

    let split: Vec<_> = split[1].split("<!-- EOT -->").collect();
    assert!(split.len() >= 2);
    new_readme.push_str("<!-- EOT -->");
    new_readme.push_str(split[1]);
    fs::write("../README.md", new_readme).unwrap();

}