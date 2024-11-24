use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/2023-day01.txt")?;

    let sum = input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect_vec())
        .map(|n| n.first().unwrap() * 10 + n.last().unwrap())
        .sum::<u32>();

    println!("solution part 1: {sum}");
    Ok(())
}
