use crate::Solution;
use std::collections::HashMap;

pub fn parse_mode(
    mem: &mut HashMap<usize, i64>,
    addr: usize,
    mode: usize,
    place: i64,
    relative_base: usize,
) -> usize {
    match match place {
        1 => mode % 10,
        2 => (mode / 10) % 10,
        3 => mode / 100,
        _ => panic!("Invalid mode index"),
    } {
        1 => addr + place as usize,
        0 => *mem.entry(addr + (place as usize)).or_insert(0) as usize,
        2 => ((relative_base as i64) + *mem.entry(addr + (place as usize)).or_insert(0)) as usize,
        _ => panic!("Invalid parameter mode"),
    }
}

pub fn mode_val(
    mut mem: &mut HashMap<usize, i64>,
    addr: usize,
    mode: usize,
    place: i64,
    rel_base: usize,
) -> i64 {
    let ind = parse_mode(&mut mem, addr, mode, place, rel_base);
    fetch_mem(&mut mem, ind)
}

pub fn fetch_mem(mem: &mut HashMap<usize, i64>, addr: usize) -> i64 {
    *mem.entry(addr).or_insert(0)
}

pub fn set_mem(mem: &mut HashMap<usize, i64>, addr: usize, val: i64) {
    *mem.entry(addr).or_insert(0) = val;
}

pub fn adjust_base(
    mem: &mut HashMap<usize, i64>,
    addr: usize,
    mode: usize,
    rel_base: &mut usize,
) -> usize {
    let old_base = *rel_base;
    *rel_base = (old_base as i64 + mode_val(mem, addr, mode, 1, old_base)) as usize;
    addr + 2
}

fn execute(memory: &mut HashMap<usize, i64>, the_input: i64) -> i64 {
    let mut out: i64 = 0;
    let mut addr = 0;
    let mut rel_base = 0;
    while addr < memory.len() {
        let mode = (memory.get(&addr).unwrap() / 100) as usize;
        addr = match memory.get(&addr).unwrap() % 100 {
            1 => add(memory, addr, mode, rel_base),
            2 => mul(memory, addr, mode, rel_base),
            3 => input(memory, addr, mode, rel_base, the_input),
            4 => output(memory, addr, mode, &mut out, rel_base),
            5 => jump_if_true(memory, addr, mode, rel_base),
            6 => jump_if_false(memory, addr, mode, rel_base),
            7 => less_than(memory, addr, mode, rel_base),
            8 => equals(memory, addr, mode, rel_base),
            9 => adjust_base(memory, addr, mode, &mut rel_base),
            99 => break,
            _ => panic!("Invalid opcode"),
        };
    }
    out
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

    let part_one = execute(&mut memory.clone(), 1);

    Solution {
        first: part_one.to_string(),
        second: String::from("Incomplete"),
    }
}

pub fn add(mut mem: &mut HashMap<usize, i64>, addr: usize, mode: usize, rel_base: usize) -> usize {
    let target_addr = parse_mode(mem, addr, mode, 3, rel_base);
    let result = mode_val(mem, addr, mode, 1, rel_base) + mode_val(mem, addr, mode, 2, rel_base);
    set_mem(&mut mem, target_addr as usize, result);
    addr + 4
}

pub fn mul(mut mem: &mut HashMap<usize, i64>, addr: usize, mode: usize, rel_base: usize) -> usize {
    let target_addr = parse_mode(mem, addr, mode, 3, rel_base);
    let result = mode_val(mem, addr, mode, 1, rel_base) * mode_val(mem, addr, mode, 2, rel_base);
    set_mem(&mut mem, target_addr as usize, result);
    addr + 4
}

pub fn input(
    mut mem: &mut HashMap<usize, i64>,
    addr: usize,
    mode: usize,
    rel_base: usize,
    the_input: i64,
) -> usize {
    let loc = parse_mode(&mut mem, addr, mode, 1, rel_base);
    set_mem(&mut mem, loc as usize, the_input);
    addr + 2
}

pub fn output(
    mem: &mut HashMap<usize, i64>,
    addr: usize,
    mode: usize,
    out: &mut i64,
    rel_base: usize,
) -> usize {
    *out = mode_val(mem, addr, mode, 1, rel_base);
    println!("{}", out);
    addr + 2
}

pub fn jump_if_true(
    mem: &mut HashMap<usize, i64>,
    addr: usize,
    mode: usize,
    rel_base: usize,
) -> usize {
    let cond = mode_val(mem, addr, mode, 1, rel_base);
    if cond != 0 {
        return mode_val(mem, addr, mode, 2, rel_base) as usize;
    }
    addr + 3
}

pub fn jump_if_false(
    mem: &mut HashMap<usize, i64>,
    addr: usize,
    mode: usize,
    rel_base: usize,
) -> usize {
    let cond = mode_val(mem, addr, mode, 1, rel_base);
    if cond == 0 {
        return mode_val(mem, addr, mode, 2, rel_base) as usize;
    }
    addr + 3
}

pub fn less_than(
    mut mem: &mut HashMap<usize, i64>,
    addr: usize,
    mode: usize,
    rel_base: usize,
) -> usize {
    let target = parse_mode(mem, addr, mode, 3, rel_base);
    if mode_val(mem, addr, mode, 1, rel_base) < mode_val(mem, addr, mode, 2, rel_base) {
        set_mem(&mut mem, target, 1);
    } else {
        set_mem(&mut mem, target, 0);
    }
    addr + 4
}

pub fn equals(
    mut mem: &mut HashMap<usize, i64>,
    addr: usize,
    mode: usize,
    rel_base: usize,
) -> usize {
    let target = parse_mode(mem, addr, mode, 3, rel_base);
    let first = mode_val(mem, addr, mode, 1, rel_base);
    let second = mode_val(mem, addr, mode, 2, rel_base);
    if first == second {
        set_mem(&mut mem, target, 1);
    } else {
        set_mem(&mut mem, target, 0);
    }
    addr + 4
}
