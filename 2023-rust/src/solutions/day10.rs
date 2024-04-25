use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct Pos {
    row: usize,
    col: usize
}
#[derive(Debug)]
struct PipeIter {
    pos: Pos,
    prev_pos: Direction
}
#[derive(Debug)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}
impl<'a> PipeIter {
    fn new(pipes: &'a Pipes, clockwise: bool) -> Self {
        assert_eq!(pipes.map[pipes.start.row][pipes.start.col], 'S');
        let (mut row, mut col) = (pipes.start.row, pipes.start.col);
        let prev_pos = if clockwise {
            if row > 0 && ['|', '7', 'F'].contains(&pipes.map[row - 1][col]) {
                row -= 1;
                Direction::SOUTH
            } else if col + 1 < pipes.map[row].len() && ['-', '7', 'J'].contains(&pipes.map[row][col + 1]) {
                col += 1;
                Direction::WEST
            } else if row + 1 < pipes.map.len() && ['|', 'L', 'J'].contains(&pipes.map[row + 1][col]) {
                row += 1;
                Direction::NORTH
            } else if col > 0 && ['-', 'L', 'F'].contains(&pipes.map[row][col - 1]) {
                col -= 1;
                Direction::EAST
            } else {
                panic!();
            }
        } else {
            if col > 0 && ['-', 'L', 'F'].contains(&pipes.map[row][col - 1]) {
                col -= 1;
                Direction::EAST
            } else if row + 1 < pipes.map.len() && ['|', 'L', 'J'].contains(&pipes.map[row + 1][col]) {
                row += 1;
                Direction::NORTH
            } else if col + 1 < pipes.map[row].len() && ['-', '7', 'J'].contains(&pipes.map[row][col + 1]) {
                col += 1;
                Direction::WEST
            } else if row > 0 &&  ['|', '7', 'F'].contains(&pipes.map[row - 1][col]) {
                row -= 1;
                Direction::SOUTH
            } else {
                panic!();
            }
        };
        Self {
            pos: Pos {
                row,
                col
            },
            prev_pos
        }
    }
    fn next(&mut self, pipes: &Pipes) {
        let current = pipes.map[self.pos.row][self.pos.col];
        if current == '|' {
            match self.prev_pos {
                Direction::NORTH => { self.pos.row += 1; },
                Direction::SOUTH => { self.pos.row -= 1; },
                _ => panic!()
            }
        } else if current == '-' {
            match self.prev_pos {
                Direction::EAST => { self.pos.col -= 1; },
                Direction::WEST => { self.pos.col += 1; },
                _ => panic!()
            }
        } else if current == 'L' {
            match self.prev_pos {
                Direction::NORTH => { self.pos.col += 1; self.prev_pos = Direction::WEST; },
                Direction::EAST => { self.pos.row -= 1; self.prev_pos = Direction::SOUTH; },
                _ => panic!()
            }
        } else if current == 'J' {
            match self.prev_pos {
                Direction::NORTH => { self.pos.col -= 1; self.prev_pos = Direction::EAST; },
                Direction::WEST => { self.pos.row -= 1; self.prev_pos = Direction::SOUTH; },
                _ => panic!()
            }
        } else if current == '7' {
            match self.prev_pos {
                Direction::SOUTH => { self.pos.col -= 1; self.prev_pos = Direction::EAST; },
                Direction::WEST => { self.pos.row += 1; self.prev_pos = Direction::NORTH; },
                _ => panic!()
            }
        } else if current == 'F' {
            match self.prev_pos {
                Direction::SOUTH => { self.pos.col += 1; self.prev_pos = Direction::WEST; },
                Direction::EAST => { self.pos.row += 1; self.prev_pos = Direction::NORTH; },
                _ => panic!()
            }
        }
    }
}

impl PartialEq<PipeIter> for PipeIter {
    fn eq(&self, other: &PipeIter) -> bool {
        self.pos.row == other.pos.row && self.pos.col == other.pos.col
    }

    fn ne(&self, other: &PipeIter) -> bool {
        self.pos.row != other.pos.row || self.pos.col != other.pos.col
    }
}
#[derive(Debug)]
struct Pipes {
    map: Vec<Vec<char>>,
    start: Pos
}

impl Pipes {
    fn new(data: &str) -> Self {
        let lines = data
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let starting_row = lines.iter().position(|x| x.contains(&'S')).unwrap();
        let starting_col = lines[starting_row].iter().position(|x| x == &'S').unwrap();
        Self {
            map: lines,
            start: Pos {
                row: starting_row,
                col: starting_col
            }
        }
    }
}

impl Clone for Pos {
    fn clone(&self) -> Self {
        Self {
            row: self.row,
            col: self.col
        }
    }
}

impl Index<&usize> for Pipes {
    type Output = Vec<char>;

    fn index(&self, index: &usize) -> &Self::Output {
        &self.map[*index]
    }
}

impl IndexMut<&usize> for Pipes {
    fn index_mut(&mut self, index: &usize) -> &mut Self::Output {
        &mut self.map[*index]
    }
}

pub fn part1(data: &str) -> i64 {
    let pipes = Pipes::new(data);
    let mut left_iter = PipeIter::new(&pipes, false);
    let mut right_iter = PipeIter::new(&pipes, true);
    let mut result = 1;
    while {
        left_iter != right_iter
    } {
        left_iter.next(&pipes);
        right_iter.next(&pipes);
        result += 1;
    }
    result
}

fn fill_edges(pipes: &mut Pipes) {
    let rows = pipes.map.len();
    let cols = pipes[&0].len();

    for i in 0..rows {
        for j in 0..cols {
            if pipes[&i][j] != '_' && (i == 0 || i == rows - 1 || j == 0 || j == cols - 1) {
                flood_fill(pipes, i, j);
            }
        }
    }
}

fn flood_fill(pipes: &mut Pipes, row: usize, col: usize) {
    let rows = pipes.map.len();
    let cols = pipes[&0].len();

    let mut stack = vec![(row, col)];

    while let Some((r, c)) = stack.pop() {
        if pipes[&r][c] != '_' && pipes[&r][c] != 'X' {
            pipes[&r][c] = 'X';
            if r > 0 && pipes[&(r - 1)][c] != 'X' {
                stack.push((r - 1, c));
            }
            if r < rows - 1 && pipes[&(r + 1)][c] != 'X' {
                stack.push((r + 1, c));
            }
            if c > 0 && pipes[&r][c - 1] != 'X' {
                stack.push((r, c - 1));
            }
            if c < cols - 1 && pipes[&r][c + 1] != 'X' {
                stack.push((r, c + 1));
            }
        }
    }
}

#[cfg(test)]
fn get_direction() -> bool {
    true
}

#[cfg(not(test))]
fn get_direction() -> bool {
    false
}

pub fn part2(data: &str) -> i64 {
    let pipes_old = Pipes::new(data);
    let mut pipes = Pipes::new(data);
    let mut iter = PipeIter::new(&pipes, false);
    while pipes[&iter.pos.row][iter.pos.col] != 'S' {
        let prev = iter.pos.clone();
        iter.next(&pipes);
        pipes[&prev.row][prev.col] = '_';
    }
    pipes[&iter.pos.row][iter.pos.col] = '_';
    fill_edges(&mut pipes);
    let mut iter = PipeIter::new(&pipes_old, get_direction());
    while pipes_old[&iter.pos.row][iter.pos.col] != 'S' {
        let prev = iter.pos.clone();
        let symbol = pipes_old[&iter.pos.row][iter.pos.col];
        if symbol == '|' {
            if let Direction::NORTH = iter.prev_pos {
                if prev.col > 0 {
                    flood_fill(&mut pipes, prev.row, prev.col - 1);
                }
            } else if let Direction::SOUTH = iter.prev_pos {
                if prev.col < pipes.map[prev.row].len() - 1 {
                    flood_fill(&mut pipes, prev.row, prev.col + 1);
                }
            }
        } else if symbol == '-' {
            if let Direction::EAST = iter.prev_pos {
                if prev.row > 0 {
                    flood_fill(&mut pipes, prev.row - 1, prev.col);
                }
            } else if let Direction::WEST = iter.prev_pos {
                if prev.row < pipes.map.len() - 1 {
                    flood_fill(&mut pipes, prev.row + 1, prev.col);
                }
            }
        } else if symbol == 'L' {
            if let Direction::NORTH = iter.prev_pos {
                if prev.row < pipes.map.len() - 1 {
                    flood_fill(&mut pipes, prev.row + 1, prev.col);
                }
                if prev.col > 0 {
                    flood_fill(&mut pipes, prev.row, prev.col - 1);
                }
            }
        } else if symbol == 'J' {
            if let Direction::WEST = iter.prev_pos {
                if prev.row < pipes.map.len() - 1 {
                    flood_fill(&mut pipes, prev.row + 1, prev.col);
                }
                if prev.col < pipes.map[prev.row].len() - 1 {
                    flood_fill(&mut pipes, prev.row, prev.col + 1);
                }
            }
        } else if symbol == 'F' {
            if let Direction::EAST = iter.prev_pos {
                if prev.row > 0 {
                    flood_fill(&mut pipes, prev.row - 1, prev.col);
                }
                if prev.col > 0 {
                    flood_fill(&mut pipes, prev.row, prev.col - 1);
                }
            }
        } else if symbol == '7' {
            if let Direction::SOUTH = iter.prev_pos {
                if prev.row > 0 {
                    flood_fill(&mut pipes, prev.row - 1, prev.col);
                }
                if prev.col < pipes.map[prev.row].len() - 1 {
                    flood_fill(&mut pipes, prev.row, prev.col + 1);
                }
            }
        }
        iter.next(&pipes_old);
    }

    pipes.map
        .iter()
        .map(|x| x.iter()
            .filter(|&&y|
                y != 'X' && y != '_'
            ).count() as i64
        ).sum()
}