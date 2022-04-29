use crate::Solution;

pub fn solve(input: String) -> Solution {
    let splitInput = input.trim_end().split("\n");
    let part_one: i32 = splitInput
        .clone()
        .map(|s| s.parse::<i32>().unwrap() / 3 - 2)
        .sum();
    let part_two: i32 = splitInput
        .map(|s| {
            let mut total = 0;
            let mut num = s.parse::<i32>().unwrap();
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
