use std::fs;

#[cfg(not(debug_assertions))]
pub fn load_data(day: i8) -> String {
    fs::read_to_string(format!("data/day{day}")).expect("Cannot load file.")
}

#[cfg(debug_assertions)]
pub fn load_data(day: i8) -> String {
    fs::read_to_string(format!("example/day{day}")).expect("Cannot load file.")
}