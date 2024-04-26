fn unique_pairs(nums: &Vec<(usize, usize)>) -> Vec<(&(usize, usize), &(usize, usize))> {
    let mut pairs = Vec::new();

    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            pairs.push((&nums[i], &nums[j]));
        }
    }

    pairs
}

fn find_galaxies(data: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    for (i, row) in data.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == '#' {
                galaxies.push((i, j));
            }
        }
    }
    galaxies
}

fn parse(data: &str) -> Vec<Vec<char>> {
    data
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn part1(data: &str) -> i64 {
    let mut data = parse(data);

    // EXPAND ROWS
    let mut i = 0;
    while i < data.len() {
        if data[i].iter().all(|&x| x == '.') {
            data.insert(i, vec!('.'; data[i].len()));
            i += 1;
        }
        i += 1;
    }

    // EXPAND COLUMNS
    let mut i = 0;
    while i < data[0].len() {
        if data.iter().all(|x| x[i] == '.') {
            for datum in data.iter_mut() {
                datum.insert(i, '.');
            }
            i += 1;
        }
        i += 1;
    }

    // FIND GALAXIES
    let galaxies = find_galaxies(&data);

    let mut result = 0;
    for (left, right) in unique_pairs(&galaxies) {
        let dx = usize::max(left.1, right.1) - usize::min(left.1, right.1);
        let dy = usize::max(left.0, right.0) - usize::min(left.0, right.0);

        result += (dx + dy) as i64;
    }
    result
}

#[cfg(test)]
fn get_expansion_factor() -> i64 {
    100
}

#[cfg(not(test))]
fn get_expansion_factor() -> i64 {
    1000000
}

pub fn part2(data: &str) -> i64 {
    let mut data = parse(data);

    // EXPAND ROWS
    let mut i = 0;
    while i < data.len() {
        if data[i].iter().all(|&x| x == '.') {
            data[i] = vec!['X'; data[i].len()];
        }
        i += 1;
    }

    // EXPAND COLUMNS
    let mut i = 0;
    while i < data[0].len() {
        if data.iter().all(|x| x[i] == '.' || x[i] == 'X') {
            for datum in data.iter_mut() {
                datum[i] = 'X';
            }
        }
        i += 1;
    }

    // FIND GALAXIES
    let galaxies = find_galaxies(&data);

    let mut result = 0;
    for (left, right) in unique_pairs(&galaxies) {

        let x_start = usize::min(left.1, right.1);
        let x_end = usize::max(left.1, right.1);
        let y_start = usize::min(left.0, right.0);
        let y_end = usize::max(left.0, right.0);
        let expansion_factor = get_expansion_factor();

        for x in x_start..x_end {
            result += if data[y_start..=y_end].iter().all(|a| a[x] == 'X') { expansion_factor } else { 1 };
        }

        for y in y_start..y_end {
            result += if data[y][x_start] == 'X' { expansion_factor } else { 1 };
        }
    }
    result
}