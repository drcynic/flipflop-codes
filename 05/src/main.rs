use std::{
    collections::{BTreeSet, HashMap},
    fs,
};

fn main() {
    let mut indices_by_tunnel: HashMap<char, [usize; 2]> = HashMap::new();
    let mut unused = Vec::new();
    let tunnels = fs::read_to_string("input2.txt")
        .unwrap()
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            indices_by_tunnel.entry(c).and_modify(|a| a[1] = i).or_insert([i, 0]);
            if !unused.contains(&c) {
                unused.push(c);
            }
            c
        })
        .collect::<Vec<_>>();
    let mut ti = 0;
    let mut steps = 0;
    let mut powered_steps = 0i32;
    loop {
        let c = tunnels[ti];
        if let Some(pos) = unused.iter().position(|&e| e == c) {
            unused.remove(pos);
        }
        let indices = indices_by_tunnel[&c];
        let d = indices[1] - indices[0];
        steps += d;
        powered_steps += if c.is_uppercase() { -(d as i32) } else { d as i32 };
        ti = if ti == indices[0] { indices[1] } else { indices[0] };
        ti += 1;
        if ti == tunnels.len() {
            break;
        }
    }
    println!("part1: {steps:?}");
    println!("part2: {:?}", unused.iter().collect::<String>());
    println!("part3: {powered_steps:?}");
}
