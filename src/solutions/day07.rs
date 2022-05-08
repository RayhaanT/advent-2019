use super::day05;
use crate::Solution;
use itertools::Itertools;
use std::cmp;

fn execute(memory: &mut Vec<i32>, phase: i32, amp: i32, pc: &usize) -> (i32, usize, bool) {
    let mut phase_next = if *pc == 0 { true } else { false };
    let mut out: i32 = 0;
    let mut addr = *pc;
    while addr < memory.len() {
        let mode = (memory[addr] / 100) as usize;
        addr = match memory[addr] % 100 {
            1 => day05::add(memory, addr, mode),
            2 => day05::mul(memory, addr, mode),
            3 => day05::input(
                memory,
                addr,
                if phase_next {
                    phase_next = !phase_next;
                    phase
                } else {
                    phase_next = !phase_next;
                    amp
                },
            ),
            4 => {
                day05::output(memory, addr, mode, &mut out, true);
                return (out, addr + 2, false);
            }
            5 => day05::jump_if_true(memory, addr, mode),
            6 => day05::jump_if_false(memory, addr, mode),
            7 => day05::less_than(memory, addr, mode),
            8 => day05::equals(memory, addr, mode),
            99 => return (out, addr, true),
            _ => panic!("Invalid opcode"),
        };
    }
    panic!("Invalid halt");
}

pub fn solve(input: String) -> Solution {
    let memory: Vec<i32> = input
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut part_one = 0;
    let perms = (0..5).permutations(5);
    for p in perms {
        let mut signal = 0;
        for phase in p {
            let tup = execute(&mut memory.clone(), phase, signal, &0);
            signal = tup.0;
        }
        part_one = cmp::max(part_one, signal);
    }

    let mut part_two = 0;
    let high_perms = (5..10).permutations(5);
    // let high_perms = vec![vec![9, 7, 8, 5, 6]];
    for p in high_perms {
        let mut mems: Vec<(Vec<i32>, usize)> = vec![
            (memory.clone(), 0),
            (memory.clone(), 0),
            (memory.clone(), 0),
            (memory.clone(), 0),
            (memory.clone(), 0),
        ];
        let mut signal = 0;
        let mut halted = false;
        while !halted {
            for phase in 0..p.len() {
                let addr = mems[phase].1;
                let tup = execute(&mut mems[phase].0, p[phase], signal, &addr);
                signal = tup.0;
                mems[phase].1 = tup.1;
                halted = tup.2;
                if halted {
                    break;
                }
            }
            part_two = cmp::max(signal, part_two);
        }
    }

    Solution {
        first: part_one.to_string(),
        second: part_two.to_string(),
    }
}
