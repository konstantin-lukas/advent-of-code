use pathfinding::prelude::astar;
#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: usize,
    y: usize,
    dir: Direction,
    steps_since_last_turn: usize,
}

fn move_to(pos: &Pos, to: Direction, grid: &Vec<Vec<usize>>, ultra_crucibles: bool) -> Option<(Pos, usize)> {
    if pos.dir == Direction::RIGHT && to == Direction::LEFT ||
        pos.dir == Direction::LEFT && to == Direction::RIGHT ||
        pos.dir == Direction::UP && to == Direction::DOWN ||
        pos.dir == Direction::DOWN && to == Direction::UP {
        return None;
    }
    let mut x = pos.x as i64;
    let mut y = pos.y as i64;
    match to {
        Direction::RIGHT => x += 1,
        Direction::LEFT => x -= 1,
        Direction::UP => y -= 1,
        Direction::DOWN => y += 1,
    }
    if x < 0 || x >= grid[0].len() as i64 || y < 0 || y >= grid.len() as i64 { return None };
    let x = x as usize;
    let y = y as usize;
    let steps = if pos.dir == to { pos.steps_since_last_turn + 1 } else { 0 };

    if ultra_crucibles {
        if pos.steps_since_last_turn + 1 < 4 && pos.dir != to || pos.steps_since_last_turn + 1 > 10 {
            return None;
        }
    } else if steps == 3 {
        return None;
    }

    Some((
        Pos {
            x,
            y,
            dir: to,
            steps_since_last_turn: steps
        },
        grid[y][x]
    ))
}
impl Pos {
    fn distance(&self, other: &Pos) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn successors(&self, grid: &Vec<Vec<usize>>, ultra_crucibles: bool) -> Vec<(Pos, usize)> {
        let mut next = Vec::with_capacity(4);
        if let Some(right) = move_to(&self, Direction::RIGHT, grid, ultra_crucibles) { next.push(right); }
        if let Some(down) = move_to(&self, Direction::DOWN, grid, ultra_crucibles) { next.push(down); }
        if let Some(left) = move_to(&self, Direction::LEFT, grid, ultra_crucibles) { next.push(left); }
        if let Some(up) = move_to(&self, Direction::UP, grid, ultra_crucibles) { next.push(up); }
        next
    }
}

fn find_shortest_path(data: &str, ultra_crucibles: bool) -> i64 {
    let data = data
        .lines()
        .map(|x| x
            .chars()
            .map(|y| y
                .to_digit(10)
                .unwrap() as usize
            ).collect::<Vec<_>>()
        ).collect::<Vec<_>>();
    let goal: Pos = Pos {
        x: data[0].len() - 1,
        y: data.len() - 1,
        dir: Direction::RIGHT,
        steps_since_last_turn: 0
    };
    let start = Pos {
        x: 0,
        y: 0,
        dir: Direction::RIGHT,
        steps_since_last_turn: 0
    };
    let result = astar(
        &start,
        |p| p.successors(&data, ultra_crucibles),
        |p| p.distance(&goal),
        |p| p.x == goal.x && p.y == goal.y
    ).unwrap();
    result.1 as i64
}

pub fn part1(data: &str) -> i64 {
    find_shortest_path(data, false)
}

pub fn part2(data: &str) -> i64 {
    find_shortest_path(data, true)
}