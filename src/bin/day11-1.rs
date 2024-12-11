use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day11.txt")?;

    let mut stones = input
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect_vec();

    (0..25).for_each(|_| {
        let mut s: Vec<u64> = vec![];
        for i in 0..stones.len() {
            let n = stones[i];
            let ns = n.to_string();
            if n == 0 {
                s.push(1);
            } else if ns.len() % 2 == 0 {
                let (x, y) = ns.split_at(ns.len() / 2);
                s.push(x.parse::<u64>().unwrap());
                s.push(y.parse::<u64>().unwrap());
            } else {
                s.push(n * 2024);
            }
        }
        stones = s;
    });

    let stone_count = stones.len();

    println!("solution part 1: {stone_count}");
    Ok(())
}
