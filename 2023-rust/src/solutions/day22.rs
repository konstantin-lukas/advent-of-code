use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    z: usize
}

#[derive(Debug)]
struct Brick {
    p1: Point,
    p2: Point
}

fn rectangle_collision_check(r1: ((usize, usize), (usize, usize)), r2: ((usize, usize), (usize, usize))) -> bool {

    let ((x1_min, y1_min), (x1_max, y1_max)) = r1;
    let ((x2_min, y2_min), (x2_max, y2_max)) = r2;

    x1_max >= x2_min && x1_min <= x2_max && y1_max >= y2_min && y1_min <= y2_max
}

fn parse(data: &str) -> Vec<Brick> {
    data.lines().map(|l| {
        let split = l
            .split_once('~')
            .unwrap();
        let p1 = split.0.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let p2 = split.1.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        Brick {
            p1: Point {
                x: p1[0],
                y: p1[1],
                z: p1[2]
            },
            p2: Point {
                x: p2[0],
                y: p2[1],
                z: p2[2]
            }
        }
    }).collect()
}

fn drop_bricks(data: &mut Vec<Brick>) {
    for i in 0..data.len() {
        let rect1 = ((data[i].p1.x, data[i].p1.y), (data[i].p2.x, data[i].p2.y));
        let min = data[i].p1.z.min(data[i].p2.z);
        let mut diff = 0;
        let vertically_aligned = data.iter().filter(|brick2| {
            let rect2 = ((brick2.p1.x, brick2.p1.y), (brick2.p2.x, brick2.p2.y));
            let max = brick2.p1.z.max(brick2.p2.z);
            rectangle_collision_check(rect1, rect2) && min > max
        }).collect::<Vec<_>>();

        while min - diff > 1 && vertically_aligned.iter().all(|b| min - diff - 1 > b.p1.z.max(b.p2.z)) {
            diff += 1;
        }

        data[i].p1.z -= diff;
        data[i].p2.z -= diff;
    }
}

fn add_support_vector(data: &Vec<Brick>) ->  Vec<(Vec<usize>, &Brick)>{
    let mut data = data.into_iter().map(|x| (vec![], x)).collect::<Vec<_>>();

    for i in 0..data.len() {
        let rect1 = ((data[i].1.p1.x, data[i].1.p1.y), (data[i].1.p2.x, data[i].1.p2.y));
        let max = data[i].1.p1.z.max(data[i].1.p2.z);
        let supports = data.iter().enumerate().filter_map(|(j, (_, brick2))| {
            let rect2 = ((brick2.p1.x, brick2.p1.y), (brick2.p2.x, brick2.p2.y));
            let min = brick2.p1.z.min(brick2.p2.z);
            if rectangle_collision_check(rect1, rect2) && min - 1 == max { Some(j) } else { None }
        }).collect::<Vec<_>>();
        data[i].0 = supports;
    }

    data
}

pub fn part1(data: &str) -> i64 {
    let mut data = parse(data);
    data.sort_by_key(|b| b.p1.z.min(b.p2.z));
    drop_bricks(&mut data);
    let data = add_support_vector(&data);

    data.iter().filter(|(supports, _brick)| {
        !supports.iter().any(|s| {
            data.iter().filter(|b| {
                b.0.contains(s)
            }).count() == 1
        })
    }).count() as i64
}

pub fn part2(data: &str) -> i64 {
    let mut data = parse(data);
    data.sort_by_key(|b| b.p1.z.min(b.p2.z));
    drop_bricks(&mut data);
    let data = add_support_vector(&data);
    let data = data.iter().enumerate().map(|(i, (supports, brick))| {
        (data
             .iter()
             .enumerate()
             .filter_map(|(j, (supports, _))|
                 if supports.contains(&i) {
                     Some(j)
                 } else {
                     None
                 }
             ).collect::<Vec<_>>(),
         supports,
         brick)
    }).collect::<Vec<_>>();

    let mut result = 0;
    for i in 0..data.len() {
        let mut collapsed = HashSet::new();
        let mut to_collapse = VecDeque::new();
        to_collapse.push_back(i);
        while let Some(j) = to_collapse.pop_front() {
            collapsed.insert(j);
            for support in data[j].1 {
                if data[*support].0.len() > 0 && data[*support].0.iter().all(|x| collapsed.contains(x)) {
                    if !to_collapse.contains(support) {
                        to_collapse.push_back(*support);
                    }
                }
            }
        }
        result += collapsed.len() as i64 - 1
    }
    result
}