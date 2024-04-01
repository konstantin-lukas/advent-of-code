use std::cmp::{max, min};
use crate::utils::load_data;

///!
///! # WARNING
///! This part of the code was originally written in Python and was just moved over to Rust.
///! Code quality might not be optimal
///!

fn map_seed(seed: i64, map: &Vec<(i64, i64, i64)>) -> i64 {
    for value in map {
        if seed >= value.0 && seed < value.0 + value.2 {
            return seed + value.1 - value.0;
        }
    }
    return seed;
}

pub fn run(test: bool) -> (i64, i64) {
    let data = load_data(5, test);
    let data = data.split("\n\n").map(String::from);
    let mut maps: [Vec<(i64, i64, i64)>; 7] = Default::default();
    let mut seeds: Vec<i64> = vec![];
    let mut locations: Vec<i64> = vec![];
    for (i, datum) in data.enumerate() {
        let data = datum.split(":").collect::<Vec<_>>();
        if i == 0 {
            let data = data[1].split(" ");
            for datum in data {
                if datum != "" {
                    if let Ok(datum) = datum.parse::<i64>() {
                        seeds.push(datum);
                    }
                }
            }
        } else {
            let data = data[1].split("\n");
            for datum in data {
                if datum != "" {
                    let data = datum.split(" ").collect::<Vec<_>>();
                    let dest = data[0].parse::<i64>().unwrap();
                    let src = data[1].parse::<i64>().unwrap();
                    let len = data[2].parse::<i64>().unwrap();
                    maps[i - 1].push((src, dest, len));
                }
            }
        }
    }

    // PART 1
    for seed in &seeds {
        let mut seed = *seed;
        for map in &maps {
            seed = map_seed(seed, map);
        }
        locations.push(seed);
    }
    let minimum = locations.iter().min().unwrap();

    // PART 2
    let mut seeds: Vec<(i64, i64)> = seeds
        .chunks(2)
        .map(|chunk| {
            (chunk[0], chunk[0] + chunk[1])
        }).collect();
    for map in &maps {
        let mut overlaps: Vec<(i64, i64)> = vec![];
        while seeds.len() > 0 {
            let (seed_start, seed_end) = seeds.pop().expect("Couldn't pop");
            let mut found_overlap = false;
            for rule in map {
                let rule_start = rule.0;
                let rule_end = rule.0 + rule.2;
                let rule_dest = rule.1;
                let overlap_start = max(seed_start, rule_start);
                let overlap_end = min(seed_end, rule_end);
                if overlap_start < overlap_end {
                    overlaps.push(
                        (overlap_start + rule_dest - rule_start, overlap_end + rule_dest - rule_start)
                    );
                    if overlap_start > seed_start {
                        seeds.push((seed_start, overlap_start));
                    }
                    if overlap_end < seed_end {
                        seeds.push((overlap_end, seed_end));
                    }
                    found_overlap = true;
                    break;
                }
            }
            if !found_overlap {
                overlaps.push((seed_start, seed_end));
            }
        }
        seeds = overlaps;
    }

    let min_tuple = seeds.iter().min_by_key(|&(first, _)| first).unwrap();


    (*minimum, min_tuple.0)

}