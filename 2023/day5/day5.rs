use std::fs;
use std::collections::HashMap;
fn main() {
    let data = fs::read_to_string("day5.data").expect("Unable to read file");
    let data = data.split("\n\n").map(String::from);
    let mut maps: [HashMap<i64, (i64, i64)>; 7] = Default::default();
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
                    maps[i - 1].insert(src, (dest, len));
                }
            }
        }
    }
    for seed in seeds {
        let mut seed = seed;
        'maps: for map in &maps {
            for value in map {
                let src = *(value.0);
                let dest = value.1.0;
                let len = value.1.1;
                if seed >= src && seed < src + len {
                    seed += dest - src;
                    continue 'maps;
                }
            }
        }
        locations.push(seed);
    }
    let min = locations.iter().min().unwrap();
    println!("{}", min);

}