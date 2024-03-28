pub mod utils;
pub mod solutions;

fn main() {
    let (part1, part2) = solutions::day4::run(false);
    println!("DAY 4, PART 1: {part1}");
    println!("DAY 4, PART 2: {part2}");
    let (part1, part2) = solutions::day5::run(false);
    println!("DAY 5, PART 1: {part1}");
    println!("DAY 5, PART 2: {part2}");
}

#[cfg(test)]
mod tests {
    use crate::solutions;
    #[test]
    fn day4() {
        let (part1, part2) = solutions::day4::run(true);
        assert_eq!(part1, 13);
        assert_eq!(part2, 30);
    }
    #[test]
    fn day5() {
        let (part1, part2) = solutions::day5::run(true);
        assert_eq!(part1, 35);
        assert_eq!(part2, 46);
    }
}