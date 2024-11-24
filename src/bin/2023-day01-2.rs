use std::{collections::HashMap, fs, io};

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/2023-day01.txt")?;

    let spelled_digits_mapping: HashMap<String, u32> =
        "one,two,three,four,five,six,seven,eight,nine"
            .split(',')
            .map(|s| s.to_string())
            .zip(1..=9)
            .collect();

    let digits_mapping: HashMap<String, u32> = (0..=9)
        .map(|i| (i.to_string(), i))
        .chain(spelled_digits_mapping)
        .collect();

    let sum = input
        .lines()
        .map(|line| parse_digits(line, &digits_mapping))
        .map(|n| n.first().unwrap() * 10 + n.last().unwrap())
        .sum::<u32>();

    println!("solution part 2: {sum}");
    Ok(())
}

fn parse_digits(line: &str, digits_mapping: &HashMap<String, u32>) -> Vec<u32> {
    (0..=line.len())
        .filter_map(|idx| {
            digits_mapping.iter().find_map(|(digit_string, digit)| {
                line[idx..].starts_with(digit_string).then_some(*digit)
            })
        })
        .collect_vec()
}
