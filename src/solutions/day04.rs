use crate::Solution;

fn repeats(mut n: i32) -> bool {
    while n > 0 {
        if (n % 100) % 11 == 0 {
            return true;
        }
        n /= 10;
    }
    false
}

fn strict_repeats(mut n: i32) -> bool {
    while n > 0 {
        if (n % 100) % 11 == 0 {
            if n < 100 || (n / 100) % 10 != n % 10 {
                return true;
            }
            let block_dig = n % 10;
            while n % 10 == block_dig {
                n /= 10;
            }
            continue;
        }
        n /= 10;
    }
    false
}

fn nondec(mut n: i32) -> bool {
    let mut low_dig = n % 10;
    n /= 10;
    while n > 0 {
        if low_dig < n % 10 {
            return false;
        }
        low_dig = n % 10;
        n /= 10;
    }
    true
}

pub fn solve(input: String) -> Solution {
    let bounds: Vec<i32> = input
        .split("-")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let low = bounds[0];
    let high = bounds[1];
    let mut possibilities = 0;
    let mut strict_possibilities = 0;

    for i in low..high {
        if repeats(i) && nondec(i) && i != 111111 {
            possibilities += 1;
            if strict_repeats(i) {
                strict_possibilities += 1;
            }
        }
    }

    Solution {
        first: possibilities.to_string(),
        second: strict_possibilities.to_string(),
    }
}
