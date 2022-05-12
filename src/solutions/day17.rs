use super::day09;
use crate::Solution;
use std::collections::{HashMap, HashSet};

fn get_scaffold(
    memory: &mut HashMap<usize, i64>,
    robo_pos: &mut (i32, i32),
) -> HashSet<(i32, i32)> {
    let mut out: i64 = -1;
    let mut addr = 0;
    let mut rel_base = 0;
    let mut scaffold: HashSet<(i32, i32)> = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    let the_input = 0;

    while addr < memory.len() {
        let mode = (memory.get(&addr).unwrap() / 100) as usize;
        addr = match memory.get(&addr).unwrap() % 100 {
            1 => day09::add(memory, addr, mode, rel_base),
            2 => day09::mul(memory, addr, mode, rel_base),
            3 => day09::input(memory, addr, mode, rel_base, the_input),
            4 => day09::output(memory, addr, mode, &mut out, rel_base, true),
            5 => day09::jump_if_true(memory, addr, mode, rel_base),
            6 => day09::jump_if_false(memory, addr, mode, rel_base),
            7 => day09::less_than(memory, addr, mode, rel_base),
            8 => day09::equals(memory, addr, mode, rel_base),
            9 => day09::adjust_base(memory, addr, mode, &mut rel_base),
            99 => break,
            _ => panic!("Invalid opcode"),
        };

        if out != -1 {
            if out == 35 {
                scaffold.insert((x, y));
                print!("#");
                x += 1;
            } else if out == 46 {
                print!(" ");
                x += 1;
            } else if out == 10 {
                println!("");
                y += 1;
                x = 0;
            } else {
                print!("{}", out as u8 as char);
                *robo_pos = (x, y);
            }
            out = -1;
        }
    }
    scaffold
}

pub fn solve(input: String) -> Solution {
    let memory: HashMap<usize, i64> = day09::parse_program(input);
    let mut robot = (0, 0);
    let scaffold = get_scaffold(&mut memory.clone(), &mut robot);

    let mut part_one: u32 = 0;

    for point in scaffold.clone().iter() {
        let mut intersection = true;
        for dir in 0..4 {
            let neighbour = (
                point.0
                    + match dir {
                        0 | 2 => 0,
                        1 => 1,
                        3 => -1,
                        _ => panic!("Broken for loop"),
                    },
                point.1
                    + match dir {
                        1 | 3 => 0,
                        0 => 1,
                        2 => -1,
                        _ => panic!("Broken for loop"),
                    },
            );
            if !scaffold.contains(&neighbour) {
                intersection = false;
                break;
            }
        }
        if intersection {
            part_one += (point.0 * point.1) as u32;
        }
    }

    Solution {
        first: part_one.to_string(),
        second: String::from("Incomplete"),
    }
}
