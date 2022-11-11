use std::fs;

const LOG_PATH: &str = "./log";

pub fn log(append_str: impl std::fmt::Display) {
    let contents = fs::read_to_string(LOG_PATH).unwrap();
    fs::write(LOG_PATH, format!("{}\n{}", contents, append_str)).unwrap();
}

pub fn clear() {
    fs::write(LOG_PATH, "").unwrap();
}
