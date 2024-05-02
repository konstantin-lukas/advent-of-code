use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Copy)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

fn next_position(row: usize, col: usize, width: usize, height: usize, dir: Direction) -> Option<(usize, usize)> {
    let pos = match dir {
        Direction::EAST => { (Some(row), col.checked_add( 1)) }
        Direction::WEST => { (Some(row), col.checked_sub(1)) }
        Direction::NORTH => { (row.checked_sub(1), Some(col)) }
        Direction::SOUTH => { (row.checked_add( 1), Some(col)) }
    };

    if let (Some(l), Some(r)) = pos {
        if l >= width || r >= height {
            None
        } else {
            Some((l, r))
        }
    } else {
        None
    }
}

fn simulate_beam(data: Rc<RefCell<Vec<Vec<(char, (bool, bool, bool, bool))>>>>, row: usize, col: usize, dir: Direction) {
    let width = data.borrow()[0].len();
    let height = data.borrow().len();
    let mut col = col;
    let mut row = row;
    let mut dir = dir;

    loop {
        let ch = data.borrow()[row][col].0;
        match dir {
            Direction::NORTH => {
                if data.borrow()[row][col].1.0 { return; }
                data.borrow_mut()[row][col].1.0 = true;
            }
            Direction::SOUTH => {
                if data.borrow()[row][col].1.1 { return; }
                data.borrow_mut()[row][col].1.1 = true;
            }
            Direction::EAST => {
                if data.borrow()[row][col].1.2 { return; }
                data.borrow_mut()[row][col].1.2 = true;
            }
            Direction::WEST => {
                if data.borrow()[row][col].1.3 { return; }
                data.borrow_mut()[row][col].1.3 = true;
            }
        }
        match ch {
            '|' => {
                if let Direction::EAST | Direction::WEST = dir {
                    dir = Direction::NORTH;
                    simulate_beam(data.clone(), row, col, Direction::SOUTH);
                }
            }
            '-' => {
                if let Direction::NORTH | Direction::SOUTH = dir {
                    dir = Direction::EAST;
                    simulate_beam(data.clone(), row, col, Direction::WEST);
                }
            }
            '/' => {
                match dir {
                    Direction::NORTH => { dir = Direction::EAST }
                    Direction::SOUTH => { dir = Direction::WEST }
                    Direction::EAST => { dir = Direction::NORTH }
                    Direction::WEST => { dir = Direction::SOUTH }
                }
            }
            '\\' => {
                match dir {
                    Direction::NORTH => { dir = Direction::WEST }
                    Direction::SOUTH => { dir = Direction::EAST }
                    Direction::EAST => { dir = Direction::SOUTH }
                    Direction::WEST => { dir = Direction::NORTH }
                }
            }
            _ => {}
        }
        if let Some((next_row, next_col)) = next_position(row, col, width, height, dir) {
            row = next_row;
            col = next_col;
        } else {
            return;
        }
    }
}

fn parse(data: &str) -> Vec<Vec<(char, (bool, bool, bool, bool))>> {
    data
        .lines()
        .map(|x|
            x.chars().map(|y| (y, (false, false, false, false))).collect::<Vec<_>>()
        ).collect::<Vec<_>>()
}

pub fn part1(data: &str) -> i64 {
    let data = Rc::new(RefCell::new(parse(data)));

    simulate_beam(data.clone(), 0, 0, Direction::EAST);

    let x = data.borrow().iter().map(|x| x.iter().filter(|&y| y.1.0 || y.1.1 || y.1.2 || y.1.3).count() as i64).sum();
    x
}

pub fn part2(data: &str) -> i64 {
    let data = parse(data);
    let width = data[0].len();
    let height = data.len();
    let mut results = vec![0; width * 2 + height * 2];
    for i in 0..width {
        let data = Rc::new(RefCell::new(data.clone()));
        simulate_beam(data.clone(), 0, i, Direction::SOUTH);
        let x: i64 = data.borrow().iter().map(|x| x.iter().filter(|&y| y.1.0 || y.1.1 || y.1.2 || y.1.3).count() as i64).sum();
        results[i] = x;
    }
    for i in 0..width {
        let data = Rc::new(RefCell::new(data.clone()));
        simulate_beam(data.clone(), height - 1, i, Direction::NORTH);
        let x: i64 = data.borrow().iter().map(|x| x.iter().filter(|&y| y.1.0 || y.1.1 || y.1.2 || y.1.3).count() as i64).sum();
        results[i + width] = x;
    }
    for i in 0..height {
        let data = Rc::new(RefCell::new(data.clone()));
        simulate_beam(data.clone(), i, 0, Direction::EAST);
        let x: i64 = data.borrow().iter().map(|x| x.iter().filter(|&y| y.1.0 || y.1.1 || y.1.2 || y.1.3).count() as i64).sum();
        results[i + (2 * width)] = x;
    }
    for i in 0..height {
        let data = Rc::new(RefCell::new(data.clone()));
        simulate_beam(data.clone(), i, width - 1, Direction::WEST);
        let x: i64 = data.borrow().iter().map(|x| x.iter().filter(|&y| y.1.0 || y.1.1 || y.1.2 || y.1.3).count() as i64).sum();
        results[i + (2 * width) + height] = x;
    }

    *results.iter().max().unwrap()
}