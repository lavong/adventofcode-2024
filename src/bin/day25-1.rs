use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day25.txt")?;

    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for kl in input.split("\n\n") {
        let keylock = kl.lines().map(|l| l.chars().collect_vec()).collect_vec();
        if keylock[0][0] == '#' {
            locks.push(keylock);
        } else {
            keys.push(keylock);
        }
    }

    let mut fitting_keylock_pairs = 0;
    for key in &keys {
        for lock in &locks {
            let mut fits = true;

            for y in 0..key.len() {
                for x in 0..key[y].len() {
                    if key[y][x] == '#' && lock[y][x] == '#' {
                        fits = false;
                    }
                }
            }

            if fits {
                fitting_keylock_pairs += 1;
            }
        }
    }

    println!("solution part 1: {fitting_keylock_pairs}");
    Ok(())
}
