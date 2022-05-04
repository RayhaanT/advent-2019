use crate::Solution;
use std::cmp;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Quadrant {
    TopRight,
    TopLeft,
    BotRight,
    BotLeft,
}

fn frac_reduce(mut frac: (i32, i32)) -> (i32, i32) {
    if frac.1 == 0 {
        panic!("Division by 0");
    }
    if frac.0 == 0 {
        return (0, 0);
    }
    let mut divisor = 2;
    while divisor <= frac.0 && divisor <= frac.1 {
        while frac.0 % divisor == 0 && frac.1 % divisor == 0 {
            frac.0 /= divisor;
            frac.1 /= divisor;
        }
        divisor += 1;
    }
    frac
}

pub fn solve(input: String) -> Solution {
    let width = input.chars().position(|c| c == '\n').unwrap();
    let asteroids: Vec<(i32, i32)> = input
        .replace("\n", "")
        .chars()
        .enumerate()
        .map(|(pos, c)| (((pos % width) as i32, (pos / width) as i32), c))
        .filter(|c| c.1 == '#')
        .map(|t| t.0)
        .collect();

    let mut most = 0;
    for i in 0..asteroids.len() {
        let mut paths: HashSet<(Quadrant, (i32, i32))> = HashSet::new();
        for j in 0..asteroids.len() {
            if i == j {
                continue;
            }
            let dx = asteroids[j].0 - asteroids[i].0;
            let dy = asteroids[j].1 - asteroids[i].1;
            let quad: Quadrant;
            if dx > 0 && dy >= 0 {
                quad = Quadrant::TopRight;
            } else if dx <= 0 && dy > 0 {
                quad = Quadrant::TopLeft;
            } else if dx < 0 && dy <= 0 {
                quad = Quadrant::BotLeft;
            } else {
                quad = Quadrant::BotRight;
            }

            paths.insert((
                quad,
                match quad {
                    Quadrant::TopRight | Quadrant::BotLeft => frac_reduce((dy.abs(), dx.abs())),
                    Quadrant::TopLeft | Quadrant::BotRight => frac_reduce((dx.abs(), dy.abs())),
                },
            ));
        }
        // println!("({}, {}) {}", asteroids[i].0, asteroids[i].1, paths.len());
        most = cmp::max(most, paths.len());
    }

    Solution {
        first: most.to_string(),
        second: String::from("Incomplete"),
    }
}
