use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day14.txt")?;

    const Y: i32 = 101;
    const X: i32 = 103;
    let mut robots: Vec<(i32, i32, i32, i32)> = input
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter_map(|w| w.parse::<i32>().ok())
        .tuples()
        .collect_vec();

    for _ in 1..=100 {
        for (y, x, dy, dx) in &mut robots {
            *y = (*y + *dy).rem_euclid(Y);
            *x = (*x + *dx).rem_euclid(X);
        }
    }

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for &(x, y, _, _) in &robots {
        if x < Y / 2 && y < X / 2 {
            q1 += 1;
        } else if x > Y / 2 && y < X / 2 {
            q2 += 1;
        } else if x < Y / 2 && y > X / 2 {
            q3 += 1;
        } else if x > Y / 2 && y > X / 2 {
            q4 += 1;
        }
    }
    let safety_factor = q1 * q2 * q3 * q4;

    println!("solution part 1: {safety_factor}");
    Ok(())
}
