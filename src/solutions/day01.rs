use crate::Solution;

pub fn solve(input: String) -> Solution {
    let mut fuel_sum: i32 = input
        .trim_end()
        .split("\n")
        .map(|s| s.parse::<i32>().unwrap() / 3 - 2)
        .sum();
    let part_one = fuel_sum;

    let mut delta_fuel = fuel_sum;
    while delta_fuel > 0 {
        delta_fuel = delta_fuel / 3 - 2;
        fuel_sum += delta_fuel;
    }
    fuel_sum -= delta_fuel;

    Solution {
        first: part_one.to_string(),
        second: fuel_sum.to_string(),
    }
}
