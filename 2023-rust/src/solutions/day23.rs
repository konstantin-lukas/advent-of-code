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

fn find_nodes(data: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut result = vec![(0, 1), (data.len() - 1, data[0].len() - 2)];

    for (ri, row) in data[1..data.len() - 1].iter().enumerate() {
        let ri = ri + 1;
        for (ci, col) in row[1..row.len() - 1].iter().enumerate() {
            let ci = ci + 1;
            if *col != '#' {
                let mut paths = 0;
                if data[ri - 1][ci] != '#' { paths += 1; }
                if data[ri][ci - 1] != '#' { paths += 1; }
                if data[ri + 1][ci] != '#' { paths += 1; }
                if data[ri][ci + 1] != '#' { paths += 1; }
                if paths > 2 { result.push((ri, ci)) }
            }
        }
    }

    result
}

fn calc_weights(
    data: &Vec<Vec<char>>,
    nodes: &Vec<(usize, usize)>
) -> HashMap<(usize, usize), Vec<((usize, usize), usize)>> {
    let mut graph = HashMap::new();

    for &node in nodes {
        let (row, col) = node;
        let mut directions = vec![];
        if row > 0 && data[row - 1][col] != '#' { directions.push((-1, 0)) }
        if col > 0 && data[row][col - 1] != '#' { directions.push((0, -1)) }
        if row < data.len() - 1 && data[row + 1][col] != '#' { directions.push((1, 0)) }
        if col < data[row].len() - 1 && data[row][col + 1] != '#' { directions.push((0, 1)) }
        for (next_node, cost) in directions.into_iter().map(|dir| {
            find_distance_to_next_node(node, data, dir, nodes)
        }) {
            if !graph.contains_key(&node) { graph.insert(node, vec![]); }
            if !graph.get(&node).unwrap().contains(&(next_node, cost)) {
                graph.get_mut(&node).unwrap().push((next_node, cost));
            }
            if !graph.contains_key(&next_node) { graph.insert(next_node, vec![]); }
            if !graph.get(&next_node).unwrap().contains(&(node, cost)) {
                graph.get_mut(&next_node).unwrap().push((node, cost));
            }
        }
    }

    graph
}

fn find_distance_to_next_node(
    mut node: (usize, usize),
    data: &Vec<Vec<char>>,
    direction: (i32, i32),
    nodes: &Vec<(usize, usize)>
) -> ((usize, usize), usize) {
    node.0 = (node.0 as i32 + direction.0) as usize;
    node.1 = (node.1 as i32 + direction.1) as usize;
    let mut coming_from = match direction {
        (-1, 0) => Direction::DOWN,
        (0, -1) => Direction:: RIGHT,
        (1, 0) => Direction::UP,
        (0, 1) => Direction:: LEFT,
        _ => panic!()
    };
    let mut steps = 1;
    while !nodes.contains(&node) {
        match coming_from {
            Direction::UP => {
                if data[node.0 + 1][node.1] != '#' {
                    node.0 += 1;
                } else if data[node.0][node.1 - 1] != '#' {
                    node.1 -= 1; coming_from = Direction::RIGHT;
                } else if data[node.0][node.1 + 1] != '#' {
                    node.1 += 1; coming_from = Direction::LEFT;
                } else {
                    panic!();
                }
            },
            Direction::DOWN => {
                if data[node.0 - 1][node.1] != '#' {
                    node.0 -= 1;
                } else if data[node.0][node.1 - 1] != '#' {
                    node.1 -= 1; coming_from = Direction::RIGHT;
                } else if data[node.0][node.1 + 1] != '#' {
                    node.1 += 1; coming_from = Direction::LEFT;
                } else {
                    panic!();
                }
            },
            Direction::LEFT => {
                if data[node.0][node.1 + 1] != '#' {
                    node.1 += 1;
                } else if data[node.0 - 1][node.1] != '#' {
                    node.0 -= 1; coming_from = Direction::DOWN;
                } else if data[node.0 + 1][node.1] != '#' {
                    node.0 += 1; coming_from = Direction::UP;
                } else {
                    panic!();
                }
            },
            Direction::RIGHT => {
                if data[node.0][node.1 - 1] != '#' {
                    node.1 -= 1;
                } else if data[node.0 - 1][node.1] != '#' {
                    node.0 -= 1; coming_from = Direction::DOWN;
                } else if data[node.0 + 1][node.1] != '#' {
                    node.0 += 1; coming_from = Direction::UP;
                } else {
                    panic!();
                }
            }
        }
        steps += 1;
    }
    (node, steps)
}

fn make_edge_vertices_directed(
    origin_node: (usize, usize),
    stop_node: (usize, usize),
    graph: &mut HashMap<(usize, usize), Vec<((usize, usize),usize)>>,
) {
    if origin_node == stop_node { return; }
    if graph.get(&origin_node).unwrap().len() == 3 {
        let targets = graph.get(&origin_node).unwrap().clone();
        for (target, _) in targets {
            let pos = graph
                .get(&target)
                .unwrap()
                .iter()
                .position(|(x, _)| *x == origin_node);
            if let Some(pos) = pos {
                graph.get_mut(&target).unwrap().remove(pos);
            }
            make_edge_vertices_directed(target, stop_node, graph);
        }
    }
}

fn longest_path(
    graph: &HashMap<(usize, usize), Vec<((usize, usize), usize)>>,
    start: (usize, usize),
    end: (usize, usize)
) -> Option<usize> {
    fn dfs(
        node: (usize, usize),
        end: (usize, usize),
        graph: &HashMap<(usize, usize), Vec<((usize, usize), usize)>>,
        visited: &mut HashSet<(usize, usize)>,
    ) -> Option<usize> {
        if node == end {
            return Some(0);
        }

        visited.insert(node);

        let mut max_distance = None;
        if let Some(neighbors) = graph.get(&node) {
            for &(next_node, weight) in neighbors {
                if !visited.contains(&next_node) {
                    if let Some(distance) = dfs(next_node, end, graph, visited) {
                        let new_distance = distance + weight;
                        max_distance = Some(max_distance.map_or(new_distance, |d: usize| d.max(new_distance)));
                    }
                }
            }
        }

        visited.remove(&node);
        max_distance
    }

    dfs(start, end, graph, &mut HashSet::new())
}

pub fn part2(data: &str) -> i64 {
    let data = parse(data);
    let nodes = find_nodes(&data);
    let mut graph = calc_weights(&data, &nodes);
    make_edge_vertices_directed(
        (0, 1),
        (data.len() - 1, data.len() - 2),
        &mut graph
    );
    longest_path(&graph, (0, 1), (data.len() - 1, data.len() - 2)).unwrap() as i64

}