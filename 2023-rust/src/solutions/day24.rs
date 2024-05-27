type Coordinates = (i64, i64, i64,);

#[derive(Debug)]
struct Hailstone {
    pos: Coordinates,
    vel: Coordinates
}

fn parse(data: &str) -> Vec<Hailstone> {
    data.lines().map(|x| {
        let (pos, vel) = x.split_once('@').unwrap();
        let pos = pos.split(',').map(|x| x
            .trim()
            .parse::<i64>()
            .unwrap()
        ).collect::<Vec<_>>();
        assert_eq!(pos.len(), 3);
        let vel = vel.split(',').map(|x| x
            .trim()
            .parse::<i64>()
            .unwrap()
        ).collect::<Vec<_>>();
        assert_eq!(vel.len(), 3);
        Hailstone {
            pos: (pos[0], pos[1], pos[2]),
            vel: (vel[0], vel[1], vel[2])
        }
    }).collect::<Vec<_>>()
}

#[cfg(test)]
fn get_boundaries() -> (i64, i64) {
    (7, 27)
}

#[cfg(not(test))]
fn get_boundaries() -> (i64, i64) {
    (200000000000000, 400000000000000)
}
pub fn part1(data: &str) -> i64 {
    let data = parse(data);
    let (lower_bound, upper_bound) = get_boundaries();
    0
}

pub fn part2(_data: &str) -> i64 {
    0
}