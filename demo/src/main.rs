use std::{collections::HashMap, fs};

use itertools::Itertools;

fn main() {
    let nums = fs::read_to_string("input2.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let part1 = nums.iter().sum::<u32>();
    println!("part1: {part1}");

    let part2 = part1.div_ceil(nums.len() as u32);
    println!("part2: {part2}");

    let mut count_by_numbers = HashMap::new();
    let mut count_by_digits = HashMap::new();
    nums.iter().for_each(|&n| {
        count_by_numbers.entry(n).and_modify(|count| *count += 1).or_insert(1);
        format!("{n}").as_bytes().iter().map(|&b| b - b'0').for_each(|digit| {
            count_by_digits.entry(digit).and_modify(|count| *count += 1).or_insert(1);
        });
    });
    let max_num = count_by_numbers.into_iter().map(|(k, v)| (v, k)).sorted().rev().nth(0).unwrap().1;
    let min_digit = count_by_digits.into_iter().map(|(k, v)| (v, k)).sorted().nth(0).unwrap().1;
    println!("part3: {max_num}{min_digit}");
}
