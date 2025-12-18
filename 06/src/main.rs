use std::fs;

fn main() {
    let mut birds = fs::read_to_string("input2.txt")
        .unwrap()
        .trim()
        .lines()
        .fold(Vec::new(), |mut acc, l| {
            let (l, r) = l.split_once(',').unwrap();
            acc.push((0, 0, l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()));
            acc
        });
    let width = 1000;
    for _r in 0..100 {
        for b in &mut birds {
            b.0 = (b.0 + b.2).rem_euclid(width);
            b.1 = (b.1 + b.3).rem_euclid(width);
        }
    }
    let xs = width / 4;
    let xe = xs + width / 2 - 1;
    let p1 = birds.iter().filter(|b| b.0 >= xs && b.0 <= xe && b.1 >= xs && b.1 <= xe).count();
    println!("part1: {p1:?}");

    let mut birds = fs::read_to_string("input2.txt")
        .unwrap()
        .trim()
        .lines()
        .fold(Vec::new(), |mut acc, l| {
            let (l, r) = l.split_once(',').unwrap();
            acc.push((0, 0, l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()));
            acc
        });
    let width = 1000;
    let mut sum = 0;
    for _ in 0..1000 {
        for _r in 0..3600 {
            for b in &mut birds {
                b.0 = (b.0 + b.2).rem_euclid(width);
                b.1 = (b.1 + b.3).rem_euclid(width);
            }
        }
        let xs = width / 4;
        let xe = xs + width / 2 - 1;
        sum += birds.iter().filter(|b| b.0 >= xs && b.0 <= xe && b.1 >= xs && b.1 <= xe).count();
    }
    println!("part2: {sum:?}");

    let birds_velocities = fs::read_to_string("input2.txt")
        .unwrap()
        .trim()
        .lines()
        .fold(Vec::new(), |mut acc, l| {
            let (l, r) = l.split_once(',').unwrap();
            acc.push((l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()));
            acc
        });
    let width = 1000;
    let mut sum = 0;
    let mut birds = vec![(0, 0); birds_velocities.len()];
    let mut rem = 0;
    for _s in 1..=1000000 {
        for i in 0..birds_velocities.len() {
            let bv = birds_velocities[i];
            let b = &mut birds[i];
            b.0 = (b.0 + bv.0).rem_euclid(width);
            b.1 = (b.1 + bv.1).rem_euclid(width);
        }
        if birds.iter().all(|&b| b == (0, 0)) {
            let year_in_s = 31556926;
            rem = year_in_s % 1000;
            break;
        }
    }
    for _year in 0..1000 {
        for _rem_s in 0..rem {
            for i in 0..birds_velocities.len() {
                let bv = birds_velocities[i];
                let b = &mut birds[i];
                b.0 = (b.0 + bv.0).rem_euclid(width);
                b.1 = (b.1 + bv.1).rem_euclid(width);
            }
        }

        let xs = width / 4;
        let xe = xs + width / 2 - 1;
        sum += birds.iter().filter(|b| b.0 >= xs && b.0 <= xe && b.1 >= xs && b.1 <= xe).count();
    }
    println!("part3: {sum:?}");
}
