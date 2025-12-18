use std::fs;

use itertools::Itertools;

fn main() {
    let mut pos = (0, 0);
    let part1 = fs::read_to_string("input2.txt").unwrap().trim().split("\n").fold(0, |acc, s| {
        let (x, y) = s.trim().split_once(",").unwrap();
        let (x, y) = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
        let (dx, dy) = (x - pos.0, y - pos.1);
        // println!("({},{}) -> ({},{})", pos.0, pos.1, x, y);
        // println!("dx: {}, dy: {}", dx, dy);
        pos = (pos.0 + dx, pos.1 + dy);
        acc + dx.abs() + dy.abs()
    });
    println!("part1: {part1:?}");

    let part2 = fs::read_to_string("input2.txt").unwrap().trim().split("\n").fold(0, |acc, s| {
        let (x, y) = s.trim().split_once(",").unwrap();
        let (x, y) = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
        let (dx, dy) = (x - pos.0, y - pos.1);
        // println!("({},{}) -> ({},{})", pos.0, pos.1, x, y);
        // println!("dx: {}, dy: {}", dx, dy);
        pos = (pos.0 + dx, pos.1 + dy);
        let max = dx.abs().max(dy.abs());
        let min = dx.abs().min(dy.abs());
        acc + min + (max - min)
    });
    println!("part2: {part2:?}");

    let mut pos = (0, 0);
    let cost = fs::read_to_string("input2.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|s| {
            let (x, y) = s.trim().split_once(",").unwrap();
            let (x, y) = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
            (x + y, (x, y))
        })
        .sorted()
        .fold(0, |acc, (_, (x, y))| {
            let (dx, dy) = (x - pos.0, y - pos.1);
            // println!("({},{}) -> ({},{})", pos.0, pos.1, x, y);
            // println!("dx: {}, dy: {}", dx, dy);
            pos = (pos.0 + dx, pos.1 + dy);
            let max = dx.abs().max(dy.abs());
            let min = dx.abs().min(dy.abs());
            acc + min + (max - min)
        });
    println!("part3: {:?}", cost);
}
