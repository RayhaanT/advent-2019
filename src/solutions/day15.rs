use super::day09;
use crate::Solution;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone)]
struct MemState {
    mem: HashMap<usize, i64>,
    addr: usize,
    rel_base: usize,
}

#[derive(Eq, PartialEq)]
struct Space {
    pos: (i32, i32),
    cost: u32,
    mem_state: MemState,
    status: u8,
}

impl Ord for Space {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Space {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn execute(mem_state: &mut MemState, out: &mut i64, input: i64) {
    let memory = &mut mem_state.mem;
    let addr = &mut mem_state.addr;
    let rel_base = &mut mem_state.rel_base;
    let mode = (memory.get(&addr).unwrap() / 100) as usize;
    *addr = match memory.get(&addr).unwrap() % 100 {
        1 => day09::add(memory, *addr, mode, *rel_base),
        2 => day09::mul(memory, *addr, mode, *rel_base),
        3 => day09::input(memory, *addr, mode, *rel_base, input),
        4 => day09::output(memory, *addr, mode, out, *rel_base, true),
        5 => day09::jump_if_true(memory, *addr, mode, *rel_base),
        6 => day09::jump_if_false(memory, *addr, mode, *rel_base),
        7 => day09::less_than(memory, *addr, mode, *rel_base),
        8 => day09::equals(memory, *addr, mode, *rel_base),
        9 => day09::adjust_base(memory, *addr, mode, rel_base),
        99 => *addr,
        _ => panic!("Invalid opcode"),
    };
}

fn move_drone(mem_state: &mut MemState, move_cmd: i64) -> u8 {
    let mut out = -1;
    let mut last_addr = mem_state.addr + 1;
    while mem_state.addr != last_addr && out == -1 {
        last_addr = mem_state.addr;
        execute(mem_state, &mut out, move_cmd);
    }
    out as u8
}

fn test_neighbours(
    this_space: Space,
    open: &mut BinaryHeap<Space>,
    closed: &mut HashSet<(i32, i32)>,
) {
    for dir in 1..5 {
        let x = this_space.pos.0
            + match dir {
                1 | 2 => 0,
                3 => -1,
                4 => 1,
                _ => panic!("Invalid direction"),
            };
        let y = this_space.pos.1
            + match dir {
                3 | 4 => 0,
                1 => 1,
                2 => -1,
                _ => panic!("Invalid direction"),
            };

        if !closed.contains(&(x, y)) {
            let mut new_state = this_space.mem_state.clone();
            let status = move_drone(&mut new_state, dir);
            if status == 0 {
                closed.insert((x, y));
            } else {
                open.push(Space {
                    pos: (x, y),
                    cost: this_space.cost + 1,
                    mem_state: new_state,
                    status,
                })
            }
        }
    }
    closed.insert(this_space.pos);
}

pub fn solve(input: String) -> Solution {
    let memory = day09::parse_program(input);
    let mut open: BinaryHeap<Space> = BinaryHeap::new();
    open.push(Space {
        pos: (0, 0),
        cost: 0,
        mem_state: MemState {
            mem: memory.clone(),
            addr: 0,
            rel_base: 0,
        },
        status: 0,
    });
    let mut closed: HashSet<(i32, i32)> = HashSet::new();

    let mut oxygen_dist: u32 = 0;
    let mut oxygen_pos: (i32, i32) = (0, 0);
    let mut oxygen_state: Option<MemState> = None;
    while !open.is_empty() {
        let this_space = open.pop().unwrap();
        if this_space.status == 2 {
            oxygen_dist = this_space.cost;
            oxygen_pos = this_space.pos;
            oxygen_state = Some(this_space.mem_state.clone());
            break;
        }
        test_neighbours(this_space, &mut open, &mut closed);
    }

    open = BinaryHeap::new();
    open.push(Space {
        pos: oxygen_pos,
        cost: 0,
        mem_state: oxygen_state.unwrap(),
        status: 2,
    });
    closed = HashSet::new();
    let mut max_cost: u32 = 0;
    while !open.is_empty() {
        let this_space = open.pop().unwrap();
        if this_space.cost > max_cost {
            max_cost = this_space.cost;
        }
        test_neighbours(this_space, &mut open, &mut closed);
    }

    Solution {
        first: oxygen_dist.to_string(),
        second: max_cost.to_string(),
    }
}
