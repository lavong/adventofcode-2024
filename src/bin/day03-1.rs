use std::fs;
use std::io;

use regex::Regex;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day03.txt")?;
    let regex = Regex::new(r"(mul\(\d+,\d+\))").unwrap();

    let mut sum = 0;
    for m in regex.find_iter(&input) {
        let p = m.as_str().replace("mul(", "").replace(")", "");
        let (n, m) = p.split_once(',').unwrap();
        sum += n.parse::<u32>().unwrap() * m.parse::<u32>().unwrap()
    }

    println!("solution part 1: {sum}");
    Ok(())
}
