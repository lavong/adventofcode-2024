use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day01.txt")?;

    let (left, right): (Vec<usize>, Vec<usize>) = input
        .lines()
        .filter_map(|line| line.split_whitespace().next_tuple())
        .map(|(n, m)| (n.parse::<usize>().unwrap(), m.parse::<usize>().unwrap()))
        .unzip();

    let similarity_score: usize = left
        .iter()
        .map(|n| n * right.iter().filter(|m| *m == n).count())
        .sum();

    println!("solution part 2: {similarity_score}");
    Ok(())
}
