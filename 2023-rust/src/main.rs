
pub mod utils;
macro_rules! generate_day_modules {
    ($($day:ident),*) => {
        pub mod solutions {
            $(pub mod $day;)*
        }
    };
}

generate_day_modules!(
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

macro_rules! run_day {
    ($day:ident) => {{
        let data = utils::load_data(stringify!($day).trim_start_matches("day").parse::<u8>().unwrap());
        println!("DAY {}, PART 1: {}", stringify!($day).trim_start_matches("day"), solutions::$day::part1(&data));
        println!("DAY {}, PART 2: {}", stringify!($day).trim_start_matches("day"), solutions::$day::part2(&data));
    }};
}

fn main() {
    run_day!(day01);
    run_day!(day02);
    run_day!(day03);
    run_day!(day04);
    run_day!(day05);
    run_day!(day06);
    run_day!(day07);
    run_day!(day08);
    run_day!(day09);
    run_day!(day10);
    run_day!(day11);
    run_day!(day12);
    run_day!(day13);
    run_day!(day14);
    run_day!(day15);
    run_day!(day16);
    run_day!(day17);
    run_day!(day18);
    run_day!(day19);
    run_day!(day20);
    run_day!(day21);
    run_day!(day22);
    run_day!(day23);
    run_day!(day24);
    run_day!(day25);

    utils::benchmark();
}


#[cfg(test)]
mod tests {
    use crate::{solutions, utils};
    macro_rules! test_day {
        ($day:ident, $part1_result:expr, $part2_result:expr) => {
            #[test]
            fn $day() {
                let day_number = stringify!($day)
                    .trim_start_matches("day")
                    .parse::<u8>()
                    .expect("Failed to parse day number");

                let data = utils::load_data(day_number);
                assert_eq!(solutions::$day::part1(&data), $part1_result);
                assert_eq!(solutions::$day::part2(&data), $part2_result);
            }
        };
    }

    test_day!(day01, 209, 281);
    test_day!(day02, 8, 2286);
    test_day!(day03, 4361, 467835);
    test_day!(day04, 13, 30);
    test_day!(day05, 35, 46);
    test_day!(day06, 288, 71503);
    test_day!(day07, 6440, 5905);
    test_day!(day08, 6, 6);
    test_day!(day09, 0, 0);
    test_day!(day10, 0, 0);
    test_day!(day11, 0, 0);
    test_day!(day12, 0, 0);
    test_day!(day13, 0, 0);
    test_day!(day14, 0, 0);
    test_day!(day15, 0, 0);
    test_day!(day16, 0, 0);
    test_day!(day17, 0, 0);
    test_day!(day18, 0, 0);
    test_day!(day19, 0, 0);
    test_day!(day20, 0, 0);
    test_day!(day21, 0, 0);
    test_day!(day22, 0, 0);
    test_day!(day23, 0, 0);
    test_day!(day24, 0, 0);
    test_day!(day25, 0, 0);
}
