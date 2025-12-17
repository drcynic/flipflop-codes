use std::fs;

use num::Signed;

fn main() {
    let chars = fs::read_to_string("input2.txt").unwrap().trim().chars().collect::<Vec<_>>();
    let part1 = chars
        .iter()
        .fold((0, 0), |acc, &c| {
            let new = acc.0 + if c == '^' { 1 } else { -1 };
            (new, acc.1.max(new))
        })
        .1;
    println!("part1: {part1}");

    let part2 = chars
        .iter()
        .fold((0, 0, 0, false), |acc, &c| {
            let (step, up) = if c == '^' {
                if acc.3 { (acc.2 + 1, true) } else { (1, true) }
            } else {
                if !acc.3 { (acc.2 - 1, false) } else { (-1, false) }
            };
            let new = acc.0 + step;
            (new, acc.1.max(new), step, up)
        })
        .1;
    println!("part2: {part2}");

    fn fib(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            _ => fib(n - 1) + fib(n - 2),
        }
    }
    let (height, max, step, up) = chars.iter().fold((0, 0, 0, false), |acc, &c| {
        let (step, up) = if c == '^' {
            if acc.3 { (acc.2 + 1, true) } else { (1, true) }
        } else {
            if !acc.3 { (acc.2 - 1, false) } else { (-1, false) }
        };
        let new = if acc.3 != up {
            acc.0 + fib(acc.2.abs()) * if acc.3 { 1 } else { -1 }
        } else {
            acc.0
        };
        (new, acc.1.max(new), step, up)
    });
    let last = fib(step.abs()) * if up { 1 } else { -1 };
    let part3 = max.max(height + last);
    println!("part3: {part3}");
}
