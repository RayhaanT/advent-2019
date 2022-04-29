use crate::Solution;

pub fn solve(input: String) -> Solution {
    let split_input = input
        .trim_end()
        .split("\n")
        .map(|s| s.parse::<i32>().unwrap());
    let part_one: i32 = split_input.clone().map(|i| i / 3 - 2).sum();
    let part_two: i32 = split_input
        .map(|i| {
            let mut total = 0;
            let mut num = i;
            while num > 0 {
                num = num / 3 - 2;
                total += num;
            }
            total - num
        })
        .sum();

    Solution {
        first: part_one.to_string(),
        second: part_two.to_string(),
    }
}
