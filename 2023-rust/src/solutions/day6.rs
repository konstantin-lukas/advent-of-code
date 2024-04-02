use crate::utils::load_data;

pub fn run(test: bool) -> (u32, u32) {
    let data = load_data(6, test);
    let data: Vec<_> = data.split("\n").collect();
    let times: Vec<_> = data[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<f32>().unwrap())
        .collect();
    let record_distances: Vec<_> = data[1]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<f32>().unwrap())
        .collect();
    assert_eq!(times.len(), record_distances.len());
    let (mut part1, mut part2) = (1, 0);

    // PART 1
    for (time, record_distance) in times.iter().zip(record_distances.iter()) {
        let y = *time;
        let z = *record_distance + 1.0;
        // Solve x * (y - x) > z for x
        let x = ((y / 2.0) + ((y.powi(2) / 4.0) - z).sqrt()).floor() as u32;
        let solution_count = (y as u32 + 1) - ((y as u32 - x) * 2);
        part1 *= solution_count;
    }

    // PART 2
    let y = data[0]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<f64>()
        .unwrap();
    let z = data[1]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<f64>()
        .unwrap() + 1.0;
    let x = ((y / 2.0) + ((y.powi(2) / 4.0) - z).sqrt()).floor() as u32;
    part2 = (y as u32 + 1) - ((y as u32 - x) * 2);

    (part1, part2)
}