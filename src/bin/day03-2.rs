use std::fs;
use std::io;

use regex::Regex;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day03.txt")?;
    let regex = Regex::new(r"(mul\(\d+,\d+\)|do(n't)?\(\))").unwrap();

    let mut do_next = true;
    let mut sum = 0;
    for m in regex.find_iter(&input) {
        match m.as_str() {
            "do()" => do_next = true,
            "don't()" => do_next = false,
            mul if do_next => {
                let p = mul.replace("mul(", "").replace(")", "");
                let (n, m) = p.split_once(',').unwrap();
                sum += n.parse::<u32>().unwrap() * m.parse::<u32>().unwrap()
            }
            _ => {}
        }
    }

    println!("solution part 2: {sum}");
    Ok(())
}
