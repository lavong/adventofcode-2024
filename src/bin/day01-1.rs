use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day01.txt")?;

    let mut location_ids_left = Vec::new();
    let mut location_ids_right = Vec::new();
    input.lines().for_each(|line| {
        let (n, m) = line.split_whitespace().next_tuple().unwrap();
        location_ids_left.push(n.parse::<u32>().unwrap());
        location_ids_right.push(m.parse::<u32>().unwrap());
    });

    location_ids_left.sort();
    location_ids_right.sort();
    let total_distance: u32 = location_ids_left
        .iter()
        .zip(location_ids_right)
        .map(|(n, m)| n.abs_diff(m))
        .sum();

    println!("solution part 1: {total_distance}");
    Ok(())
}
