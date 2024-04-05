struct Mapping {
    from: i64,
    to: i64,
    length: i64
}

fn parse_data(data: &str) -> (Vec<i64>, [Vec<Mapping>; 7]) {
    let data = data.split("\n\n")
        .collect::<Vec<_>>();
    let mut maps: [Vec<Mapping>; 7] = Default::default();
    let mut seeds: Vec<i64> = vec![];
    for (i, datum) in data.iter().enumerate() {
        let data = datum.split_once(':').unwrap().1.trim();
        if i == 0 {
            for datum in data.split_whitespace() {
                if datum != "" {
                    if let Ok(datum) = datum.parse::<i64>() {
                        seeds.push(datum);
                    }
                }
            }
        } else {
            for datum in data.lines() {
                if datum != "" {
                    let mut data = datum.split_whitespace();
                    let to = data.next().unwrap().parse().unwrap();
                    let from = data.next().unwrap().parse().unwrap();
                    let length = data.next().unwrap().parse().unwrap();
                    maps[i - 1].push(Mapping {
                        from,
                        to,
                        length
                    });
                }
            }
        }
    }
    (seeds, maps)
}

fn map_seed(seed: i64, map: &Vec<Mapping>) -> i64 {
    for value in map {
        if seed >= value.from && seed < value.from + value.length {
            return seed + value.to - value.from;
        }
    }
    seed
}

pub fn part1(data: &str) -> i64 {
    let data = data.replace("\r\n", "\n");
    let (seeds, maps) = parse_data(&data);

    let mut locations: Vec<i64> = vec![];
    for seed in seeds {
        let mut seed = seed;
        for map in &maps {
            seed = map_seed(seed, map);
        }
        locations.push(seed);
    }
    *locations.iter().min().unwrap()
}

pub fn part2(data: &str) -> i64 {
    let data = data.replace("\r\n", "\n");
    let (seeds, maps) = parse_data(&data);
    let mut seeds = seeds
        .chunks(2)
        .map(|chunk| {
            (chunk[0], chunk[0] + chunk[1])
        }).collect::<Vec<_>>();

    for map in &maps {
        let mut overlaps: Vec<(i64, i64)> = vec![];
        while seeds.len() > 0 {
            let (seed_start, seed_end) = seeds.pop().expect("Couldn't pop");
            let mut found_overlap = false;
            for rule in map {
                let rule_start = rule.from;
                let rule_end = rule.from + rule.length;
                let rule_dest = rule.to;
                let overlap_start = seed_start.max(rule_start);
                let overlap_end = seed_end.min(rule_end);
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

    seeds.iter().min_by_key(|(first, _)| first).unwrap().0
}