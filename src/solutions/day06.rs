use crate::Solution;
use std::collections::HashMap;

fn backtrace(orbit_tree: &HashMap<&str, &str>, target: &str) -> Vec<String> {
    let mut trace: Vec<String> = vec![target.to_string()];
    while orbit_tree.get(&trace.last().unwrap()[..]).is_some() {
        trace.push(
            orbit_tree
                .get(&trace.last().unwrap()[..])
                .unwrap()
                .to_string(),
        );
    }
    trace
}

pub fn solve(input: String) -> Solution {
    let mut orbit_tree: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut orbit_tree_rev: HashMap<&str, &str> = HashMap::new();
    for s in input.split("\n") {
        let bodies: Vec<&str> = s.split(")").collect();

        let orbits = orbit_tree.get_mut(&bodies[0]);
        if orbits.is_some() {
            orbits.unwrap().push(bodies[1]);
        } else {
            orbit_tree.insert(bodies[0], vec![bodies[1]]);
        }

        orbit_tree_rev.insert(bodies[1], bodies[0]);
    }

    let mut open_set: Vec<(&str, u32)> = vec![("COM", 0)];
    let mut total = 0;
    while open_set.len() != 0 {
        let fetch = orbit_tree.get(&open_set[0].0);
        if fetch.is_some() {
            for s in fetch.unwrap() {
                open_set.push((s, open_set[0].1 + 1));
                total += 1 + open_set[0].1;
            }
        }
        open_set.remove(0);
    }

    let mut san_trace = backtrace(&orbit_tree_rev, "SAN");
    let mut my_trace = backtrace(&orbit_tree_rev, "YOU");
    let mut dist = san_trace.len() + my_trace.len() - 2;
    while san_trace.last().unwrap() == my_trace.last().unwrap() {
        dist -= 2;
        san_trace.remove(san_trace.len() - 1);
        my_trace.remove(my_trace.len() - 1);
    }

    Solution {
        first: total.to_string(),
        second: dist.to_string(),
    }
}
