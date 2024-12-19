use std::collections::HashMap;
use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day19.txt")?;
    let (input_patterns, input_designs) = input.split_once("\n\n").unwrap();

    let patterns = input_patterns.split(", ").collect_vec();
    let designs = input_designs.lines().collect_vec();

    let mut cache = HashMap::new();
    let possible_towel_designs = designs
        .iter()
        .filter(|design| towel_designs(&patterns, design, &mut cache) > 0)
        .count();

    println!("solution part 1: {possible_towel_designs}");
    Ok(())
}

fn towel_designs<'a>(
    patterns: &Vec<&str>,
    design: &'a str,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if design.is_empty() {
        return 1;
    }
    if let Some(count) = cache.get(design) {
        return *count;
    }
    let mut count = 0;
    for p in patterns {
        if design.starts_with(p) {
            count += towel_designs(&patterns, &design[p.len()..], cache)
        }
    }
    cache.insert(design, count);
    count
}
