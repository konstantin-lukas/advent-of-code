#[derive(Debug)]
struct Pos {
    row: usize,
    col: usize
}
#[derive(Debug)]
struct PipeIter<'a> {
    pipes: &'a Pipes,
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
impl<'a> PipeIter<'a> {
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
            pipes,
            pos: Pos {
                row,
                col
            },
            prev_pos
        }
    }
    fn next(&mut self) {
        let current = self.pipes.map[self.pos.row][self.pos.col];
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

impl PartialEq<PipeIter<'_>> for PipeIter<'_> {
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

pub fn part1(data: &str) -> i64 {
    let pipes = Pipes::new(data);
    let mut left_iter = PipeIter::new(&pipes, false);
    let mut right_iter = PipeIter::new(&pipes, true);
    let mut result = 1;
    while {
        left_iter.next();
        right_iter.next();
        result += 1;
        left_iter != right_iter
    } {}
    result
}

pub fn part2(_data: &str) -> i64 {
    0
}