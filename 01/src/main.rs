use std::fs;

use regex::Regex;

fn main() {
    let re = Regex::new(r"(ba|na|ne)").unwrap();
    let part1 = fs::read_to_string("input2.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|s| re.find_iter(s).count())
        .sum::<usize>();
    println!("part1: {part1}");

    let re_na = Regex::new(r"(na|ne)").unwrap();
    let part2 = fs::read_to_string("input2.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|s| {
            let n = re_na.find_iter(s).count();
            if n % 2 == 0 { 0 } else { re.find_iter(s).count() }
        })
        .sum::<usize>();
    println!("part2: {part2}");

    let re_ne = Regex::new(r"(ne)").unwrap();
    let part3 = fs::read_to_string("input2.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|s| {
            let n = re_ne.find_iter(s).count();
            if n > 0 { 0 } else { re.find_iter(s).count() }
        })
        .sum::<usize>();
    println!("part3: {part3}");
}
