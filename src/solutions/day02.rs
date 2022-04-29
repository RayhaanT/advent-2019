use crate::Solution;

fn mem_dump(mem: &Vec<usize>) {
    for m in mem {
        print!("{},", m);
    }
    println!("");
}

fn add(mem: &mut Vec<usize>, addr: usize) -> usize {
    let target_addr = mem[addr + 3];
    mem[target_addr] = mem[mem[addr + 1]] + mem[mem[addr + 2]];
    addr + 4
}

fn mul(mem: &mut Vec<usize>, addr: usize) -> usize {
    let target_addr = mem[addr + 3];
    mem[target_addr] = mem[mem[addr + 1]] * mem[mem[addr + 2]];
    addr + 4
}

fn execute(memory: &mut Vec<usize>) -> usize {
    let mut addr = 0;
    while addr < memory.len() {
        addr = match memory[addr] {
            1 => add(memory, addr),
            2 => mul(memory, addr),
            99 => break,
            _ => panic!("Invalid opcode"),
        };
    }
    memory[0]
}

pub fn solve(input: String, release: bool) -> Solution {
    let mut memory: Vec<usize> = input
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let mut noun = 12;
    let mut verb = 2;
    if release {
        memory[1] = noun;
        memory[2] = verb;
    }

    let part_one = execute(&mut memory.clone());
    let mut result = part_one;
    noun = 99;
    verb = 99;

    if release {
        while result != 19690720 {
            noun = (noun + 1) % 100;
            verb = if noun == 0 { (verb + 1) % 100 } else { verb };
            memory[1] = noun;
            memory[2] = verb;
            result = execute(&mut memory.clone());
        }
    }
    mem_dump(&memory);

    Solution {
        first: part_one.to_string(),
        second: (noun * 100 + verb).to_string(),
    }
}
