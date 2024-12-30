use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day22.txt")?;
    let secret_numbers = input.lines().map(|l| l.parse::<u64>().unwrap());

    let mut sum = 0;
    for n in secret_numbers {
        let mut s = n;
        for _ in 0..2000 {
            s = (s ^ (s * 64)) % 16777216;
            s = (s ^ (s / 32)) % 16777216;
            s = (s ^ (s * 2048)) % 16777216;
        }
        sum += s;
    }

    println!("solution part 1: {sum}");
    Ok(())
}
