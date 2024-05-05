#[derive(Debug)]
struct Instruction<'a>(char, usize, &'a str);

fn parse(data: &str) -> Vec<Instruction> {
    data
        .lines()
        .map(|x| {
            let s = x.split(' ').collect::<Vec<_>>();
            Instruction(s[0].chars().nth(0).unwrap(), s[1].parse::<usize>().unwrap(), s[2])
        }).collect::<Vec<Instruction>>()
}

fn correct_instructions(instructions: &mut Vec<Instruction>) {
    for instruction in instructions {
        let steps = usize::from_str_radix(&instruction.2[2..7], 16).unwrap();
        let dir = instruction.2.chars().nth(7).unwrap();
        let dir = match dir {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => panic!()
        };
        instruction.0 = dir;
        instruction.1 = steps;
    }
}

fn shoelace_formula(data: &Vec<Instruction>) -> i64 {
    let mut area = 0;
    let mut offset = 0;
    let mut perim = 0;
    for Instruction(dir, dis, ..) in data.iter() {
        match dir {
            'R' => { offset += *dis as i64 }
            'D' => { area += offset * *dis as i64 }
            'L' => { offset -= *dis as i64 }
            'U' => { area -= offset * *dis as i64 }
            _ => { panic!(); }
        }
        perim += *dis as i64;
    }
    area.abs() + (perim / 2) + 1
}

pub fn part1(data: &str) -> i64 {
    let data = parse(data);
    shoelace_formula(&data)
}

pub fn part2(data: &str) -> i64 {
    let mut data = parse(data);
    correct_instructions(&mut data);
    shoelace_formula(&data)
}