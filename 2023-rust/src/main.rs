
pub mod utils;
pub mod solutions {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day20;
    pub mod day21;
    pub mod day22;
    pub mod day23;
    pub mod day24;
    pub mod day25;
}

fn main() {
    // Day 1
    let data = utils::load_data(1);
    println!("DAY 1, PART 1: {}", solutions::day01::part1(&data));
    println!("DAY 1, PART 2: {}", solutions::day01::part2(&data));

    // Day 2
    let data = utils::load_data(2);
    println!("DAY 2, PART 1: {}", solutions::day02::part1(&data));
    println!("DAY 2, PART 2: {}", solutions::day02::part2(&data));

    // Day 3
    let data = utils::load_data(3);
    println!("DAY 3, PART 1: {}", solutions::day03::part1(&data));
    println!("DAY 3, PART 2: {}", solutions::day03::part2(&data));

    // Day 4
    let data = utils::load_data(4);
    println!("DAY 4, PART 1: {}", solutions::day04::part1(&data));
    println!("DAY 4, PART 2: {}", solutions::day04::part2(&data));

    // Day 5
    let data = utils::load_data(5);
    println!("DAY 5, PART 1: {}", solutions::day05::part1(&data));
    println!("DAY 5, PART 2: {}", solutions::day05::part2(&data));

    // Day 6
    let data = utils::load_data(6);
    println!("DAY 6, PART 1: {}", solutions::day06::part1(&data));
    println!("DAY 6, PART 2: {}", solutions::day06::part2(&data));

    // Day 7
    let data = utils::load_data(7);
    println!("DAY 7, PART 1: {}", solutions::day07::part1(&data));
    println!("DAY 7, PART 2: {}", solutions::day07::part2(&data));

    // Day 8
    let data = utils::load_data(8);
    println!("DAY 8, PART 1: {}", solutions::day08::part1(&data));
    println!("DAY 8, PART 2: {}", solutions::day08::part2(&data));

    // Day 9
    let data = utils::load_data(9);
    println!("DAY 9, PART 1: {}", solutions::day09::part1(&data));
    println!("DAY 9, PART 2: {}", solutions::day09::part2(&data));

    // Day 10
    let data = utils::load_data(10);
    println!("DAY 10, PART 1: {}", solutions::day10::part1(&data));
    println!("DAY 10, PART 2: {}", solutions::day10::part2(&data));

    // Day 11
    let data = utils::load_data(11);
    println!("DAY 11, PART 1: {}", solutions::day11::part1(&data));
    println!("DAY 11, PART 2: {}", solutions::day11::part2(&data));

    // Day 12
    let data = utils::load_data(12);
    println!("DAY 12, PART 1: {}", solutions::day12::part1(&data));
    println!("DAY 12, PART 2: {}", solutions::day12::part2(&data));

    // Day 13
    let data = utils::load_data(13);
    println!("DAY 13, PART 1: {}", solutions::day13::part1(&data));
    println!("DAY 13, PART 2: {}", solutions::day13::part2(&data));

    // Day 14
    let data = utils::load_data(14);
    println!("DAY 14, PART 1: {}", solutions::day14::part1(&data));
    println!("DAY 14, PART 2: {}", solutions::day14::part2(&data));

    // Day 15
    let data = utils::load_data(15);
    println!("DAY 15, PART 1: {}", solutions::day15::part1(&data));
    println!("DAY 15, PART 2: {}", solutions::day15::part2(&data));

    // Day 16
    let data = utils::load_data(16);
    println!("DAY 16, PART 1: {}", solutions::day16::part1(&data));
    println!("DAY 16, PART 2: {}", solutions::day16::part2(&data));

    // Day 17
    let data = utils::load_data(17);
    println!("DAY 17, PART 1: {}", solutions::day17::part1(&data));
    println!("DAY 17, PART 2: {}", solutions::day17::part2(&data));

    // Day 18
    let data = utils::load_data(18);
    println!("DAY 18, PART 1: {}", solutions::day18::part1(&data));
    println!("DAY 18, PART 2: {}", solutions::day18::part2(&data));

    // Day 19
    let data = utils::load_data(19);
    println!("DAY 19, PART 1: {}", solutions::day19::part1(&data));
    println!("DAY 19, PART 2: {}", solutions::day19::part2(&data));

    // Day 20
    let data = utils::load_data(20);
    println!("DAY 20, PART 1: {}", solutions::day20::part1(&data));
    println!("DAY 20, PART 2: {}", solutions::day20::part2(&data));

    // Day 21
    let data = utils::load_data(21);
    println!("DAY 21, PART 1: {}", solutions::day21::part1(&data));
    println!("DAY 21, PART 2: {}", solutions::day21::part2(&data));

    // Day 22
    let data = utils::load_data(22);
    println!("DAY 22, PART 1: {}", solutions::day22::part1(&data));
    println!("DAY 22, PART 2: {}", solutions::day22::part2(&data));

    // Day 23
    let data = utils::load_data(23);
    println!("DAY 23, PART 1: {}", solutions::day23::part1(&data));
    println!("DAY 23, PART 2: {}", solutions::day23::part2(&data));

    // Day 24
    let data = utils::load_data(24);
    println!("DAY 24, PART 1: {}", solutions::day24::part1(&data));
    println!("DAY 24, PART 2: {}", solutions::day24::part2(&data));

    // Day 25
    let data = utils::load_data(25);
    println!("DAY 25, PART 1: {}", solutions::day25::part1(&data));
    println!("DAY 25, PART 2: {}", solutions::day25::part2(&data));

    utils::benchmark();
}

#[cfg(test)]
mod tests {
    use crate::{solutions, utils};

    #[test]
    fn day1() {
        let data = utils::load_data(1);
        assert_eq!(solutions::day01::part1(&data), 209);
        assert_eq!(solutions::day01::part2(&data), 281);
    }

    #[test]
    fn day02() {
        let data = utils::load_data(2);
        assert_eq!(solutions::day02::part1(&data), 8);
        assert_eq!(solutions::day02::part2(&data), 2286);
    }

    #[test]
    fn day03() {
        let data = utils::load_data(3);
        assert_eq!(solutions::day03::part1(&data), 4361);
        assert_eq!(solutions::day03::part2(&data), 467835);
    }

    #[test]
    fn day04() {
        let data = utils::load_data(4);
        assert_eq!(solutions::day04::part1(&data), 13);
        assert_eq!(solutions::day04::part2(&data), 30);
    }

    #[test]
    fn day05() {
        let data = utils::load_data(5);
        assert_eq!(solutions::day05::part1(&data), 35);
        assert_eq!(solutions::day05::part2(&data), 46);
    }

    #[test]
    fn day6() {
        let data = utils::load_data(6);
        assert_eq!(solutions::day06::part1(&data), 288);
        assert_eq!(solutions::day06::part2(&data), 71503);
    }

    #[test]
    fn day07() {
        let data = utils::load_data(7);
        assert_eq!(solutions::day07::part1(&data), 0);
        assert_eq!(solutions::day07::part2(&data), 0);
    }

    #[test]
    fn day08() {
        let data = utils::load_data(8);
        assert_eq!(solutions::day08::part1(&data), 0);
        assert_eq!(solutions::day08::part2(&data), 0);
    }

    #[test]
    fn day09() {
        let data = utils::load_data(9);
        assert_eq!(solutions::day09::part1(&data), 0);
        assert_eq!(solutions::day09::part2(&data), 0);
    }

    #[test]
    fn day10() {
        let data = utils::load_data(10);
        assert_eq!(solutions::day10::part1(&data), 0);
        assert_eq!(solutions::day10::part2(&data), 0);
    }

    #[test]
    fn day11() {
        let data = utils::load_data(11);
        assert_eq!(solutions::day11::part1(&data), 0);
        assert_eq!(solutions::day11::part2(&data), 0);
    }

    #[test]
    fn day12() {
        let data = utils::load_data(12);
        assert_eq!(solutions::day12::part1(&data), 0);
        assert_eq!(solutions::day12::part2(&data), 0);
    }

    #[test]
    fn day13() {
        let data = utils::load_data(13);
        assert_eq!(solutions::day13::part1(&data), 0);
        assert_eq!(solutions::day13::part2(&data), 0);
    }

    #[test]
    fn day14() {
        let data = utils::load_data(14);
        assert_eq!(solutions::day14::part1(&data), 0);
        assert_eq!(solutions::day14::part2(&data), 0);
    }

    #[test]
    fn day15() {
        let data = utils::load_data(15);
        assert_eq!(solutions::day15::part1(&data), 0);
        assert_eq!(solutions::day15::part2(&data), 0);
    }

    #[test]
    fn day16() {
        let data = utils::load_data(16);
        assert_eq!(solutions::day16::part1(&data), 0);
        assert_eq!(solutions::day16::part2(&data), 0);
    }

    #[test]
    fn day17() {
        let data = utils::load_data(17);
        assert_eq!(solutions::day17::part1(&data), 0);
        assert_eq!(solutions::day17::part2(&data), 0);
    }

    #[test]
    fn day18() {
        let data = utils::load_data(18);
        assert_eq!(solutions::day18::part1(&data), 0);
        assert_eq!(solutions::day18::part2(&data), 0);
    }

    #[test]
    fn day19() {
        let data = utils::load_data(19);
        assert_eq!(solutions::day19::part1(&data), 0);
        assert_eq!(solutions::day19::part2(&data), 0);
    }

    #[test]
    fn day20() {
        let data = utils::load_data(20);
        assert_eq!(solutions::day20::part1(&data), 0);
        assert_eq!(solutions::day20::part2(&data), 0);
    }

    #[test]
    fn day21() {
        let data = utils::load_data(21);
        assert_eq!(solutions::day21::part1(&data), 0);
        assert_eq!(solutions::day21::part2(&data), 0);
    }

    #[test]
    fn day22() {
        let data = utils::load_data(22);
        assert_eq!(solutions::day22::part1(&data), 0);
        assert_eq!(solutions::day22::part2(&data), 0);
    }

    #[test]
    fn day23() {
        let data = utils::load_data(23);
        assert_eq!(solutions::day23::part1(&data), 0);
        assert_eq!(solutions::day23::part2(&data), 0);
    }

    #[test]
    fn day24() {
        let data = utils::load_data(24);
        assert_eq!(solutions::day24::part1(&data), 0);
        assert_eq!(solutions::day24::part2(&data), 0);
    }

    #[test]
    fn day25() {
        let data = utils::load_data(25);
        assert_eq!(solutions::day25::part1(&data), 0);
        assert_eq!(solutions::day25::part2(&data), 0);
    }

}