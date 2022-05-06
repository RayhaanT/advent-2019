use crate::Solution;
use std::collections::HashMap;

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
        return (0, 1);
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
    let mut station = (0, 0);
    let mut most_paths: HashMap<(Quadrant, (i32, i32)), Vec<(i32, i32)>> = HashMap::new();
    for i in 0..asteroids.len() {
        let mut paths: HashMap<(Quadrant, (i32, i32)), Vec<(i32, i32)>> = HashMap::new();
        for j in 0..asteroids.len() {
            if i == j {
                continue;
            }
            let dx = asteroids[j].0 - asteroids[i].0;
            let dy = -(asteroids[j].1 - asteroids[i].1); // y-axis upside down
            let quad: Quadrant;
            if dx >= 0 && dy > 0 {
                quad = Quadrant::TopRight;
            } else if dx < 0 && dy >= 0 {
                quad = Quadrant::TopLeft;
            } else if dx <= 0 && dy < 0 {
                quad = Quadrant::BotLeft;
            } else {
                quad = Quadrant::BotRight;
            }

            paths
                .entry((
                    quad,
                    match quad {
                        Quadrant::TopRight | Quadrant::BotLeft => frac_reduce((dx.abs(), dy.abs())),
                        Quadrant::TopLeft | Quadrant::BotRight => frac_reduce((dy.abs(), dx.abs())),
                    },
                ))
                .or_insert(Vec::new())
                .push(asteroids[j]);
        }

        if paths.len() > most {
            most = paths.len();
            most_paths = paths;
            station = asteroids[i];
        }
    }

    let mut sight_lines: Vec<(f32, Vec<(i32, i32)>)> = most_paths
        .into_iter()
        .map(|((quad, pos), grp)| {
            let f = (
                (match quad {
                    Quadrant::TopRight => 0,
                    Quadrant::BotRight => 1,
                    Quadrant::BotLeft => 2,
                    Quadrant::TopLeft => 3,
                } * width) as f32
                    + pos.0 as f32 / pos.1 as f32,
                grp,
            );
            f
        })
        .collect();

    sight_lines.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for i in 0..sight_lines.len() {
        sight_lines[i]
            .1
            .sort_by_key(|(x, y)| (station.0 - x).abs() + (station.1 - y).abs());
    }

    let mut count = if asteroids.len() < 200 {
        asteroids.len() - 2
    } else {
        199
    };

    // for p in &sight_lines {
    //     println!("{}: ", p.0);
    //     for a in &p.1 {
    //         println!("\t({} {})", a.0, a.1);
    //     }
    // }

    let mut rot = 0;
    while count > 0 {
        count -= 1;
        // println!(
        //     "{}: ({}, {})",
        //     200 - count as i64,
        //     sight_lines[rot].1[0].0,
        //     sight_lines[rot].1[0].1
        // );
        sight_lines[rot].1.remove(0);
        if sight_lines[rot].1.len() == 0 {
            sight_lines.remove(rot);
            if rot == sight_lines.len() {
                rot = 0;
            }
        } else {
            rot = (rot + 1) % sight_lines.len();
        }
    }

    Solution {
        first: most.to_string(),
        second: (sight_lines[rot].1[0].0 * 100 + sight_lines[rot].1[0].1).to_string(),
    }
}
