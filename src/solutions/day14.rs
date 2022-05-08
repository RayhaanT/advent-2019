use crate::Solution;
use std::collections::HashMap;

fn ore_cost(reactions: &HashMap<&str, (Vec<(&str, u64)>, u64)>, count: u64) -> u64 {
    let mut sources: HashMap<&str, u64> = HashMap::new();
    for (out, ins) in reactions.iter() {
        if ins.0[0].0 == "ORE" {
            sources.insert(out, 0);
        }
    }

    let mut targets: HashMap<&str, u64> = HashMap::new();
    targets.insert("FUEL", count);
    let mut inventory: HashMap<&str, i64> = HashMap::new();
    while targets.len() > 0 {
        let name: &str;

        {
            let this_target = targets.iter().nth(0).unwrap();
            name = *this_target.0;
            let mut needed = *this_target.1 as i64;
            if inventory.contains_key(name) {
                let stash = *inventory.get(name).unwrap();
                needed -= stash;
                *inventory.get_mut(name).unwrap() = -needed;
                if *inventory.get(name).unwrap() <= 0 {
                    inventory.remove(name);
                }
                if needed < 0 {
                    needed = 0;
                }
            }

            if needed != 0 {
                let inputs = reactions.get(this_target.0).unwrap();

                let num_reactions =
                    needed as u64 / inputs.1 + if needed as u64 % inputs.1 == 0 { 0 } else { 1 };
                let mut leftover = inputs.1 - (needed as u64 % inputs.1);
                if leftover == inputs.1 {
                    leftover = 0;
                }
                if leftover != 0 {
                    *inventory.entry(this_target.0).or_insert(0) += leftover as i64;
                }

                for (id, count) in &inputs.0 {
                    if sources.contains_key(id) {
                        *sources.get_mut(id).unwrap() += count * num_reactions;
                    } else {
                        *targets.entry(id).or_insert(0) += count * num_reactions;
                    }
                }
            }
        }

        targets.remove(name);
    }

    let mut ore: u64 = 0;
    for (name, count) in sources {
        let recipe = reactions.get(name).unwrap();
        let new_ore = recipe.0[0].1;
        let num_reactions = count / recipe.1 + if count % recipe.1 == 0 { 0 } else { 1 };
        ore += (new_ore * num_reactions) as u64;
    }

    ore
}

pub fn solve(input: String) -> Solution {
    let mut reactions: HashMap<&str, (Vec<(&str, u64)>, u64)> = HashMap::new();

    for (ins, mut out) in input.split("\n").map(|line| {
        let mut e = line.split("=>");
        (
            e.next()
                .unwrap()
                .split(",")
                .map(|s| {
                    let p: Vec<&str> = s.trim().split(" ").collect();
                    (p[1], p[0].parse::<u64>().unwrap())
                })
                .collect::<Vec<(&str, u64)>>(),
            e.next().unwrap().trim().split(" "),
        )
    }) {
        let count = out.nth(0).unwrap().parse::<u64>().unwrap();
        let name = out.nth(0).unwrap();
        reactions.insert(name, (ins, count));
    }

    let one_cost = ore_cost(&reactions, 1);

    // Binary search to find best highest fuel with <10..00 ore
    let all_ore: u64 = 1000000000000;
    let mut low = all_ore / one_cost;
    let mut high = low * 2;
    while low <= high {
        let mid = (low + high) / 2;
        let new_ore = ore_cost(&reactions, mid);
        if new_ore <= all_ore {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    Solution {
        first: one_cost.to_string(),
        second: high.to_string(),
    }
}
