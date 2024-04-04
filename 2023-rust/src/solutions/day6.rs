use crate::utils::load_data;


/// Solves x * (y - x) > z for x and calculates all winning strategies
fn calc_solution(y: f64, z: f64) -> i64 {
    let x = ((y / 2.0) + ((y.powi(2) / 4.0) - z).sqrt()).floor() as i64;
    (y as i64 + 1) - ((y as i64 - x) * 2)
}


pub fn part1() {

}


pub fn run() -> (i64, i64) {
    let data = load_data(6);
    let data: Vec<_> = data.split("\n").collect();
    let times: Vec<_> = data[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<f64>().unwrap())
        .collect();
    let record_distances: Vec<_> = data[1]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<f64>().unwrap())
        .collect();
    assert_eq!(times.len(), record_distances.len());
    let mut part1 = 1;

    // PART 1
    for (time, record_distance) in times.iter().zip(record_distances.iter()) {
        let y = *time;
        let z = *record_distance + 1.0;
        part1 *= calc_solution(y, z);
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
    let part2 = calc_solution(y, z);

    (part1, part2)
}