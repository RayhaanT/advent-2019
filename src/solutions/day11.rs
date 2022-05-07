use super::day09;
use crate::Solution;
use std::collections::HashMap;

fn execute(
    memory: &mut HashMap<usize, i64>,
    mut x: i32,
    mut y: i32,
    mut dir: i32,
    mut field: HashMap<(i32, i32), bool>,
    width: &mut i32,
) -> HashMap<(i32, i32), bool> {
    let mut colour: i64 = -1;
    let mut rot: i64 = -1;
    let mut rel_base = 0;
    let mut addr = 0;
    *width = 0;

    while addr < memory.len() {
        let mode = (memory.get(&addr).unwrap() / 100) as usize;
        addr = match memory.get(&addr).unwrap() % 100 {
            1 => day09::add(memory, addr, mode, rel_base),
            2 => day09::mul(memory, addr, mode, rel_base),
            3 => day09::input(
                memory,
                addr,
                mode,
                rel_base,
                if *field.entry((x, y)).or_insert(false) {
                    1
                } else {
                    0
                },
            ),
            4 => day09::output(
                memory,
                addr,
                mode,
                if colour == -1 { &mut colour } else { &mut rot },
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

        if colour != -1 && rot != -1 {
            *field.entry((x, y)).or_insert(false) = match colour {
                0 => false,
                1 => true,
                _ => panic!("Invalid colour"),
            };
            dir =
                (dir + match rot {
                    0 => -1,
                    1 => 1,
                    _ => panic!("Invalid rotation"),
                } + 4)
                    % 4;
            x += match dir {
                1 => 1,
                3 => -1,
                _ => 0,
            };
            y += match dir {
                0 => 1,
                2 => -1,
                _ => 0,
            };
            colour = -1;
            rot = -1;
            if x > *width {
                *width = x;
            }
        }
    }
    *width += 1;
    field
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

    let mut width = 0;
    let field: HashMap<(i32, i32), bool> =
        execute(&mut memory.clone(), 0, 0, 0, HashMap::new(), &mut width);
    let part_one = field.len();

    let mut white_field: HashMap<(i32, i32), bool> = HashMap::new();
    white_field.insert((0, 5), true);
    let mut correct_field: Vec<((i32, i32), bool)> =
        execute(&mut memory.clone(), 0, 5, 0, white_field, &mut width)
            .iter()
            .map(|t| (*t.0, *t.1))
            .collect();

    correct_field.sort_by_key(|((x, y), _c)| (-y * width) + x);

    let mut output = String::from("\n");
    let mut last_y = correct_field[0].0 .1;
    // Need offsetting cause lines are not all the same length
    // Want them all to start same point so wait until the first #
    // Kinda cheating cause first letter is B
    let mut offsetting = true;
    for (p, c) in correct_field {
        if p.1 != last_y {
            last_y = p.1;
            output = format!("{}\n", output);
            offsetting = true;
        }
        if c {
            offsetting = false;
            output = format!("{}{}", output, "#");
        } else {
            if !offsetting {
                output = format!("{}{}", output, " ");
            }
        }
    }

    Solution {
        first: part_one.to_string(),
        second: output,
    }
}
