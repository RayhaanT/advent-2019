use crate::Solution;
use std::collections::HashMap;

#[derive(Clone)]
struct Moon {
    pos: Vec<i32>,
    vel: Vec<i32>,
}

fn gravitate(moons: &mut Vec<Moon>, a: usize, b: usize) {
    for i in 0..moons[a].pos.len() {
        if moons[a].pos[i] - moons[b].pos[i] == 0 {
            continue;
        } else if moons[a].pos[i] - moons[b].pos[i] > 0 {
            moons[a].vel[i] -= 1;
            moons[b].vel[i] += 1;
        } else {
            moons[a].vel[i] += 1;
            moons[b].vel[i] -= 1;
        }
    }
}

fn energy(m: &Moon) -> u32 {
    let mut kin: u32 = 0;
    let mut pot: u32 = 0;
    for i in 0..m.vel.len() {
        kin += m.vel[i].abs() as u32;
        pot += m.pos[i].abs() as u32;
    }
    kin * pot
}

fn loops(mut posns: Vec<i32>, mut vels: Vec<i32>) -> (u32, u32) {
    let mut cycles = 0;
    let mut seen: HashMap<(Vec<i32>, Vec<i32>), u32> = HashMap::new();

    while !seen.contains_key(&(posns.clone(), vels.clone())) {
        seen.insert((posns.clone(), vels.clone()), cycles);
        cycles += 1;
        for i in 0..posns.len() {
            for j in (i + 1)..posns.len() {
                if posns[i] - posns[j] == 0 {
                    continue;
                }
                if posns[i] - posns[j] > 0 {
                    vels[i] -= 1;
                    vels[j] += 1;
                } else {
                    vels[i] += 1;
                    vels[j] -= 1;
                }
            }
        }

        for i in 0..posns.len() {
            posns[i] += vels[i];
        }
    }

    (cycles, *seen.get(&(posns, vels)).unwrap())
}

fn least_common_multiple(a: u64, b: u64) -> u64 {
    let mut div = 2;
    let mut min = if a < b { a } else { b };

    let mut common: Vec<u64> = Vec::new();
    while div <= min {
        if a % div == 0 && b % div == 0 && min % div == 0 {
            min /= div;
            common.push(div);
        } else {
            div += 1;
        }
    }

    common.iter().fold(a * b, |acc, x| acc / x)
}

pub fn solve(input: String) -> Solution {
    let steps = 1000;
    let mut moons: Vec<Moon> = input
        .split("\n")
        .map(|s| &s[1..(s.len() - 1)])
        .map(|s| s.split(","))
        .map(|s| {
            let mut coords: Vec<i32> = Vec::new();
            for c in s {
                coords.push(c.split("=").nth(1).unwrap().parse::<i32>().unwrap());
            }
            let dim = coords.len();
            Moon {
                pos: coords,
                vel: vec![0; dim],
            }
        })
        .collect();

    let mut loop_lens: Vec<u64> = Vec::new();
    for i in 0..moons[0].pos.len() {
        let t = loops(
            moons.clone().iter().map(|m| m.pos[i]).collect(),
            moons.clone().iter().map(|m| m.vel[i]).collect(),
        );
        println!("{} {}", t.0, t.1);
        loop_lens.push(t.0 as u64);
    }
    let part_two = loop_lens
        .iter()
        .fold(1, |acc, x| least_common_multiple(acc, *x));

    for _step in 0..steps {
        for i in 0..moons.len() {
            for j in (i + 1)..moons.len() {
                if i == j {
                    continue;
                }
                gravitate(&mut moons, i, j);
            }
        }

        for i in 0..moons.len() {
            for j in 0..moons[i].pos.len() {
                moons[i].pos[j] += moons[i].vel[j];
            }
        }
    }

    for i in 0..moons.len() {
        for v in &moons[i].pos {
            print!("{} ", *v);
        }
        print!(" || ");
        for v in &moons[i].vel {
            print!("{} ", *v);
        }
        println!("");
    }

    Solution {
        first: moons.iter().map(|m| energy(m)).sum::<u32>().to_string(),
        second: part_two.to_string(),
    }
}
