use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

fn parse(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>()
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::UP, Direction::UP) => true,
            (Direction::DOWN, Direction::DOWN) => true,
            (Direction::LEFT, Direction::LEFT) => true,
            (Direction::RIGHT, Direction::RIGHT) => true,
            _ => false,
        }
    }
}

fn mov(
    position: (usize, usize),
    dir: Direction,
    data: &Vec<Vec<char>>,
    visited: &HashSet<(usize, usize)>
) -> Option<(usize, usize)> {
    let (row, col) = match dir {
        Direction::UP => (position.0 - 1, position.1),
        Direction::DOWN => (position.0 + 1, position.1),
        Direction::LEFT => (position.0, position.1 - 1),
        Direction::RIGHT => (position.0, position.1 + 1)
    };
    let target = data[row][col];
    if target == '#'
        || target == 'v' && dir == Direction::UP
        || target == '<' && dir == Direction::RIGHT
        || target == '>' && dir == Direction::LEFT
        || target == '^' && dir == Direction::DOWN
        || visited.contains(&(row, col))
    { return None; }

    Some((row, col))
}

fn mov2(
    position: (usize, usize),
    dir: Direction,
    data: &Vec<Vec<char>>,
    visited: &HashSet<(usize, usize)>
) -> Option<(usize, usize)> {
    let (row, col) = match dir {
        Direction::UP => (position.0 - 1, position.1),
        Direction::DOWN => (position.0 + 1, position.1),
        Direction::LEFT => (position.0, position.1 - 1),
        Direction::RIGHT => (position.0, position.1 + 1)
    };
    let target = data[row][col];
    if target == '#'
        || visited.contains(&(row, col))
    { return None; }

    Some((row, col))
}

fn traverse(
    mut position: (usize, usize),
    mut coming_from: Direction,
    data: &Vec<Vec<char>>,
    mut path_len: i64,
    mut visited: HashSet<(usize, usize)>
) -> i64 {

    let mut new_positions = vec![];
    while {
        if position.0 == data.len() - 1 { return path_len; }
        new_positions.clear();
        visited.insert(position);
        match coming_from {
            Direction::UP => {
                if let Some(res) = mov(position, Direction::LEFT, data, &visited) {
                    new_positions.push((Direction::RIGHT, res));
                }
                if let Some(res) = mov(position, Direction::RIGHT, data, &visited) {
                    new_positions.push((Direction::LEFT, res));
                }
                if let Some(res) = mov(position, Direction::DOWN, data, &visited) {
                    new_positions.push((Direction::UP, res));
                }
            },
            Direction::DOWN => {
                if let Some(res) = mov(position, Direction::LEFT, data, &visited) {
                    new_positions.push((Direction::RIGHT, res));
                }
                if let Some(res) = mov(position, Direction::RIGHT, data, &visited) {
                    new_positions.push((Direction::LEFT, res));
                }
                if let Some(res) = mov(position, Direction::UP, data, &visited) {
                    new_positions.push((Direction::DOWN, res));
                }
            },
            Direction::LEFT => {
                if let Some(res) = mov(position, Direction::UP, data, &visited) {
                    new_positions.push((Direction::DOWN, res));
                }
                if let Some(res) = mov(position, Direction::RIGHT, data, &visited) {
                    new_positions.push((Direction::LEFT, res));
                }
                if let Some(res) = mov(position, Direction::DOWN, data, &visited) {
                    new_positions.push((Direction::UP, res));
                }
            },
            Direction::RIGHT => {
                if let Some(res) = mov(position, Direction::LEFT, data, &visited) {
                    new_positions.push((Direction::RIGHT, res));
                }
                if let Some(res) = mov(position, Direction::UP, data, &visited) {
                    new_positions.push((Direction::DOWN, res));
                }
                if let Some(res) = mov(position, Direction::DOWN, data, &visited) {
                    new_positions.push((Direction::UP, res));
                }
            }
        }
        new_positions.len() == 1
    } {
        position = new_positions[0].1;
        coming_from = new_positions[0].0;
        path_len += 1;
    }

    if new_positions.is_empty() {
        return 0;
    }

    let mut lengths = vec![];
    for new_position in new_positions {
        let (from, pos) = new_position;
        lengths.push(traverse(pos, from, data, path_len + 1, visited.clone()));
    }
    *lengths.iter().max().unwrap()
}

pub fn part1(data: &str) -> i64 {
    let data = parse(data);
    traverse((0, 1), Direction::UP, &data, 0, HashSet::new())
}

fn build_graph(
    graph: &mut HashMap<(usize, usize), Vec<(usize, (usize, usize))>>,
    previous_node: (usize, usize),
    mut position: (usize, usize),
    mut coming_from: Direction,
    data: &Vec<Vec<char>>,
    mut path_len: usize,
    mut visited: HashSet<(usize, usize)>
) {
    let mut new_positions = vec![];
    while {
        if position.0 == data.len() - 1 {
            if !graph.contains_key(&previous_node) {
                graph.insert(previous_node, Vec::new());
            }
            graph.get_mut(&previous_node).unwrap().push((path_len, position));
            return;
        }
        new_positions.clear();
        match coming_from {
            Direction::UP => {
                if let Some(res) = mov2(position, Direction::LEFT, data, &visited) {
                    new_positions.push((Direction::RIGHT, res));
                }
                if let Some(res) = mov2(position, Direction::RIGHT, data, &visited) {
                    new_positions.push((Direction::LEFT, res));
                }
                if let Some(res) = mov2(position, Direction::DOWN, data, &visited) {
                    new_positions.push((Direction::UP, res));
                }
            },
            Direction::DOWN => {
                if let Some(res) = mov2(position, Direction::LEFT, data, &visited) {
                    new_positions.push((Direction::RIGHT, res));
                }
                if let Some(res) = mov2(position, Direction::RIGHT, data, &visited) {
                    new_positions.push((Direction::LEFT, res));
                }
                if let Some(res) = mov2(position, Direction::UP, data, &visited) {
                    new_positions.push((Direction::DOWN, res));
                }
            },
            Direction::LEFT => {
                if let Some(res) = mov2(position, Direction::UP, data, &visited) {
                    new_positions.push((Direction::DOWN, res));
                }
                if let Some(res) = mov2(position, Direction::RIGHT, data, &visited) {
                    new_positions.push((Direction::LEFT, res));
                }
                if let Some(res) = mov2(position, Direction::DOWN, data, &visited) {
                    new_positions.push((Direction::UP, res));
                }
            },
            Direction::RIGHT => {
                if let Some(res) = mov2(position, Direction::LEFT, data, &visited) {
                    new_positions.push((Direction::RIGHT, res));
                }
                if let Some(res) = mov2(position, Direction::UP, data, &visited) {
                    new_positions.push((Direction::DOWN, res));
                }
                if let Some(res) = mov2(position, Direction::DOWN, data, &visited) {
                    new_positions.push((Direction::UP, res));
                }
            }
        }
        new_positions.len() == 1
    } {
        position = new_positions[0].1;
        coming_from = new_positions[0].0;
        path_len += 1;
    }

    if new_positions.is_empty() { return; }

    if !graph.contains_key(&previous_node) {
        graph.insert(previous_node, Vec::new());
    }
    graph.get_mut(&previous_node).unwrap().push((path_len, position));
    visited.insert(position);
/*
    if !graph.contains_key(&position) {
        graph.insert(position, Vec::new());
    }
    graph.get_mut(&position).unwrap().push((path_len, previous_node));*/

    for new_position in new_positions {
        let (from, pos) = new_position;
        build_graph(graph, position, pos, from, data, 0, visited.clone());
    }
}

fn traverse_graph(
    node: (usize, usize),
    graph: &HashMap<(usize, usize), Vec<(usize, (usize, usize))>>,
    mut seen: HashSet<(usize, usize)>,
    path_len: usize
) -> i64 {

    seen.insert(node);
    let mut lengths = vec![];
    if let Some(next_nodes) = graph.get(&node) {
        for (cost, next_node) in next_nodes {
            if !seen.contains(next_node) {
                lengths.push(
                    traverse_graph(*next_node, graph, seen.clone(), path_len + cost + 1)
                );
            }
        }
    } else {
        return path_len as i64;
    }

    *lengths.iter().max().unwrap_or(&(path_len as i64))
}

pub fn part2(data: &str) -> i64 {
    let data = parse(data);
    let mut graph = HashMap::new();
    build_graph(
        &mut graph,
        (0, 1),
        (0, 1),
        Direction::UP,
        &data,
        0,
        HashSet::new()
    );0
    //traverse_graph((0, 1), &graph, HashSet::new(), 0) - 1

}