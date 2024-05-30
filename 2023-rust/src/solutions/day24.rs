type Coordinates = (i128, i128, i128);

#[derive(Debug)]
struct Hailstone {
    pos: Coordinates,
    vel: Coordinates,
    line: Coordinates
}

fn parse(data: &str) -> Vec<Hailstone> {
    data.lines().map(|x| {
        let (pos, vel) = x.split_once('@').unwrap();
        let pos = pos.split(',').map(|x| x
            .trim()
            .parse::<i128>()
            .unwrap()
        ).collect::<Vec<_>>();
        assert_eq!(pos.len(), 3);
        let vel = vel.split(',').map(|x| x
            .trim()
            .parse::<i128>()
            .unwrap()
        ).collect::<Vec<_>>();
        assert_eq!(vel.len(), 3);
        Hailstone {
            pos: (pos[0], pos[1], pos[2]),
            vel: (vel[0], vel[1], vel[2]),
            line: (vel[1], -vel[0], vel[1] * pos[0] - vel[0] * pos[1])
        }
    }).collect::<Vec<_>>()
}

#[cfg(test)]
fn get_boundaries() -> (f64, f64) {
    (7f64, 27f64)
}

#[cfg(not(test))]
fn get_boundaries() -> (f64, f64) {
    (200000000000000f64, 400000000000000f64)
}
pub fn part1(data: &str) -> i64 {
    let data = parse(data);
    let (lower_bound, upper_bound) = get_boundaries();

    let mut result = 0;
    for (i,  h1) in data.iter().enumerate() {
        for h2 in data.iter().skip(i + 1) {
            if h1.line.0 * h2.line.1 == h1.line.1 * h2.line.0 {
                continue;
            }
            let denominator = (h1.line.0 * h2.line.1 - h2.line.0 * h1.line.1) as f64;
            let x = (h1.line.2 * h2.line.1 - h2.line.2 * h1.line.1) as f64 / denominator;
            let y = (h2.line.2 * h1.line.0 - h1.line.2 * h2.line.0) as f64 / denominator;

            if x >= lower_bound && x <= upper_bound && y >= lower_bound && y <= upper_bound
            && ((x - h1.pos.0 as f64) * h1.vel.0 as f64) >= 0f64
            && ((y - h1.pos.1 as f64) * h1.vel.1 as f64) >= 0f64
            && ((x - h2.pos.0 as f64) * h2.vel.0 as f64) >= 0f64
            && ((y - h2.pos.1 as f64) * h2.vel.1 as f64) >= 0f64 {
                result += 1;
            }
        }
    }

    result
}


pub fn part2(data: &str) -> i64 {
    let _data = parse(data);
    // God is dead.
    0
}