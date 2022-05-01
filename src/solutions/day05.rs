use crate::Solution;

#[allow(dead_code)]
pub fn mem_dump(mem: &Vec<i32>) -> String {
    let mut out: String = String::from("");
    for m in mem {
        out = format!("{}{},", out, m);
    }
    out.pop();
    out
}

pub fn parse_mode(mem: &Vec<i32>, addr: usize, mode: usize, place: i32) -> usize {
    if match place {
        1 => mode % 10,
        2 => (mode / 10) % 10,
        3 => mode / 100,
        _ => panic!("Invalid mode index"),
    } == 1
    {
        addr + place as usize
    } else {
        mem[addr + (place as usize)] as usize
    }
}

pub fn mode_val(mem: &Vec<i32>, addr: usize, mode: usize, place: i32) -> i32 {
    mem[parse_mode(&mem, addr, mode, place)]
}

pub fn add(mem: &mut Vec<i32>, addr: usize, mode: usize) -> usize {
    let target_addr = parse_mode(&mem, addr, mode, 3);
    mem[target_addr as usize] = mode_val(&mem, addr, mode, 1) + mode_val(&mem, addr, mode, 2);
    addr + 4
}

pub fn mul(mem: &mut Vec<i32>, addr: usize, mode: usize) -> usize {
    let target_addr = parse_mode(&mem, addr, mode, 3);
    mem[target_addr as usize] = mode_val(&mem, addr, mode, 1) * mode_val(&mem, addr, mode, 2);
    addr + 4
}

pub fn input(mem: &mut Vec<i32>, addr: usize, the_input: i32) -> usize {
    let loc = mem[addr + 1];
    mem[loc as usize] = the_input;
    addr + 2
}

pub fn output(mem: &mut Vec<i32>, addr: usize, mode: usize, out: &mut i32) -> usize {
    *out = mode_val(&mem, addr, mode, 1);
    println!("{}", out);
    addr + 2
}

pub fn jump_if_true(mem: &mut Vec<i32>, addr: usize, mode: usize) -> usize {
    let cond = mode_val(&mem, addr, mode, 1);
    if cond != 0 {
        return mode_val(&mem, addr, mode, 2) as usize;
    }
    addr + 3
}

pub fn jump_if_false(mem: &mut Vec<i32>, addr: usize, mode: usize) -> usize {
    let cond = mode_val(&mem, addr, mode, 1);
    if cond == 0 {
        return mode_val(&mem, addr, mode, 2) as usize;
    }
    addr + 3
}

pub fn less_than(mem: &mut Vec<i32>, addr: usize, mode: usize) -> usize {
    let target = parse_mode(&mem, addr, mode, 3);
    if mode_val(&mem, addr, mode, 1) < mode_val(&mem, addr, mode, 2) {
        mem[target] = 1;
    }
    mem[target] = 0;
    addr + 4
}

pub fn equals(mem: &mut Vec<i32>, addr: usize, mode: usize) -> usize {
    let target = parse_mode(&mem, addr, mode, 3);
    if mode_val(&mem, addr, mode, 1) == mode_val(&mem, addr, mode, 2) {
        mem[target] = 1;
    }
    mem[target] = 0;
    addr + 4
}

fn execute(memory: &mut Vec<i32>, the_input: i32) -> i32 {
    let mut out: i32 = 0;
    let mut addr = 0;
    while addr < memory.len() {
        let mode = (memory[addr] / 100) as usize;
        addr = match memory[addr] % 100 {
            1 => add(memory, addr, mode),
            2 => mul(memory, addr, mode),
            3 => input(memory, addr, the_input),
            4 => output(memory, addr, mode, &mut out),
            5 => jump_if_true(memory, addr, mode),
            6 => jump_if_false(memory, addr, mode),
            7 => less_than(memory, addr, mode),
            8 => equals(memory, addr, mode),
            99 => break,
            _ => panic!("Invalid opcode"),
        };
    }
    out
}

pub fn solve(input: String) -> Solution {
    let memory: Vec<i32> = input
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let part_one = execute(&mut memory.clone(), 1);
    let part_two = execute(&mut memory.clone(), 5);
    // println!("{}", mem_dump(&memory));

    Solution {
        first: part_one.to_string(),
        second: part_two.to_string(),
    }
}
