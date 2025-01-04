use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day22.txt")?;
    let secret_numbers = input.lines().map(|l| l.parse::<u64>().unwrap());

    let mut bananas: HashMap<(i64, i64, i64, i64), u64> = HashMap::new();
    for n in secret_numbers {
        let mut prices = [0; 2000];
        let mut s = n;
        for i in 0..2000 {
            s = (s ^ (s * 64)) % 16777216;
            s = (s ^ (s / 32)) % 16777216;
            s = (s ^ (s * 2048)) % 16777216;
            prices[i] = s % 10
        }

        let mut cache = HashSet::new();
        for (p1, p2, p3, p4, p5) in prices.iter().tuple_windows() {
            let diff_pattern = [p1, p2, p3, p4, p5]
                .iter()
                .tuple_windows()
                .map(|(n, m)| **m as i64 - **n as i64)
                .collect_tuple()
                .unwrap();
            if cache.insert(diff_pattern) {
                *bananas.entry(diff_pattern).or_default() += p5;
            }
        }
    }
    let most_bananas_to_get = bananas.values().max().unwrap();

    println!("solution part 2: {most_bananas_to_get}");
    Ok(())
}
