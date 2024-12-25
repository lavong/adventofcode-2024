use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day25.txt")?;

    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for kl in input.split("\n\n") {
        let keylock = kl.lines().map(|l| l.chars().collect_vec()).collect_vec();
        let mut heights = vec![0; keylock.len()];
        for y in 0..keylock.len() {
            for x in 0..keylock[y].len() {
                if keylock[y][x] == '#' {
                    heights[x] += 1;
                }
            }
        }
        if keylock[0][0] == '#' {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    let mut fitting_keylock_pairs = 0;
    for key in &keys {
        for lock in &locks {
            if key.iter().zip(lock.iter()).all(|(k, l)| k + l < 8) {
                fitting_keylock_pairs += 1;
            }
        }
    }

    println!("solution part 1: {fitting_keylock_pairs}");
    Ok(())
}
