pub mod utils;
pub mod solutions {
    pub mod day01;
    pub mod day02;
    pub mod day06;
}

fn main() {
    let data = utils::load_data(1);
    println!("DAY 1, PART 1: {}", solutions::day01::part1(&data));
    println!("DAY 1, PART 2: {}", solutions::day01::part2(&data));
    let data = utils::load_data(2);
    println!("DAY 2, PART 1: {}", solutions::day02::part1(&data));
    println!("DAY 2, PART 2: {}", solutions::day02::part2(&data));
    let data = utils::load_data(6);
    println!("DAY 6, PART 1: {}", solutions::day06::part1(&data));
    println!("DAY 6, PART 2: {}", solutions::day06::part2(&data));

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
    fn day2() {
        let data = utils::load_data(2);
        assert_eq!(solutions::day02::part1(&data), 0);
        assert_eq!(solutions::day02::part2(&data), 0);
    }
    #[test]
    fn day6() {
        let data = utils::load_data(6);
        assert_eq!(solutions::day06::part1(&data), 288);
        assert_eq!(solutions::day06::part2(&data), 71503);
    }
}