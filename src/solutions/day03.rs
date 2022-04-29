use crate::Solution;
use std::collections::HashMap;

struct Footprint {
    path_len: i32,
    owner: usize,
}

pub fn solve(input: String) -> Solution {
    let wires: Vec<Vec<&str>> = input.split("\n").map(|l| l.split(",").collect()).collect();
    let mut grid = HashMap::new();

    let mut closest = -1;
    let mut shortest = -1;

    for (ind, path) in wires.iter().enumerate() {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut steps = 0;
        for dir in path {
            let (dx, dy) = match dir.chars().nth(0).unwrap() {
                'R' => (1, 0),
                'L' => (-1, 0),
                'U' => (0, 1),
                'D' => (0, -1),
                _ => panic!("Invalid input"),
            };
            let dist = dir[1..].parse::<i32>().unwrap();

            for _i in 0..dist {
                if grid.contains_key(&(x, y)) && !(x == 0 && y == 0) {
                    let manhattan = x.abs() + y.abs();
                    if manhattan < closest || closest == -1 {
                        closest = manhattan;
                    }
                    let hist: &Footprint = grid.get(&(x, y)).unwrap();
                    if (steps + hist.path_len < shortest || shortest == -1) && hist.owner != ind {
                        shortest = steps + hist.path_len;
                    }
                } else {
                    grid.insert(
                        (x, y),
                        Footprint {
                            path_len: steps,
                            owner: ind,
                        },
                    );
                }
                x += dx;
                y += dy;
                steps += 1;
            }
        }
    }

    return Solution {
        first: closest.to_string(),
        second: shortest.to_string(),
    };
}
