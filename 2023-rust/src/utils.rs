use std::fs;

pub fn load_data(day: i8, test: bool) -> String {
    if test {
        return fs::read_to_string(format!("example/day{day}")).expect("Cannot load file.");
    }
    fs::read_to_string(format!("data/day{day}")).expect("Cannot load file.")
}