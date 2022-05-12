use crate::Solution;

fn phase(input: &Vec<i32>, pattern: &Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();

    for dig in 0..input.len() {
        let mut out_dig = 0;
        for i in 1..(input.len() + 1) {
            out_dig += input[i - 1] * pattern[(i / (dig + 1)) % pattern.len()];
            // out_dig = out_dig % 10;
        }
        output.push(out_dig.abs() % 10);
    }

    output
}

pub fn solve(input: String) -> Solution {
    let mut nums: Vec<i32> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let base_pattern = vec![0, 1, 0, -1];
    let phases = 100;
    for _i in 0..phases {
        nums = phase(&nums, &base_pattern);
    }

    let mut part_one = String::from("");
    for n in 0..8 {
        part_one = format!("{}{}", part_one, nums[n]);
    }

    Solution {
        first: part_one,
        second: String::from("Incomplete"),
    }
}
