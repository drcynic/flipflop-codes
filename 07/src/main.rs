use std::fs;

fn main() {
    let sizes = fs::read_to_string("input2.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|l| {
            let (l, r) = l.split_once(' ').unwrap();
            (l.parse::<u128>().unwrap(), r.parse::<u128>().unwrap())
        })
        .collect::<Vec<_>>();

    println!("part1: {}", sizes.iter().map(|&(r, c)| paths_3d((r, c, 1))).sum::<u128>());
    println!("part2: {}", sizes.iter().map(|&(r, c)| paths_3d((r, c, r))).sum::<u128>());
    println!("part3: {}", sizes.iter().map(|&(d, s)| paths_nd((d, s))).sum::<u128>());
}

pub fn factorial(num: u128) -> u128 {
    (1..=num).product()
}

fn paths_3d((r, c, d): (u128, u128, u128)) -> u128 {
    let n = factorial((r - 1) + (c - 1) + (d - 1));
    let k = factorial(r - 1) * factorial(c - 1) * factorial(d - 1);
    n / k
}

fn paths_nd((d, s): (u128, u128)) -> u128 {
    let n = factorial((s - 1) * d);
    let k = (1..=d).map(|_| factorial(s - 1)).product::<u128>();
    n / k
}
