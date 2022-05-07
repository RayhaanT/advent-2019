use super::day09;
use crate::Solution;
use std::collections::HashMap;

#[derive(PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

fn parse_tile(id: i64) -> Tile {
    match id {
        0 => Tile::Empty,
        1 => Tile::Wall,
        2 => Tile::Block,
        3 => Tile::Paddle,
        4 => Tile::Ball,
        _ => panic!("Invalid tile id"),
    }
}

fn execute(memory: &mut HashMap<usize, i64>) -> (i64, i64) {
    let mut addr = 0;
    let mut rel_base = 0;

    let mut x = -2;
    let mut y = -2;
    let mut tid = -2;

    let mut block_count = 0;

    let mut screen: HashMap<(u32, u32), Tile> = HashMap::new();
    let mut score = 0;
    let mut paddle = 0;
    let mut ball = (0, 0);
    let mut tilt = 0;

    while addr < memory.len() {
        let mode = (memory.get(&addr).unwrap() / 100) as usize;
        addr = match memory.get(&addr).unwrap() % 100 {
            1 => day09::add(memory, addr, mode, rel_base),
            2 => day09::mul(memory, addr, mode, rel_base),
            3 => day09::input(memory, addr, mode, rel_base, tilt),
            4 => day09::output(
                memory,
                addr,
                mode,
                if x == -2 {
                    &mut x
                } else {
                    if y == -2 {
                        &mut y
                    } else {
                        &mut tid
                    }
                },
                rel_base,
                true,
            ),
            5 => day09::jump_if_true(memory, addr, mode, rel_base),
            6 => day09::jump_if_false(memory, addr, mode, rel_base),
            7 => day09::less_than(memory, addr, mode, rel_base),
            8 => day09::equals(memory, addr, mode, rel_base),
            9 => day09::adjust_base(memory, addr, mode, &mut rel_base),
            99 => break,
            _ => panic!("Invalid opcode"),
        };

        if tid == 2 {
            block_count += 1;
        }
        if tid != -2 && x != -2 && y != -2 {
            if x == -1 {
                score = tid;
            } else {
                let tile = parse_tile(tid);
                if tile == Tile::Ball {
                    ball = (x, y);
                } else if tile == Tile::Paddle {
                    paddle = x;
                }
                *screen.entry((x as u32, y as u32)).or_insert(Tile::Empty) = tile;
            }

            tid = -2;
            x = -2;
            y = -2;
        }

        tilt = if paddle - ball.0 == 0 {
            0
        } else {
            if paddle - ball.0 > 0 {
                -1
            } else {
                1
            }
        };
    }
    (block_count, score)
}

pub fn solve(input: String) -> Solution {
    let program: Vec<i64> = input
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let mut memory: HashMap<usize, i64> = HashMap::new();
    for (ind, val) in program.iter().enumerate() {
        memory.insert(ind, *val);
    }

    let part_one = execute(&mut memory.clone()).0;

    memory.insert(0, 2);
    let score = execute(&mut memory.clone()).1;

    Solution {
        first: part_one.to_string(),
        second: score.to_string(),
    }
}
