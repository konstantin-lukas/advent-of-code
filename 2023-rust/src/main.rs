pub mod utils;
pub mod solutions {
    pub mod day6;
}

fn main() {
    /*let (part1, part2) = solutions::day1::run();
    println!("DAY 1, PART 1: {part1}");
    println!("DAY 1, PART 2: {part2}");
    let (part1, part2) = solutions::day2::run();
    println!("DAY 2, PART 1: {part1}");
    println!("DAY 2, PART 2: {part2}");
    let (part1, part2) = solutions::day3::run();
    println!("DAY 3, PART 1: {part1}");
    println!("DAY 3, PART 2: {part2}");
    let (part1, part2) = solutions::day4::run();
    println!("DAY 4, PART 1: {part1}");
    println!("DAY 4, PART 2: {part2}");
    let (part1, part2) = solutions::day5::run();
    println!("DAY 5, PART 1: {part1}");
    println!("DAY 5, PART 2: {part2}");*/
    let data = utils::load_data(6);
    println!("DAY 6, PART 1: {}", solutions::day6::part1(&data));
    println!("DAY 6, PART 2: {}", solutions::day6::part2(&data));

    utils::benchmark();
}

#[cfg(test)]
mod tests {
    use crate::{solutions, utils};
    /*#[test]
    fn day1() {
        let (part1, part2) = solutions::day1::run();
        assert_eq!(part1, 297);
        assert_eq!(part2, 281);
    }
    #[test]
    fn day2() {
        let (part1, part2) = solutions::day2::run();
        assert_eq!(part1, 8);
        assert_eq!(part2, 2286);
    }
    #[test]
    fn day3() {
        let (part1, part2) = solutions::day3::run();
        assert_eq!(part1, 4361);
        assert_eq!(part2, 467835);
    }
    #[test]
    fn day4() {
        let (part1, part2) = solutions::day4::run();
        assert_eq!(part1, 13);
        assert_eq!(part2, 30);
    }
    #[test]
    fn day5() {
        let (part1, part2) = solutions::day5::run();
        assert_eq!(part1, 35);
        assert_eq!(part2, 46);
    }*/
    #[test]
    fn day6() {
        let data = utils::load_data(6);
        assert_eq!(solutions::day6::part1(&data), 288);
        assert_eq!(solutions::day6::part2(&data), 71503);
    }
}