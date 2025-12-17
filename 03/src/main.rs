use std::{collections::HashMap, fs};

use itertools::Itertools;
use num::Signed;

fn main() {
    let mut colors = HashMap::new();
    fs::read_to_string("input2.txt").unwrap().trim().split("\n").for_each(|s| {
        let s = s.trim().split(",").map(|e| e.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let c = [s[0], s[1], s[2]];
        colors.entry(c).and_modify(|v| *v += 1).or_insert(1);
    });
    let part1 = colors.iter().map(|(k, v)| (v, k)).sorted().rev().nth(0).unwrap().1;
    println!("part1: {part1:?}");

    let mut colors = [0; 4];
    fs::read_to_string("input2.txt").unwrap().trim().split("\n").for_each(|s| {
        let s = s.trim().split(",").map(|e| e.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let (r, g, b) = (s[0], s[1], s[2]);
        let idx = if r == g || g == b || r == b {
            3
        } else {
            if r > g && r > b {
                0
            } else if g > r && g > b {
                1
            } else {
                2
            }
        };
        colors[idx] += 1;
    });
    let part2 = colors[1];
    println!("part2: {part2:?}");

    let part3 = colors
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            v * match i {
                0 => 5,
                1 => 2,
                2 => 4,
                3 => 10,
                _ => unreachable!(),
            }
        })
        .sum::<u32>();
    println!("Part3: {part3}");
}
