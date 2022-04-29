use std::env;
use std::fs;

pub fn get_input(day: i32) -> String {
    let working_dir = env::current_dir().unwrap();
    let fp = working_dir
        .join("inputs")
        .join(format!("day{:02}.txt", day));

    let f = fs::read_to_string(fp);
    f.expect("Error loading input file").trim().to_string()
}
