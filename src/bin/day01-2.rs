use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day01.txt")?;

    let mut location_ids_left = Vec::new();
    let mut location_ids_right = Vec::new();
    input.lines().for_each(|line| {
        let (n, m) = line.split_whitespace().next_tuple().unwrap();
        location_ids_left.push(n.parse::<usize>().unwrap());
        location_ids_right.push(m.parse::<usize>().unwrap());
    });

    let similarity_score: usize = location_ids_left
        .iter()
        .map(|n| n * location_ids_right.iter().filter(|m| *m == n).count())
        .sum();

    println!("solution part 2: {similarity_score}");
    Ok(())
}
