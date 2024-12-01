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

    let total_distance: usize = (left.iter().sorted())
        .zip(right.iter().sorted())
        .map(|(n, m)| n.abs_diff(*m))
        .sum();

    println!("solution part 1: {total_distance}");
    Ok(())
}
