use crate::utils::load_data;
use fancy_regex::Regex;

///!
///! # WARNING
///! This part of the code was originally written in Python and was just moved over to Rust.
///! Code quality might not be optimal
///!


pub fn run() -> (i64, i64) {
    let data = load_data(2);
    let data = data.split('\n');
    let max_reds = 12;
    let max_greens = 13;
    let max_blues = 14;
    let mut part1 = 0;
    let mut part2 = 0;
    let red_regex = Regex::new("\\d+(?=\\s*red)").unwrap();
    let green_regex = Regex::new("\\d+(?=\\s*green)").unwrap();
    let blue_regex = Regex::new("\\d+(?=\\s*blue)").unwrap();

    for game in data {
        if game.is_empty() { continue; }
        let split_line: Vec<&str> = game.split(": ").collect();
        let rounds = split_line[1].split(';');
        let mut game_possible = true;
        let mut least_amount_of_reds = 0;
        let mut least_amount_of_greens = 0;
        let mut least_amount_of_blues = 0;
        for round in rounds {
            let reds = red_regex.find(round).unwrap();
            let greens = green_regex.find(round).unwrap();
            let blues = blue_regex.find(round).unwrap();
            if let Some(r) = reds {
                let red_count = round.get(r.start()..r.end());
                if let Some(red_count) = red_count {
                    let red_count = red_count.parse::<i64>().unwrap();
                    if red_count > least_amount_of_reds {
                        least_amount_of_reds = red_count;
                    }
                    if red_count > max_reds {
                        game_possible = false;
                    }
                }
            }
            if let Some(g) = greens {
                let green_count = round.get(g.start()..g.end());
                if let Some(green_count) = green_count {
                    let green_count = green_count.parse::<i64>().unwrap();
                    if green_count > least_amount_of_greens {
                        least_amount_of_greens = green_count;
                    }
                    if green_count > max_greens {
                        game_possible = false;
                    }
                }
            }
            if let Some(b) = blues {
                let blue_count = round.get(b.start()..b.end());
                if let Some(blue_count) = blue_count {
                    let blue_count = blue_count.parse::<i64>().unwrap();
                    if blue_count > least_amount_of_blues {
                        least_amount_of_blues = blue_count;
                    }
                    if blue_count > max_blues {
                        game_possible = false;
                    }
                }
            }
        }
        part2 += least_amount_of_reds * least_amount_of_greens * least_amount_of_blues;
        if game_possible {
            let r = Regex::new("\\d+").unwrap();
            let m = r.find(split_line[0]).unwrap();
            if let Some(m) = m {
                part1 += split_line[0]
                    .get(m.start()..m.end())
                    .unwrap()
                    .parse::<i64>()
                    .unwrap();
            }
        }
        
    }


    (part1, part2)
}