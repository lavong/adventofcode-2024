use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day02.txt")?;

    let safe_reactors = input.lines().filter(|l| is_safe(l)).count();

    println!("solution part 1: {safe_reactors}");
    Ok(())
}

fn is_safe(line: &str) -> bool {
    let diffs = line
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .tuple_windows::<(i32, i32)>()
        .map(|(n, m)| m - n)
        .collect_vec();
    let all_negative = diffs.iter().all(|n| n < &0);
    let all_positive = diffs.iter().all(|n| n > &0);

    diffs.iter().fold(all_negative || all_positive, |acc, n| {
        acc && (1..=3).contains(&n.abs())
    })
}
