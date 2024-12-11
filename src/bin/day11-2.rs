use std::collections::HashMap;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day11.txt")?;

    let mut stones = input
        .split_whitespace()
        .map(|n| (n.parse::<u64>().unwrap(), 1))
        .collect::<HashMap<u64, u64>>();

    (0..75).for_each(|_| {
        let mut s: HashMap<u64, u64> = HashMap::new();
        for (&n, &cache) in &stones {
            if n == 0 {
                *s.entry(1).or_default() += cache;
            } else {
                let digits = n.ilog10() + 1;
                if digits % 2 == 0 {
                    *s.entry(n % 10u64.pow(digits / 2)).or_default() += cache;
                    *s.entry(n / 10u64.pow(digits / 2)).or_default() += cache;
                } else {
                    *s.entry(n * 2024).or_default() += cache;
                }
            }
        }
        stones = s;
    });

    let stone_count: u64 = stones.values().sum();

    println!("solution part 2: {stone_count}");
    Ok(())
}
