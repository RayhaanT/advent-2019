mod file_utils;
mod solutions;

pub use crate::solutions::*;
use std::env;

pub struct Solution {
    first: String,
    second: String,
}

fn solve(day: i32, input: String, release: bool) -> Solution {
    match day {
        1 => day01::solve(input),
        2 => day02::solve(input, release),
        3 => day03::solve(input),
        4 => day04::solve(input),
        5 => day05::solve(input),
        6 => day06::solve(input),
        7 => day07::solve(input),
        8 => day08::solve(input),
        9 => day09::solve(input),
        10 => day10::solve(input),
        11 => day11::solve(input),
        12 => day12::solve(input),
        13 => day13::solve(input),
        14 => day14::solve(input),
        15 => day15::solve(input),
        16 => day16::solve(input),
        17 => day17::solve(input),
        18 => day18::solve(input),
        19 => day19::solve(input),
        20 => day20::solve(input),
        21 => day21::solve(input),
        22 => day22::solve(input),
        23 => day23::solve(input),
        24 => day24::solve(input),
        25 => day25::solve(input),
        _ => panic!("This day doesn't exist"),
    }
}

fn get_day_arg() -> i32 {
    let args: Vec<String> = env::args().collect();
    args[1].parse::<i32>().unwrap()
}

fn main() {
    let day = get_day_arg();
    let input = file_utils::get_input(day);
    let solution = solve(day, input, true);

    println!("First: {}", solution.first);
    println!("Second: {}", solution.second);
}

#[cfg(test)]
mod tests {
    use super::*;
    use glob::glob;
    use std::fs;

    pub fn get_samples(day: i32) -> Vec<(String, String)> {
        let cwd = env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        let sample_pattern = format!("{}/samples/day{:02}s*.in", cwd, day);

        let mut tests: Vec<(String, String)> = Vec::new();
        for file_result in
            glob(&sample_pattern).expect("Failed to read glob pattern for sample inputs")
        {
            let file = file_result.unwrap();
            let truth = file
                .clone()
                .into_os_string()
                .into_string()
                .unwrap()
                .replace(".in", ".out");
            tests.push((
                fs::read_to_string(file)
                    .expect("Error reading a test input {:?}")
                    .trim()
                    .to_string(),
                fs::read_to_string(truth)
                    .expect("Error reading a test solution {:?}")
                    .trim()
                    .to_string(),
            ));
        }
        tests
    }

    #[test]
    fn sample_test() {
        let args: Vec<String> = env::args().collect();
        let day = args[2].parse::<i32>().unwrap();

        let sample_pairs = get_samples(day);
        for p in sample_pairs {
            let soln = solve(day, p.0, false);
            println!("First: {}", soln.first);
            println!("Second: {}", soln.second);
            println!("Ground truth: {}\n", p.1);
            assert!(soln.first == p.1 || soln.second == p.1);
        }
    }
}
